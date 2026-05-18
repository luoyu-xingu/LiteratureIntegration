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
    <div
      className="animate-fade-in"
      style={{
        display: 'flex',
        gap: 12,
        alignItems: 'center',
        marginBottom: 24,
      }}
    >
      <Radio.Group
        value={mode}
        onChange={(e) => setMode(e.target.value)}
        size="small"
      >
        <Radio.Button value="keyword">关键词</Radio.Button>
        <Radio.Button value="author">作者</Radio.Button>
      </Radio.Group>
      <Input
        style={{ flex: 1, maxWidth: 400 }}
        placeholder={mode === 'keyword' ? '搜索关键词...' : '搜索作者姓名...'}
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        onPressEnter={() => onSearch(query, mode)}
        size="large"
      />
      <Button
        type="primary"
        icon={<SearchOutlined />}
        loading={loading}
        onClick={() => onSearch(query, mode)}
        size="large"
      >
        搜索
      </Button>
    </div>
  );
}
