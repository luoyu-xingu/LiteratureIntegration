import { useState } from 'react';
import { Radio, Button, message } from 'antd';
import { DownloadOutlined } from '@ant-design/icons';
import { exportWorkspace } from '../api/export';

interface Props {
  workspaceId: string;
}

export default function ExportPanel({ workspaceId }: Props) {
  const [groupBy, setGroupBy] = useState<'author' | 'keyword'>('author');
  const [loading, setLoading] = useState(false);

  const handleExport = async () => {
    setLoading(true);
    try {
      await exportWorkspace(workspaceId, groupBy);
      message.success('导出成功');
    } catch {
      message.error('导出失败');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div
      className="animate-fade-in"
      style={{
        background: 'var(--bg-surface)',
        border: '1px solid var(--border)',
        borderRadius: 'var(--radius-md)',
        padding: '20px 24px',
      }}
    >
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          gap: 8,
          marginBottom: 16,
          fontSize: 13,
          color: 'var(--text-secondary)',
          fontWeight: 500,
        }}
      >
        <DownloadOutlined style={{ color: 'var(--accent)' }} />
        导出 Markdown
      </div>
      <div style={{ marginBottom: 16 }}>
        <div
          style={{
            marginBottom: 8,
            fontSize: 13,
            color: 'var(--text-muted)',
          }}
        >
          分组方式
        </div>
        <Radio.Group
          value={groupBy}
          onChange={(e) => setGroupBy(e.target.value)}
          size="small"
        >
          <Radio.Button value="author">按作者</Radio.Button>
          <Radio.Button value="keyword">按关键词</Radio.Button>
        </Radio.Group>
      </div>
      <Button
        type="primary"
        icon={<DownloadOutlined />}
        loading={loading}
        onClick={handleExport}
      >
        下载 .md
      </Button>
    </div>
  );
}
