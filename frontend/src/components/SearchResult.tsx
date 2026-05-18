import { Tag } from 'antd';
import { FileTextOutlined, UserOutlined } from '@ant-design/icons';
import type { Paper, AuthorWithPapers } from '../types';
import { useNavigate } from 'react-router-dom';

interface Props {
  results: Paper[] | AuthorWithPapers[] | null;
  mode: 'keyword' | 'author';
}

export default function SearchResult({ results, mode }: Props) {
  const navigate = useNavigate();

  if (!results) return null;

  if (results.length === 0) {
    return (
      <div
        className="animate-fade-in"
        style={{
          textAlign: 'center',
          padding: '40px 0',
          color: 'var(--text-muted)',
        }}
      >
        无搜索结果
      </div>
    );
  }

  if (mode === 'keyword') {
    const papers = results as Paper[];
    return (
      <div style={{ display: 'grid', gap: 10 }}>
        {papers.map((paper, i) => (
          <div
            key={paper.id}
            className="animate-fade-in"
            style={{
              animationDelay: `${i * 0.04}s`,
              background: 'var(--bg-surface)',
              border: '1px solid var(--border)',
              borderRadius: 'var(--radius-md)',
              padding: '14px 18px',
              cursor: 'pointer',
              transition: 'all var(--transition)',
              display: 'flex',
              alignItems: 'center',
              gap: 14,
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderColor = 'var(--border-light)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderColor = 'var(--border)';
            }}
            onClick={() => navigate(`/paper/${paper.id}`)}
          >
            <FileTextOutlined
              style={{ color: 'var(--accent)', fontSize: 18, flexShrink: 0 }}
            />
            <div style={{ flex: 1, minWidth: 0 }}>
              <div
                style={{
                  fontSize: 15,
                  fontWeight: 500,
                  color: 'var(--text-primary)',
                  marginBottom: 4,
                }}
              >
                {paper.title}
              </div>
              <div style={{ fontSize: 13, color: 'var(--text-secondary)' }}>
                {paper.year} · {paper.journal || '—'}
              </div>
            </div>
          </div>
        ))}
      </div>
    );
  }

  const authorResults = results as AuthorWithPapers[];
  return (
    <div style={{ display: 'grid', gap: 12 }}>
      {authorResults.map((item, i) => (
        <div
          key={item.author.id}
          className="animate-fade-in"
          style={{
            animationDelay: `${i * 0.04}s`,
            background: 'var(--bg-surface)',
            border: '1px solid var(--border)',
            borderRadius: 'var(--radius-md)',
            padding: '16px 20px',
          }}
        >
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: 8,
              marginBottom: 10,
            }}
          >
            <UserOutlined style={{ color: 'var(--accent)' }} />
            <span style={{ fontWeight: 600, color: 'var(--text-primary)' }}>
              {item.author.name}
            </span>
            <span
              style={{
                fontSize: 12,
                color: 'var(--text-muted)',
                marginLeft: 4,
              }}
            >
              {item.papers.length} 篇论文
            </span>
          </div>
          <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap' }}>
            {item.papers.map((p) => (
              <Tag
                key={p.id}
                style={{ cursor: 'pointer' }}
                onClick={() => navigate(`/paper/${p.id}`)}
              >
                {p.title.length > 40 ? p.title.slice(0, 40) + '…' : p.title}
              </Tag>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
