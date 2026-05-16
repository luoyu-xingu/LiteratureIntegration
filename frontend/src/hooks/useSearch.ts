import { useState } from 'react';
import { searchByKeyword, searchByAuthor } from '../api/search';
import type { Paper, AuthorWithPapers } from '../types';

export function useSearch(workspaceId: string | undefined) {
  const [results, setResults] = useState<Paper[] | AuthorWithPapers[] | null>(null);
  const [mode, setMode] = useState<'keyword' | 'author'>('keyword');
  const [loading, setLoading] = useState(false);

  const search = async (query: string, searchMode: 'keyword' | 'author') => {
    if (!workspaceId || !query.trim()) return;
    setLoading(true);
    setMode(searchMode);
    try {
      if (searchMode === 'keyword') {
        const res = await searchByKeyword(workspaceId, query);
        setResults(res.results as Paper[]);
      } else {
        const res = await searchByAuthor(workspaceId, query);
        setResults(res.results as AuthorWithPapers[]);
      }
    } catch { } finally {
      setLoading(false);
    }
  };

  return { results, mode, loading, search };
}
