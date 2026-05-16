import { useState, useEffect } from 'react';
import { getGraphData } from '../api/author';
import type { GraphData } from '../types';

export function useGraph(workspaceId: string | undefined) {
  const [data, setData] = useState<GraphData | null>(null);
  const [loading, setLoading] = useState(false);

  const load = async () => {
    if (!workspaceId) return;
    setLoading(true);
    try {
      const result = await getGraphData(workspaceId);
      setData(result);
    } catch { } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(); }, [workspaceId]);

  return { data, loading, reload: load };
}
