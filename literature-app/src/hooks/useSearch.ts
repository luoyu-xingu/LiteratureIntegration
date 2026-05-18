import { useState } from 'react';
import { searchByKeyword, searchByAuthor, searchByContent } from '../api/search';
import type { Paper, AuthorWithPapers } from '../types';

export function useSearch(workspaceId: string | undefined) {
  const [results, setResults] = useState<Paper[] | AuthorWithPapers[] | null>(null);
  const [mode, setMode] = useState<'keyword' | 'author' | 'content'>('keyword');
  const [loading, setLoading] = useState(false);

  const search = async (query: string, searchMode: 'keyword' | 'author' | 'content') => {
    if (!workspaceId || !query.trim()) return;
    setLoading(true);
    setMode(searchMode);
    try {
      if (searchMode === 'keyword') {
        const res = await searchByKeyword(workspaceId, query);
        setResults(res.results as Paper[]);
      } else if (searchMode === 'author') {
        const res = await searchByAuthor(workspaceId, query);
        setResults(res.results as AuthorWithPapers[]);
      } else {
        const res = await searchByContent(workspaceId, query);
        setResults(res.results as Paper[]);
      }
    } catch { } finally {
      setLoading(false);
    }
  };

  return { results, mode, loading, search };
}
