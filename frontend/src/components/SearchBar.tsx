import { useState } from 'react';
import { Input, Radio, Button } from 'antd';
import { SearchOutlined } from '@ant-design/icons';

interface Props {
  onSearch: (query: string, mode: 'keyword' | 'author') => void;
  loading: boolean;
}

export default function SearchBar({ onSearch, loading }: Props) {
  const [query, setQuery] = useState('');
  const [mode, setMode] = useState<'keyword' | 'author'>('keyword');

  return (
    <div style={{ display: 'flex', gap: 12, alignItems: 'center', marginBottom: 16 }}>
      <Radio.Group value={mode} onChange={(e) => setMode(e.target.value)}>
        <Radio.Button value="keyword">关键词搜索</Radio.Button>
        <Radio.Button value="author">作者搜索</Radio.Button>
      </Radio.Group>
      <Input
        style={{ width: 300 }}
        placeholder={mode === 'keyword' ? '搜索关键词...' : '搜索作者姓名...'}
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        onPressEnter={() => onSearch(query, mode)}
      />
      <Button type="primary" icon={<SearchOutlined />} loading={loading} onClick={() => onSearch(query, mode)}>
        搜索
      </Button>
    </div>
  );
}
