use neo4rs::Graph;
use crate::models::workspace::Workspace;
use crate::errors::AppError;

fn is_session_token_error(err: &neo4rs::Error) -> bool {
    let msg = err.to_string();
    msg.contains("invalid session token")
        || (msg.contains("session") && msg.contains("token"))
}

#[derive(Clone)]
pub struct Neo4jRepo {
    graph: Graph,
}

const DEFAULT_PAPERS_CAPACITY: usize = 64;
const DEFAULT_AUTHORS_CAPACITY: usize = 32;
const DEFAULT_KEYWORDS_CAPACITY: usize = 16;
const DEFAULT_WORKSPACES_CAPACITY: usize = 32;
const DEFAULT_GRAPH_NODES_CAPACITY: usize = 128;
const DEFAULT_GRAPH_LINKS_CAPACITY: usize = 256;

macro_rules! run_query {
    ($self:expr, $query:expr) => {{
        let query = $query;
        let mut attempts = 0u32;
        loop {
            match $self.graph.execute(query.clone()).await {
                Ok(stream) => break stream,
                Err(e) if is_session_token_error(&e) && attempts < 3 => {
                    attempts += 1;
                    tracing::warn!("Invalid session token, retrying (attempt {}/3)", attempts);
                    tokio::time::sleep(std::time::Duration::from_millis(200 * attempts as u64)).await;
                    continue;
                }
                Err(e) => return Err(AppError::from(e)),
            }
        }
    }};
}

impl Neo4jRepo {
    pub fn new(graph: Graph) -> Self {
        Self { graph }
    }

    pub async fn create_indexes(&self) -> Result<(), AppError> {
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS FOR (w:Workspace) ON (w.id)",
            "CREATE INDEX IF NOT EXISTS FOR (p:Paper) ON (p.id)",
            "CREATE INDEX IF NOT EXISTS FOR (p:Paper) ON (p.title)",
            "CREATE INDEX IF NOT EXISTS FOR (p:Paper) ON (p.abstract)",
            "CREATE INDEX IF NOT EXISTS FOR (a:Author) ON (a.id)",
            "CREATE INDEX IF NOT EXISTS FOR (a:Author) ON (a.name)",
            "CREATE INDEX IF NOT EXISTS FOR (k:Keyword) ON (k.name)",
            "CREATE INDEX IF NOT EXISTS FOR ()-[r:CONTAINS]->() ON (r.workspace_id)",
            "CREATE INDEX IF NOT EXISTS FOR ()-[r:FIRST_AUTHOR_OF]->() ON (r.paper_id)",
            "CREATE INDEX IF NOT EXISTS FOR ()-[r:CORRESPONDING_AUTHOR_OF]->() ON (r.paper_id)",
        ];

        let mut tasks = Vec::with_capacity(indexes.len());
        for idx in indexes {
            let graph = self.graph.clone();
            tasks.push(tokio::spawn(async move {
                let query = neo4rs::query(idx);
                let mut result = graph.execute(query).await?;
                let _ = result.next().await;
                Ok::<(), AppError>(())
            }));
        }

        for task in tasks {
            let result = task.await.map_err(|e| AppError::Neo4jError(format!("Task join error: {}", e)))?;
            result?;
        }

        tracing::info!("All indexes created successfully");
        Ok(())
    }

    pub async fn create_workspace(&self, id: &str, name: &str, description: &str, created_at: &str) -> Result<Workspace, AppError> {
        let query = neo4rs::query(
            "CREATE (w:Workspace {id: $id, name: $name, description: $description, created_at: $created_at}) RETURN w"
        )
        .param("id", id)
        .param("name", name)
        .param("description", description)
        .param("created_at", created_at);

        let mut result = run_query!(self, query);
        let row = result.next().await?.ok_or_else(|| AppError::Neo4jError("No row returned".into()))?;
        let node: neo4rs::Node = row.get("w")?;
        Ok(workspace_from_node(&node))
    }

    pub async fn list_workspaces(&self) -> Result<Vec<Workspace>, AppError> {
        let query = neo4rs::query("MATCH (w:Workspace) RETURN w ORDER BY w.created_at DESC");
        let mut result = run_query!(self, query);
        let mut workspaces = Vec::with_capacity(DEFAULT_WORKSPACES_CAPACITY);
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("w")?;
            workspaces.push(workspace_from_node(&node));
        }
        Ok(workspaces)
    }

    pub async fn get_workspace(&self, id: &str) -> Result<Option<Workspace>, AppError> {
        let query = neo4rs::query("MATCH (w:Workspace {id: $id}) RETURN w").param("id", id);
        let mut result = run_query!(self, query);
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("w")?;
            Ok(Some(workspace_from_node(&node)))
        } else {
            Ok(None)
        }
    }

    pub async fn update_workspace(&self, id: &str, name: Option<&str>, description: Option<&str>) -> Result<Option<Workspace>, AppError> {
        let query = neo4rs::query(
            "MATCH (w:Workspace {id: $id}) SET w.name = COALESCE($name, w.name), w.description = COALESCE($description, w.description) RETURN w"
        )
        .param("id", id)
        .param("name", name)
        .param("description", description);

        let mut result = run_query!(self, query);
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("w")?;
            Ok(Some(workspace_from_node(&node)))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_workspace(&self, id: &str) -> Result<bool, AppError> {
        let query = neo4rs::query(
            "MATCH (w:Workspace {id: $id}) DETACH DELETE w RETURN count(w) AS deleted"
        )
        .param("id", id);

        let mut result = run_query!(self, query);
        if let Some(row) = result.next().await? {
            let deleted: i64 = row.get("deleted")?;
            Ok(deleted > 0)
        } else {
            Ok(false)
        }
    }

    pub async fn create_paper_if_not_exists(
        &self,
        id: &str,
        title: &str,
        doi: Option<&str>,
        arxiv_id: Option<&str>,
        abstract_text: Option<&str>,
        year: Option<i32>,
        journal: Option<&str>,
        created_at: &str,
    ) -> Result<crate::models::paper::Paper, AppError> {
        let query = neo4rs::query(
            "MERGE (p:Paper {doi: COALESCE($doi, ''), arxiv_id: COALESCE($arxiv_id, '')}) \
             ON CREATE SET p.id = $id, p.title = $title, p.abstract = $abstract_text, \
             p.year = $year, p.journal = $journal, p.created_at = $created_at, p.user_notes = '' \
             RETURN p"
        )
        .param("id", id)
        .param("title", title)
        .param("doi", doi.unwrap_or(""))
        .param("arxiv_id", arxiv_id.unwrap_or(""))
        .param("abstract_text", abstract_text.unwrap_or(""))
        .param("year", year.unwrap_or(0))
        .param("journal", journal.unwrap_or(""))
        .param("created_at", created_at);

        let mut result = run_query!(self, query);
        let row = result.next().await?.ok_or_else(|| AppError::Neo4jError("No row returned".into()))?;
        let node: neo4rs::Node = row.get("p")?;
        Ok(paper_from_node(&node))
    }

    pub async fn add_paper_to_workspace(&self, workspace_id: &str, paper_id: &str, added_at: &str) -> Result<(), AppError> {
        let query = neo4rs::query(
            "MATCH (w:Workspace {id: $workspace_id}), (p:Paper {id: $paper_id}) \
             MERGE (w)-[:CONTAINS {added_at: $added_at}]->(p)"
        )
        .param("workspace_id", workspace_id)
        .param("paper_id", paper_id)
        .param("added_at", added_at);
        let mut result = run_query!(self, query);
        let _ = result.next().await;
        Ok(())
    }

    pub async fn create_author_if_not_exists(
        &self,
        id: &str,
        name: &str,
        orcid: Option<&str>,
    ) -> Result<crate::models::author::Author, AppError> {
        let query = neo4rs::query(
            "MERGE (a:Author {name: $name, orcid: COALESCE($orcid, '')}) \
             ON CREATE SET a.id = $id \
             RETURN a"
        )
        .param("id", id)
        .param("name", name)
        .param("orcid", orcid.unwrap_or(""));

        let mut result = run_query!(self, query);
        let row = result.next().await?.ok_or_else(|| AppError::Neo4jError("No row returned".into()))?;
        let node: neo4rs::Node = row.get("a")?;
        Ok(author_from_node(&node))
    }

    pub async fn link_first_author(&self, author_id: &str, paper_id: &str) -> Result<(), AppError> {
        let query = neo4rs::query(
            "MATCH (a:Author {id: $author_id}), (p:Paper {id: $paper_id}) \
             MERGE (a)-[:FIRST_AUTHOR_OF]->(p)"
        )
        .param("author_id", author_id)
        .param("paper_id", paper_id);
        let mut result = run_query!(self, query);
        let _ = result.next().await;
        Ok(())
    }

    pub async fn link_corresponding_author(&self, author_id: &str, paper_id: &str) -> Result<(), AppError> {
        let query = neo4rs::query(
            "MATCH (a:Author {id: $author_id}), (p:Paper {id: $paper_id}) \
             MERGE (a)-[:CORRESPONDING_AUTHOR_OF]->(p)"
        )
        .param("author_id", author_id)
        .param("paper_id", paper_id);
        let mut result = run_query!(self, query);
        let _ = result.next().await;
        Ok(())
    }

    pub async fn link_co_authors(&self, author1_id: &str, author2_id: &str, workspace_id: &str) -> Result<(), AppError> {
        let query = neo4rs::query(
            "MATCH (a1:Author {id: $author1_id}), (a2:Author {id: $author2_id}) \
             MERGE (a1)-[r:CO_AUTHOR_OF {workspace_id: $workspace_id}]-(a2) \
             ON CREATE SET r.paper_count = 1 \
             ON MATCH SET r.paper_count = r.paper_count + 1"
        )
        .param("author1_id", author1_id)
        .param("author2_id", author2_id)
        .param("workspace_id", workspace_id);
        let mut result = run_query!(self, query);
        let _ = result.next().await;
        Ok(())
    }

    pub async fn add_keyword(&self, id: &str, name: &str, paper_id: &str) -> Result<(), AppError> {
        let query = neo4rs::query(
            "MERGE (k:Keyword {name: $name}) \
             ON CREATE SET k.id = $id \
             WITH k MATCH (p:Paper {id: $paper_id}) \
             MERGE (p)-[:HAS_KEYWORD]->(k)"
        )
        .param("id", id)
        .param("name", name)
        .param("paper_id", paper_id);
        let mut result = run_query!(self, query);
        let _ = result.next().await;
        Ok(())
    }

    pub async fn create_authors_batch(
        &self,
        authors: &[(String, String, Option<String>, bool, bool)],
        paper_id: &str,
        workspace_id: &str,
    ) -> Result<(Option<crate::models::author::Author>, Option<crate::models::author::Author>), AppError> {
        let n = authors.len();
        let mut ids = Vec::with_capacity(n);
        let mut names = Vec::with_capacity(n);
        let mut orcids = Vec::with_capacity(n);
        let mut is_first = Vec::with_capacity(n);
        let mut is_corresponding = Vec::with_capacity(n);
        let mut first_id = String::new();
        let mut corresponding_id = String::new();

        for a in authors {
            ids.push(a.0.as_str());
            names.push(a.1.as_str());
            orcids.push(a.2.as_deref().unwrap_or(""));
            is_first.push(a.3);
            is_corresponding.push(a.4);
            if a.3 && first_id.is_empty() {
                first_id = a.0.clone();
            }
            if a.4 && corresponding_id.is_empty() {
                corresponding_id = a.0.clone();
            }
        }

        let cypher = "UNWIND range(0, size($ids)-1) AS idx
                      MERGE (a:Author {name: $names[idx], orcid: COALESCE($orcids[idx], '')})
                      ON CREATE SET a.id = $ids[idx]
                      WITH a, $is_first[idx] AS is_first, $is_corresponding[idx] AS is_corresponding, $paper_id AS pid
                      FOREACH (_ IN CASE WHEN is_first THEN [1] ELSE [] END |
                        MERGE (a)-[:FIRST_AUTHOR_OF]->(:Paper {id: pid})
                      )
                      FOREACH (_ IN CASE WHEN is_corresponding THEN [1] ELSE [] END |
                        MERGE (a)-[:CORRESPONDING_AUTHOR_OF]->(:Paper {id: pid})
                      )
                      WITH COLLECT({id: a.id, name: a.name, orcid: a.orcid, is_first: is_first, is_corresponding: is_corresponding}) AS authors_list,
                           $first_id AS first_id, $corresponding_id AS corresponding_id, $workspace_id AS ws_id
                      FOREACH (_ IN CASE WHEN first_id <> '' AND corresponding_id <> '' AND first_id <> corresponding_id THEN [1] ELSE [] END |
                        MATCH (a1:Author {id: first_id}), (a2:Author {id: corresponding_id})
                        MERGE (a1)-[r:CO_AUTHOR_OF {workspace_id: ws_id}]-(a2)
                        ON CREATE SET r.paper_count = 1
                        ON MATCH SET r.paper_count = r.paper_count + 1
                      )
                      UNWIND authors_list AS author
                      RETURN author.id AS id, author.name AS name, author.orcid AS orcid, author.is_first AS is_first, author.is_corresponding AS is_corresponding";

        let query = neo4rs::query(cypher)
            .param("ids", ids.as_slice())
            .param("names", names.as_slice())
            .param("orcids", orcids.as_slice())
            .param("is_first", is_first.as_slice())
            .param("is_corresponding", is_corresponding.as_slice())
            .param("paper_id", paper_id)
            .param("first_id", &*first_id)
            .param("corresponding_id", &*corresponding_id)
            .param("workspace_id", workspace_id);

        let mut result = run_query!(self, query);
        
        let mut first_author: Option<crate::models::author::Author> = None;
        let mut corresponding_author: Option<crate::models::author::Author> = None;

        while let Some(row) = result.next().await? {
            let id: String = row.get("id")?;
            let name: String = row.get("name")?;
            let orcid: Option<String> = row.get("orcid").ok().filter(|s: &String| !s.is_empty());
            let is_first_flag: bool = row.get("is_first")?;
            let is_corresponding_flag: bool = row.get("is_corresponding")?;

            if is_first_flag && first_author.is_none() {
                first_author = Some(crate::models::author::Author { id: id.clone(), name: name.clone(), orcid: orcid.clone() });
            }
            if is_corresponding_flag && corresponding_author.is_none() {
                corresponding_author = Some(crate::models::author::Author { id: id.clone(), name: name.clone(), orcid });
            }
        }

        Ok((first_author, corresponding_author))
    }

    pub async fn add_keywords_batch(&self, keywords: &[(String, String)], paper_id: &str) -> Result<(), AppError> {
        let n = keywords.len();
        let mut ids = Vec::with_capacity(n);
        let mut names = Vec::with_capacity(n);

        for k in keywords {
            ids.push(k.0.as_str());
            names.push(k.1.as_str());
        }

        let cypher = "UNWIND range(0, size($ids)-1) AS idx
                      MERGE (k:Keyword {name: $names[idx]})
                      ON CREATE SET k.id = $ids[idx]
                      WITH k, $paper_id AS pid
                      MATCH (p:Paper {id: pid})
                      MERGE (p)-[:HAS_KEYWORD]->(k)";

        let query = neo4rs::query(cypher)
            .param("ids", ids.as_slice())
            .param("names", names.as_slice())
            .param("paper_id", paper_id);

        let mut result = run_query!(self, query);
        let _ = result.next().await;
        Ok(())
    }

    pub async fn list_papers_in_workspace(&self, workspace_id: &str) -> Result<Vec<crate::models::paper::Paper>, AppError> {
        let query = neo4rs::query(
            "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper) RETURN p ORDER BY p.year DESC"
        )
        .param("workspace_id", workspace_id);

        let mut result = run_query!(self, query);
        let mut papers = Vec::with_capacity(DEFAULT_PAPERS_CAPACITY);
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("p")?;
            papers.push(paper_from_node(&node));
        }
        Ok(papers)
    }

    pub async fn get_paper(&self, id: &str) -> Result<Option<crate::models::paper::Paper>, AppError> {
        let query = neo4rs::query("MATCH (p:Paper {id: $id}) RETURN p").param("id", id);
        let mut result = run_query!(self, query);
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("p")?;
            Ok(Some(paper_from_node(&node)))
        } else {
            Ok(None)
        }
    }

    pub async fn update_paper_notes(&self, id: &str, user_notes: &str) -> Result<Option<crate::models::paper::Paper>, AppError> {
        let query = neo4rs::query(
            "MATCH (p:Paper {id: $id}) SET p.user_notes = $user_notes RETURN p"
        )
        .param("id", id)
        .param("user_notes", user_notes);

        let mut result = run_query!(self, query);
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("p")?;
            Ok(Some(paper_from_node(&node)))
        } else {
            Ok(None)
        }
    }

    pub async fn remove_paper_from_workspace(&self, workspace_id: &str, paper_id: &str) -> Result<bool, AppError> {
        let query = neo4rs::query(
            "MATCH (w:Workspace {id: $workspace_id})-[r:CONTAINS]->(p:Paper {id: $paper_id}) DELETE r RETURN count(r) AS deleted"
        )
        .param("workspace_id", workspace_id)
        .param("paper_id", paper_id);

        let mut result = run_query!(self, query);
        if let Some(row) = result.next().await? {
            let deleted: i64 = row.get("deleted")?;
            Ok(deleted > 0)
        } else {
            Ok(false)
        }
    }

    pub async fn get_paper_first_author(&self, paper_id: &str) -> Result<Option<crate::models::author::Author>, AppError> {
        let query = neo4rs::query(
            "MATCH (a:Author)-[:FIRST_AUTHOR_OF]->(p:Paper {id: $paper_id}) RETURN a"
        )
        .param("paper_id", paper_id);

        let mut result = run_query!(self, query);
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("a")?;
            Ok(Some(author_from_node(&node)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_paper_corresponding_author(&self, paper_id: &str) -> Result<Option<crate::models::author::Author>, AppError> {
        let query = neo4rs::query(
            "MATCH (a:Author)-[:CORRESPONDING_AUTHOR_OF]->(p:Paper {id: $paper_id}) RETURN a"
        )
        .param("paper_id", paper_id);

        let mut result = run_query!(self, query);
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("a")?;
            Ok(Some(author_from_node(&node)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_paper_keywords(&self, paper_id: &str) -> Result<Vec<crate::models::keyword::Keyword>, AppError> {
        let query = neo4rs::query(
            "MATCH (p:Paper {id: $paper_id})-[:HAS_KEYWORD]->(k:Keyword) RETURN k"
        )
        .param("paper_id", paper_id);

        let mut result = run_query!(self, query);
        let mut keywords = Vec::with_capacity(DEFAULT_KEYWORDS_CAPACITY);
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("k")?;
            keywords.push(keyword_from_node(&node));
        }
        Ok(keywords)
    }

    pub async fn get_paper_detail(&self, paper_id: &str) -> Result<Option<(
        crate::models::paper::Paper,
        Option<crate::models::author::Author>,
        Option<crate::models::author::Author>,
        Vec<crate::models::keyword::Keyword>,
    )>, AppError> {
        let cypher = "MATCH (p:Paper {id: $paper_id})
                      OPTIONAL MATCH (fa:Author)-[:FIRST_AUTHOR_OF]->(p)
                      OPTIONAL MATCH (ca:Author)-[:CORRESPONDING_AUTHOR_OF]->(p)
                      OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)
                      WITH p, 
                           head(collect(DISTINCT fa)) AS fa, 
                           head(collect(DISTINCT ca)) AS ca,
                           collect(DISTINCT k) AS keywords
                      RETURN p, fa, ca, keywords";
        let query = neo4rs::query(cypher)
            .param("paper_id", paper_id);

        let mut result = run_query!(self, query);
        if let Some(row) = result.next().await? {
            let paper_node: neo4rs::Node = row.get("p")?;
            let first_author: Option<neo4rs::Node> = row.get("fa").ok();
            let corresponding_author: Option<neo4rs::Node> = row.get("ca").ok();
            let keyword_nodes: Vec<neo4rs::Node> = row.get("keywords").unwrap_or_default();

            let first_author = first_author.map(|n| author_from_node(&n));
            let corresponding_author = corresponding_author.map(|n| author_from_node(&n));
            let keywords: Vec<_> = keyword_nodes.iter().map(keyword_from_node).collect();

            Ok(Some((paper_from_node(&paper_node), first_author, corresponding_author, keywords)))
        } else {
            Ok(None)
        }
    }

    pub async fn list_authors_in_workspace(&self, workspace_id: &str) -> Result<Vec<crate::models::author::Author>, AppError> {
        let query = neo4rs::query(
            "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)<-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]-(a:Author) \
             RETURN DISTINCT a ORDER BY a.name"
        )
        .param("workspace_id", workspace_id);

        let mut result = run_query!(self, query);
        let mut authors = Vec::with_capacity(DEFAULT_AUTHORS_CAPACITY);
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("a")?;
            authors.push(author_from_node(&node));
        }
        Ok(authors)
    }

    pub async fn get_author_papers(&self, author_id: &str) -> Result<Vec<crate::models::paper::Paper>, AppError> {
        let query = neo4rs::query(
            "MATCH (a:Author {id: $author_id})-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p:Paper) RETURN p ORDER BY p.year DESC"
        )
        .param("author_id", author_id);

        let mut result = run_query!(self, query);
        let mut papers = Vec::with_capacity(DEFAULT_PAPERS_CAPACITY);
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("p")?;
            papers.push(paper_from_node(&node));
        }
        Ok(papers)
    }

    pub async fn get_graph_data(&self, workspace_id: &str) -> Result<(Vec<crate::models::dto::GraphNode>, Vec<crate::models::dto::GraphLink>), AppError> {
        let node_cypher = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)<-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]-(a:Author)
                          WITH a, count(DISTINCT p) AS paper_count,
                               exists((a)-[:FIRST_AUTHOR_OF]->(:Paper)<-[:CONTAINS]-(w)) AS has_first,
                               exists((a)-[:CORRESPONDING_AUTHOR_OF]->(:Paper)<-[:CONTAINS]-(w)) AS has_corresponding
                          RETURN a.id AS id, a.name AS name, paper_count,
                                 CASE WHEN has_first AND has_corresponding THEN 'both' WHEN has_first THEN 'first' ELSE 'corresponding' END AS author_type";

        let link_cypher = "MATCH (a1:Author)-[r:CO_AUTHOR_OF {workspace_id: $workspace_id}]-(a2:Author)
                          WHERE a1.id < a2.id
                          RETURN a1.id AS source, a2.id AS target, r.paper_count AS paper_count";

        let graph = self.graph.clone();
        let ws_id_clone = workspace_id.to_string();

        let nodes_handle = tokio::spawn(async move {
            let query = neo4rs::query(node_cypher).param("workspace_id", ws_id_clone.as_str());
            let mut result = graph.execute(query).await?;
            let mut nodes = Vec::with_capacity(DEFAULT_GRAPH_NODES_CAPACITY);
            while let Some(row) = result.next().await? {
                nodes.push(crate::models::dto::GraphNode {
                    id: row.get("id")?,
                    name: row.get("name")?,
                    paper_count: row.get("paper_count")?,
                    author_type: row.get("author_type")?,
                });
            }
            Ok::<Vec<crate::models::dto::GraphNode>, AppError>(nodes)
        });

        let graph2 = self.graph.clone();
        let ws_id_clone2 = workspace_id.to_string();

        let links_handle = tokio::spawn(async move {
            let query = neo4rs::query(link_cypher).param("workspace_id", ws_id_clone2.as_str());
            let mut result = graph2.execute(query).await?;
            let mut links = Vec::with_capacity(DEFAULT_GRAPH_LINKS_CAPACITY);
            while let Some(row) = result.next().await? {
                links.push(crate::models::dto::GraphLink {
                    source: row.get("source")?,
                    target: row.get("target")?,
                    paper_count: row.get("paper_count")?,
                });
            }
            Ok::<Vec<crate::models::dto::GraphLink>, AppError>(links)
        });

        let nodes = nodes_handle.await.map_err(|e| AppError::Neo4jError(format!("Node task join error: {}", e)))??;
        let links = links_handle.await.map_err(|e| AppError::Neo4jError(format!("Link task join error: {}", e)))??;
        
        Ok((nodes, links))
    }

    pub async fn search_by_keyword(&self, workspace_id: &str, query_str: &str) -> Result<Vec<crate::models::paper::Paper>, AppError> {
        let cypher = "MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)
                      OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)
                      WHERE (p.title CONTAINS $query OR p.abstract CONTAINS $query)
                        OR k.name CONTAINS $query
                      WITH DISTINCT p
                      ORDER BY p.year DESC
                      RETURN p
                      LIMIT 100";
        let query = neo4rs::query(cypher)
            .param("workspace_id", workspace_id)
            .param("query", query_str);

        let mut result = run_query!(self, query);
        let mut papers = Vec::with_capacity(DEFAULT_PAPERS_CAPACITY);
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("p")?;
            papers.push(paper_from_node(&node));
        }
        Ok(papers)
    }

    pub async fn search_by_author(&self, workspace_id: &str, author_name: &str) -> Result<Vec<crate::models::dto::AuthorWithPapers>, AppError> {
        // Use CONTAINS alone since STARTS WITH matches are a subset of CONTAINS matches;
        // this avoids scanning the Author index twice and deduplicating in application code.
        let cypher = "MATCH (a:Author) 
                      WHERE a.name CONTAINS $author_name
                      MATCH (a)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p:Paper)<-[:CONTAINS]-(w:Workspace {id: $workspace_id}) 
                      RETURN a, collect(DISTINCT p) AS papers 
                      ORDER BY size(papers) DESC 
                      LIMIT 20";
        let query = neo4rs::query(cypher)
            .param("workspace_id", workspace_id)
            .param("author_name", author_name);

        let mut result = run_query!(self, query);
        let mut authors_with_papers = Vec::with_capacity(DEFAULT_AUTHORS_CAPACITY);
        while let Some(row) = result.next().await? {
            let author_node: neo4rs::Node = row.get("a")?;
            let paper_nodes: Vec<neo4rs::Node> = row.get("papers")?;
            let mut papers = Vec::with_capacity(paper_nodes.len());
            for n in &paper_nodes {
                papers.push(paper_from_node(n));
            }
            authors_with_papers.push(crate::models::dto::AuthorWithPapers {
                author: author_from_node(&author_node),
                papers,
            });
        }
        Ok(authors_with_papers)
    }

    pub async fn get_papers_for_export(&self, workspace_id: &str, author_ids: Option<&[String]>, keyword_ids: Option<&[String]>, _year_range: Option<(i32, i32)>) -> Result<Vec<crate::models::paper::Paper>, AppError> {
        // Pre-calculate capacity based on expected query size
        const BASE_CAPACITY: usize = 150;
        const AUTHOR_FILTER_SIZE: usize = 80;
        const KEYWORD_FILTER_SIZE: usize = 60;
        
        let capacity = BASE_CAPACITY 
            + author_ids.filter(|a| !a.is_empty()).map_or(0, |_| AUTHOR_FILTER_SIZE)
            + keyword_ids.filter(|k| !k.is_empty()).map_or(0, |_| KEYWORD_FILTER_SIZE);
        
        let mut cypher = String::with_capacity(capacity);
        cypher.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");

        let mut has_conditions = false;

        if let Some(aids) = author_ids {
            if !aids.is_empty() {
                cypher.push_str(" MATCH (a:Author)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p)");
                cypher.push_str(" WHERE a.id IN $author_ids");
                has_conditions = true;
            }
        }
        if let Some(kids) = keyword_ids {
            if !kids.is_empty() {
                cypher.push_str(" MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)");
                if has_conditions {
                    cypher.push_str(" AND k.id IN $keyword_ids");
                } else {
                    cypher.push_str(" WHERE k.id IN $keyword_ids");
                }
            }
        }

        cypher.push_str(" RETURN DISTINCT p ORDER BY p.year DESC LIMIT 200");

        let mut query = neo4rs::query(&cypher)
            .param("workspace_id", workspace_id);

        if let Some(aids) = author_ids {
            query = query.param("author_ids", aids);
        }
        if let Some(kids) = keyword_ids {
            query = query.param("keyword_ids", kids);
        }

        let mut result = run_query!(self, query);
        let mut papers = Vec::with_capacity(DEFAULT_PAPERS_CAPACITY);
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("p")?;
            papers.push(paper_from_node(&node));
        }
        Ok(papers)
    }

    pub async fn get_papers_detail_batch(&self, workspace_id: &str, author_ids: Option<&[String]>, keyword_ids: Option<&[String]>, year_range: Option<(i32, i32)>) -> Result<Vec<(
        crate::models::paper::Paper,
        Option<crate::models::author::Author>,
        Option<crate::models::author::Author>,
        Vec<crate::models::keyword::Keyword>,
    )>, AppError> {
        // Pre-calculate capacity based on expected query size
        const BASE_CAPACITY: usize = 300;
        const AUTHOR_FILTER_SIZE: usize = 80;
        const KEYWORD_FILTER_SIZE: usize = 60;
        const YEAR_FILTER_SIZE: usize = 50;
        
        let capacity = BASE_CAPACITY 
            + author_ids.filter(|a| !a.is_empty()).map_or(0, |_| AUTHOR_FILTER_SIZE)
            + keyword_ids.filter(|k| !k.is_empty()).map_or(0, |_| KEYWORD_FILTER_SIZE)
            + year_range.map_or(0, |_| YEAR_FILTER_SIZE);
        
        let mut cypher = String::with_capacity(capacity);
        cypher.push_str("MATCH (w:Workspace {id: $workspace_id})-[:CONTAINS]->(p:Paper)");

        let mut has_conditions = false;

        if let Some(aids) = author_ids {
            if !aids.is_empty() {
                cypher.push_str(" MATCH (a:Author)-[:FIRST_AUTHOR_OF|CORRESPONDING_AUTHOR_OF]->(p)");
                cypher.push_str(" WHERE a.id IN $author_ids");
                has_conditions = true;
            }
        }
        if let Some(kids) = keyword_ids {
            if !kids.is_empty() {
                cypher.push_str(" MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)");
                if has_conditions {
                    cypher.push_str(" AND k.id IN $keyword_ids");
                } else {
                    cypher.push_str(" WHERE k.id IN $keyword_ids");
                }
                has_conditions = true;
            }
        }
        if let Some((start_year, end_year)) = year_range {
            use std::fmt::Write;
            if has_conditions {
                write!(cypher, " AND p.year >= {} AND p.year <= {}", start_year, end_year).unwrap();
            } else {
                write!(cypher, " WHERE p.year >= {} AND p.year <= {}", start_year, end_year).unwrap();
            }
        }

        cypher.push_str(
            " WITH DISTINCT p
              OPTIONAL MATCH (fa:Author)-[:FIRST_AUTHOR_OF]->(p)
              OPTIONAL MATCH (ca:Author)-[:CORRESPONDING_AUTHOR_OF]->(p)
              OPTIONAL MATCH (p)-[:HAS_KEYWORD]->(k:Keyword)
              WITH p, head(collect(DISTINCT fa)) AS fa, head(collect(DISTINCT ca)) AS ca, collect(DISTINCT k) AS keywords
              RETURN p, fa, ca, keywords
              ORDER BY p.year DESC
              LIMIT 50"
        );

        let mut query = neo4rs::query(&cypher)
            .param("workspace_id", workspace_id);

        if let Some(aids) = author_ids {
            query = query.param("author_ids", aids);
        }
        if let Some(kids) = keyword_ids {
            query = query.param("keyword_ids", kids);
        }

        let mut result = run_query!(self, query);
        let mut papers_detail = Vec::with_capacity(DEFAULT_PAPERS_CAPACITY);
        while let Some(row) = result.next().await? {
            let paper_node: neo4rs::Node = row.get("p")?;
            let first_author: Option<neo4rs::Node> = row.get("fa").ok();
            let corresponding_author: Option<neo4rs::Node> = row.get("ca").ok();
            let keyword_nodes: Vec<neo4rs::Node> = row.get("keywords").unwrap_or_default();

            let first_author = first_author.map(|n| author_from_node(&n));
            let corresponding_author = corresponding_author.map(|n| author_from_node(&n));
            let keywords: Vec<_> = keyword_nodes.iter().map(keyword_from_node).collect();

            papers_detail.push((paper_from_node(&paper_node), first_author, corresponding_author, keywords));
        }
        Ok(papers_detail)
    }
}

fn workspace_from_node(node: &neo4rs::Node) -> Workspace {
    // Use get with unwrap_or_default to avoid redundant Option handling
    Workspace {
        id: node.get::<String>("id").unwrap_or_default(),
        name: node.get::<String>("name").unwrap_or_default(),
        description: node.get::<String>("description").unwrap_or_default(),
        created_at: node.get::<String>("created_at").unwrap_or_default(),
    }
}

fn paper_from_node(node: &neo4rs::Node) -> crate::models::paper::Paper {
    // Helper function to filter empty strings - more efficient than filter chain
    fn non_empty_string(s: String) -> Option<String> {
        if s.is_empty() { None } else { Some(s) }
    }
    
    // Use helper function for cleaner and slightly more efficient filtering
    fn non_zero_year(y: i32) -> Option<i32> {
        if y > 0 { Some(y) } else { None }
    }
    
    crate::models::paper::Paper {
        id: node.get::<String>("id").unwrap_or_default(),
        title: node.get::<String>("title").unwrap_or_default(),
        doi: node.get::<String>("doi").ok().and_then(non_empty_string),
        arxiv_id: node.get::<String>("arxiv_id").ok().and_then(non_empty_string),
        abstract_text: node.get::<String>("abstract").ok().and_then(non_empty_string),
        user_notes: node.get::<String>("user_notes").ok().and_then(non_empty_string),
        year: node.get::<i32>("year").ok().and_then(non_zero_year),
        journal: node.get::<String>("journal").ok().and_then(non_empty_string),
        created_at: node.get::<String>("created_at").unwrap_or_default(),
    }
}

fn author_from_node(node: &neo4rs::Node) -> crate::models::author::Author {
    fn non_empty_string(s: String) -> Option<String> {
        if s.is_empty() { None } else { Some(s) }
    }
    
    crate::models::author::Author {
        id: node.get::<String>("id").unwrap_or_default(),
        name: node.get::<String>("name").unwrap_or_default(),
        orcid: node.get::<String>("orcid").ok().and_then(non_empty_string),
    }
}

fn keyword_from_node(node: &neo4rs::Node) -> crate::models::keyword::Keyword {
    crate::models::keyword::Keyword {
        id: node.get::<String>("id").unwrap_or_default(),
        name: node.get::<String>("name").unwrap_or_default(),
    }
}
