import { get } from './client';
import type { Author, GraphData, AuthorWithPapers } from '../types';

export const listAuthors = (workspaceId: string) =>
  get<Author[]>(`/workspaces/${workspaceId}/authors`);

export const getGraphData = (workspaceId: string) =>
  get<GraphData>(`/workspaces/${workspaceId}/authors/graph`);

export const getAuthorPapers = (authorId: string) =>
  get<AuthorWithPapers>(`/authors/${authorId}/papers`);
