import { Tag } from 'antd';
import {
  CalendarOutlined,
  BookOutlined,
  LinkOutlined,
  UserOutlined,
  TagOutlined,
  FileTextOutlined,
} from '@ant-design/icons';
import ReactMarkdown from 'react-markdown';
import type { PaperDetail as PaperDetailType } from '../types';

interface Props {
  detail: PaperDetailType;
}

export default function PaperDetail({ detail }: Props) {
  const { paper, abstract_text } = detail;

  return (
    <div className="animate-fade-in">
      <div
        style={{
          fontFamily: 'var(--font-display)',
          fontSize: 28,
          fontWeight: 700,
          lineHeight: 1.3,
          color: 'var(--text-primary)',
          marginBottom: 24,
        }}
      >
        {paper.title}
      </div>

      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))',
          gap: 16,
          marginBottom: 24,
        }}
      >
        <InfoCard
          icon={<CalendarOutlined />}
          label="年份"
          value={paper.year?.toString() || '—'}
          accent
        />
        <InfoCard
          icon={<BookOutlined />}
          label="期刊"
          value={paper.journal || '—'}
        />
        <InfoCard
          icon={<UserOutlined />}
          label="一作"
          value={paper.first_author || '—'}
        />
        <InfoCard
          icon={<UserOutlined />}
          label="通讯作者"
          value={paper.corresponding_author || '—'}
        />
      </div>

      {(paper.doi || paper.arxiv_id) && (
        <div
          style={{
            display: 'flex',
            gap: 10,
            marginBottom: 20,
            flexWrap: 'wrap',
          }}
        >
          {paper.doi && (
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 6,
                background: 'var(--bg-elevated)',
                padding: '6px 12px',
                borderRadius: 'var(--radius-sm)',
                fontSize: 13,
                border: '1px solid var(--border)',
              }}
            >
              <LinkOutlined style={{ color: 'var(--accent)' }} />
              <span style={{ color: 'var(--text-muted)' }}>DOI:</span>
              <span style={{ color: 'var(--accent)' }}>{paper.doi}</span>
            </div>
          )}
          {paper.arxiv_id && (
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 6,
                background: 'var(--bg-elevated)',
                padding: '6px 12px',
                borderRadius: 'var(--radius-sm)',
                fontSize: 13,
                border: '1px solid var(--border)',
              }}
            >
              <span style={{ color: 'var(--accent)' }}>arXiv:</span>
              <span style={{ color: 'var(--text-primary)' }}>{paper.arxiv_id}</span>
            </div>
          )}
        </div>
      )}

      {paper.keywords.length > 0 && (
        <div style={{ marginBottom: 24 }}>
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: 8,
              marginBottom: 10,
              fontSize: 13,
              color: 'var(--text-secondary)',
              fontWeight: 500,
            }}
          >
            <TagOutlined style={{ color: 'var(--accent)' }} />
            关键词
          </div>
          <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap' }}>
            {paper.keywords.map((k) => (
              <Tag key={k}>{k}</Tag>
            ))}
          </div>
        </div>
      )}

      {abstract_text && (
        <div style={{ marginBottom: 24 }}>
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: 8,
              marginBottom: 10,
              fontSize: 13,
              color: 'var(--text-secondary)',
              fontWeight: 500,
            }}
          >
            <FileTextOutlined style={{ color: 'var(--accent)' }} />
            Abstract
          </div>
          <div
            style={{
              background: 'var(--bg-surface)',
              border: '1px solid var(--border)',
              borderRadius: 'var(--radius-md)',
              padding: '16px 20px',
            }}
          >
            <div className="markdown-body" style={{ lineHeight: 1.7 }}>
              <ReactMarkdown>{abstract_text}</ReactMarkdown>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function InfoCard({
  icon,
  label,
  value,
  accent,
}: {
  icon: React.ReactNode;
  label: string;
  value: string;
  accent?: boolean;
}) {
  return (
    <div
      style={{
        background: 'var(--bg-surface)',
        border: '1px solid var(--border)',
        borderRadius: 'var(--radius-md)',
        padding: '12px 16px',
        display: 'flex',
        alignItems: 'center',
        gap: 12,
      }}
    >
      <div
        style={{
          width: 36,
          height: 36,
          borderRadius: 8,
          background: accent ? 'var(--accent-dim)' : 'var(--bg-elevated)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          color: 'var(--accent)',
          fontSize: 16,
          flexShrink: 0,
        }}
      >
        {icon}
      </div>
      <div>
        <div
          style={{
            fontSize: 11,
            color: 'var(--text-muted)',
            textTransform: 'uppercase',
            letterSpacing: '0.05em',
            marginBottom: 2,
          }}
        >
          {label}
        </div>
        <div
          style={{
            fontSize: 15,
            fontWeight: 500,
            color: accent ? 'var(--accent)' : 'var(--text-primary)',
          }}
        >
          {value}
        </div>
      </div>
    </div>
  );
}
