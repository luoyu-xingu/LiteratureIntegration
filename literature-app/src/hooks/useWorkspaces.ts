import { useState, useEffect } from 'react';
import { listWorkspaces } from '../api/workspace';
import type { Workspace } from '../types';

export function useWorkspaces() {
  const [workspaces, setWorkspaces] = useState<Workspace[]>([]);
  const [loading, setLoading] = useState(false);

  const load = async () => {
    setLoading(true);
    try {
      const data = await listWorkspaces();
      setWorkspaces(data);
    } catch { } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(); }, []);

  return { workspaces, loading, reload: load };
}
