use neo4rs::Graph;
use crate::models::workspace::Workspace;
use crate::errors::AppError;

pub struct Neo4jRepo {
    graph: Graph,
}

impl Neo4jRepo {
    pub fn new(graph: Graph) -> Self {
        Self { graph }
    }

    pub async fn create_workspace(&self, id: &str, name: &str, description: &str, created_at: &str) -> Result<Workspace, AppError> {
        let query = neo4rs::query(
            "CREATE (w:Workspace {id: $id, name: $name, description: $description, created_at: $created_at}) RETURN w"
        )
        .param("id", id)
        .param("name", name)
        .param("description", description)
        .param("created_at", created_at);

        let mut result = self.graph.execute(query).await?;
        let row = result.next().await?.ok_or_else(|| AppError::Neo4jError("No row returned".into()))?;
        let node: neo4rs::Node = row.get("w")?;
        Ok(workspace_from_node(&node))
    }

    pub async fn list_workspaces(&self) -> Result<Vec<Workspace>, AppError> {
        let query = neo4rs::query("MATCH (w:Workspace) RETURN w ORDER BY w.created_at DESC");
        let mut result = self.graph.execute(query).await?;
        let mut workspaces = Vec::new();
        while let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("w")?;
            workspaces.push(workspace_from_node(&node));
        }
        Ok(workspaces)
    }

    pub async fn get_workspace(&self, id: &str) -> Result<Option<Workspace>, AppError> {
        let query = neo4rs::query("MATCH (w:Workspace {id: $id}) RETURN w").param("id", id);
        let mut result = self.graph.execute(query).await?;
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

        let mut result = self.graph.execute(query).await?;
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

        let mut result = self.graph.execute(query).await?;
        if let Some(row) = result.next().await? {
            let deleted: i64 = row.get("deleted")?;
            Ok(deleted > 0)
        } else {
            Ok(false)
        }
    }
}

fn workspace_from_node(node: &neo4rs::Node) -> Workspace {
    Workspace {
        id: node.get::<String>("id").unwrap_or_default(),
        name: node.get::<String>("name").unwrap_or_default(),
        description: node.get::<String>("description").unwrap_or_default(),
        created_at: node.get::<String>("created_at").unwrap_or_default(),
    }
}
