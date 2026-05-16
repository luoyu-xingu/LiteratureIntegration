import { useParams } from 'react-router-dom';
import SearchBar from '../components/SearchBar';
import SearchResult from '../components/SearchResult';
import ExportPanel from '../components/ExportPanel';
import { useSearch } from '../hooks/useSearch';

export default function SearchPage() {
  const { id } = useParams<{ id: string }>();
  const { results, mode, loading, search } = useSearch(id);

  return (
    <div>
      <h2>搜索</h2>
      <SearchBar onSearch={search} loading={loading} />
      <SearchResult results={results} mode={mode} />
      <div style={{ marginTop: 24 }}>
        {id && <ExportPanel workspaceId={id} />}
      </div>
    </div>
  );
}
