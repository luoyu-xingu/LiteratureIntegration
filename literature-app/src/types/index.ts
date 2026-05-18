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
  year: number | null;
  journal: string | null;
  first_author: string | null;
  corresponding_author: string | null;
  keywords: string[];
  created_at: string;
}

export interface PaperDetail {
  paper: Paper;
  abstract_text: string | null;
  user_notes: string | null;
}

export interface Author {
  name: string;
  first_author_count: number;
  corresponding_author_count: number;
  paper_count: number;
}

export interface GraphNode {
  id: string;
  name: string;
  paper_count: number;
  author_type: string;
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
  author_name: string;
  papers: Paper[];
}

export interface ExportRequest {
  format: string;
  group_by?: string;
}
