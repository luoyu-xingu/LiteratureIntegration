import { get, post, put, del } from './client';
import type { Paper, PaperDetail } from '../types';

export const importPaper = (workspaceId: string, identifier: string) =>
  post<PaperDetail>(`/papers?workspace_id=${workspaceId}`, { identifier });

export const listPapers = (workspaceId: string) =>
  get<Paper[]>(`/papers?workspace_id=${workspaceId}`);

export const getPaper = (id: string) => get<PaperDetail>(`/paper/${id}`);

export const updatePaper = (id: string, data: { user_notes?: string }) =>
  put<Paper>(`/paper/${id}`, data);

export const deletePaper = (workspaceId: string, paperId: string) =>
  del<{ removed: boolean }>(`/paper-rm?workspace_id=${workspaceId}&paper_id=${paperId}`);
