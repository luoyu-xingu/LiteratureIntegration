import { tauriInvoke } from './client';

export const exportWorkspace = (workspaceId: string, groupBy: string = 'author') =>
  tauriInvoke<string>('export_workspace', { workspaceId, groupBy });
