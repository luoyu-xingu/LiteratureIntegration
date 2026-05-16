import { get } from './client';
import type { Author, GraphData, AuthorWithPapers } from '../types';

export const listAuthors = (workspaceId: string) =>
  get<Author[]>(`/authors?workspace_id=${workspaceId}`);

export const getGraphData = (workspaceId: string) =>
  get<GraphData>(`/graph?workspace_id=${workspaceId}`);

export const getAuthorPapers = (authorId: string) =>
  get<AuthorWithPapers>(`/author-papers/${authorId}`);
