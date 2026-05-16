import { get, post, put, del } from './client';
import type { Paper, PaperDetail } from '../types';

export const importPaper = (workspaceId: string, identifier: string) =>
  post<PaperDetail>(`/workspaces/${workspaceId}/papers`, { identifier });

export const listPapers = (workspaceId: string) =>
  get<Paper[]>(`/workspaces/${workspaceId}/papers`);

export const getPaper = (id: string) => get<PaperDetail>(`/papers/${id}`);

export const updatePaper = (id: string, data: { user_notes?: string }) =>
  put<Paper>(`/papers/${id}`, data);

export const deletePaper = (workspaceId: string, paperId: string) =>
  del<{ removed: boolean }>(`/workspaces/${workspaceId}/papers/${paperId}`);
