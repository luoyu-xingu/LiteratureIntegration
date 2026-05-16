import { get } from './client';

export const searchByKeyword = (workspaceId: string, query: string) =>
  get<{ mode: string; query: string; results: any[] }>(`/workspaces/${workspaceId}/search?q=${encodeURIComponent(query)}`);

export const searchByAuthor = (workspaceId: string, author: string) =>
  get<{ mode: string; query: string; results: any[] }>(`/workspaces/${workspaceId}/search?author=${encodeURIComponent(author)}`);
