import { get } from './client';

export const searchByKeyword = (workspaceId: string, query: string) =>
  get<{ mode: string; query: string; results: any[] }>(`/search?workspace_id=${workspaceId}&q=${encodeURIComponent(query)}`);

export const searchByAuthor = (workspaceId: string, author: string) =>
  get<{ mode: string; query: string; results: any[] }>(`/search?workspace_id=${workspaceId}&author=${encodeURIComponent(author)}`);
