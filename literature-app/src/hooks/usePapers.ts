import { useState, useEffect } from 'react';
import { listPapers } from '../api/paper';
import type { Paper } from '../types';

export function usePapers(workspaceId: string | undefined) {
  const [papers, setPapers] = useState<Paper[]>([]);
  const [loading, setLoading] = useState(false);

  const load = async () => {
    if (!workspaceId) return;
    setLoading(true);
    try {
      const data = await listPapers(workspaceId);
      setPapers(data);
    } catch { } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(); }, [workspaceId]);

  return { papers, loading, reload: load };
}
