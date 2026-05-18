import { downloadMarkdown } from './client';
import type { ExportRequest } from '../types';

export const exportWorkspace = (workspaceId: string, req: ExportRequest) =>
  downloadMarkdown(`/export?workspace_id=${workspaceId}`, req);
