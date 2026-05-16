import { tauriInvoke } from './client';

export interface SearchResponse {
  mode: string;
  results: any;
}

export const searchByKeyword = (workspaceId: string, query: string) =>
  tauriInvoke<SearchResponse>('search', { workspaceId, query, mode: 'keyword' });
export const searchByAuthor = (workspaceId: string, query: string) =>
  tauriInvoke<SearchResponse>('search', { workspaceId, query, mode: 'author' });
export const searchByContent = (workspaceId: string, query: string) =>
  tauriInvoke<SearchResponse>('search', { workspaceId, query, mode: 'content' });
