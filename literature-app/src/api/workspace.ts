import { tauriInvoke } from './client';
import type { Workspace } from '../types';

export const listWorkspaces = () => tauriInvoke<Workspace[]>('list_workspaces');
export const createWorkspace = (data: { name: string; description?: string }) =>
  tauriInvoke<Workspace>('create_workspace', { req: data });
export const getWorkspace = (id: string) => tauriInvoke<Workspace>('get_workspace', { id });
export const updateWorkspace = (id: string, data: { name?: string; description?: string }) =>
  tauriInvoke<Workspace>('update_workspace', { id, req: data });
export const deleteWorkspace = (id: string) => tauriInvoke<boolean>('delete_workspace', { id });
