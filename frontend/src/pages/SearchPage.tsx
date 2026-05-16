import { useParams } from 'react-router-dom';
import SearchBar from '../components/SearchBar';
import SearchResult from '../components/SearchResult';
import ExportPanel from '../components/ExportPanel';
import { useSearch } from '../hooks/useSearch';

export default function SearchPage() {
  const { id } = useParams<{ id: string }>();
  const { results, mode, loading, search } = useSearch(id);

  return (
    <div className="animate-fade-in">
      <h2
        style={{
          fontFamily: 'var(--font-display)',
          fontSize: 28,
          fontWeight: 700,
          marginBottom: 24,
        }}
      >
        搜索论文
      </h2>
      <SearchBar onSearch={search} loading={loading} />
      <SearchResult results={results} mode={mode} />
      <div style={{ marginTop: 32 }}>
        {id && <ExportPanel workspaceId={id} />}
      </div>
    </div>
  );
}
