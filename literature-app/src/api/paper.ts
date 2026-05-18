import { tauriInvoke } from './client';
import type { Paper, PaperDetail } from '../types';

export const listPapers = (workspaceId: string) =>
  tauriInvoke<Paper[]>('list_papers', { workspaceId });
export const importPaper = (workspaceId: string, identifier: string) =>
  tauriInvoke<PaperDetail>('import_paper', { workspaceId, req: { identifier } });
export const getPaper = (id: string) =>
  tauriInvoke<PaperDetail>('get_paper', { id });
export const updatePaper = (id: string, data: { user_notes?: string }) =>
  tauriInvoke<PaperDetail>('update_paper', { id, req: data });
export const deletePaper = (workspaceId: string, paperId: string) =>
  tauriInvoke<boolean>('delete_paper', { workspaceId, paperId });
