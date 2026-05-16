import { tauriInvoke } from './client';
import type { Author, GraphData, AuthorWithPapers } from '../types';

export const listAuthors = (workspaceId: string) =>
  tauriInvoke<Author[]>('get_authors', { workspaceId });
export const getGraphData = (workspaceId: string) =>
  tauriInvoke<GraphData>('get_graph', { workspaceId });
export const getAuthorPapers = (authorName: string) =>
  tauriInvoke<AuthorWithPapers>('get_author_papers', { authorName });
