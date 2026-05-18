import { Tag } from 'antd';
import { FileTextOutlined, LinkOutlined } from '@ant-design/icons';
import type { Paper } from '../types';
import { useNavigate } from 'react-router-dom';

interface Props {
  papers: Paper[];
  loading: boolean;
}

export default function PaperList({ papers, loading }: Props) {
  const navigate = useNavigate();

  if (!loading && papers.length === 0) {
    return (
      <div
        className="animate-fade-in"
        style={{
          textAlign: 'center',
          padding: '60px 0',
          color: 'var(--text-muted)',
        }}
      >
        <FileTextOutlined style={{ fontSize: 40, marginBottom: 12, opacity: 0.3 }} />
        <div style={{ fontSize: 15 }}>暂无论文，点击「导入论文」开始</div>
      </div>
    );
  }

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
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.borderColor = 'var(--border-light)';
            e.currentTarget.style.background = 'var(--bg-elevated)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.borderColor = 'var(--border)';
            e.currentTarget.style.background = 'var(--bg-surface)';
          }}
          onClick={() => navigate(`/paper/${paper.id}`)}
        >
          <div
            style={{
              fontSize: 15,
              fontWeight: 500,
              color: 'var(--text-primary)',
              marginBottom: 6,
              lineHeight: 1.4,
            }}
          >
            {paper.title}
          </div>
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: 12,
              flexWrap: 'wrap',
              fontSize: 13,
              color: 'var(--text-secondary)',
            }}
          >
            {paper.year && (
              <span style={{ fontWeight: 600, color: 'var(--accent)' }}>
                {paper.year}
              </span>
            )}
            {paper.journal && <span>{paper.journal}</span>}
            {paper.doi && (
              <Tag
                icon={<LinkOutlined />}
                style={{ fontSize: 11, margin: 0 }}
              >
                DOI
              </Tag>
            )}
            {paper.arxiv_id && (
              <Tag style={{ fontSize: 11, margin: 0 }}>arXiv</Tag>
            )}
          </div>
        </div>
      ))}
    </div>
  );
}
