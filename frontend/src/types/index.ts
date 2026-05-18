export interface Workspace {
  id: string;
  name: string;
  description: string;
  created_at: string;
}

export interface Paper {
  id: string;
  title: string;
  doi: string | null;
  arxiv_id: string | null;
  abstract_text: string | null;
  user_notes: string | null;
  year: number | null;
  journal: string | null;
  created_at: string;
}

export interface Author {
  id: string;
  name: string;
  orcid: string | null;
}

export interface Keyword {
  id: string;
  name: string;
}

export interface PaperDetail {
  paper: Paper;
  first_author: Author | null;
  corresponding_author: Author | null;
  keywords: Keyword[];
}

export interface GraphNode {
  id: string;
  name: string;
  paper_count: number;
  author_type: 'first' | 'corresponding' | 'both';
}

export interface GraphLink {
  source: string;
  target: string;
  paper_count: number;
}

export interface GraphData {
  nodes: GraphNode[];
  links: GraphLink[];
}

export interface AuthorWithPapers {
  author: Author;
  papers: Paper[];
}

export interface ExportRequest {
  format: string;
  group_by?: string;
  filter?: {
    author_ids?: string[];
    keyword_ids?: string[];
    year_range?: [number, number];
  };
}
