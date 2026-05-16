import { get, post, put, del } from './client';
import type { Workspace } from '../types';

export const createWorkspace = (data: { name: string; description?: string }) =>
  post<Workspace>('/workspaces', data);

export const listWorkspaces = () => get<Workspace[]>('/workspaces');

export const getWorkspace = (id: string) => get<Workspace>(`/workspace/${id}`);

export const updateWorkspace = (id: string, data: { name?: string; description?: string }) =>
  put<Workspace>(`/workspace/${id}`, data);

export const deleteWorkspace = (id: string) => del<{ deleted: boolean }>(`/workspace/${id}`);
