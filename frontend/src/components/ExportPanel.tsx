import { useState } from 'react';
import { Card, Radio, Button, message } from 'antd';
import { DownloadOutlined } from '@ant-design/icons';
import { exportWorkspace } from '../api/export';
import type { ExportRequest } from '../types';

interface Props {
  workspaceId: string;
}

export default function ExportPanel({ workspaceId }: Props) {
  const [groupBy, setGroupBy] = useState<'author' | 'keyword'>('author');
  const [loading, setLoading] = useState(false);

  const handleExport = async () => {
    setLoading(true);
    try {
      const req: ExportRequest = { format: 'markdown', group_by: groupBy };
      await exportWorkspace(workspaceId, req);
      message.success('导出成功');
    } catch {
      message.error('导出失败');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card title="导出 Markdown">
      <div style={{ marginBottom: 16 }}>
        <div style={{ marginBottom: 8 }}>分组方式:</div>
        <Radio.Group value={groupBy} onChange={(e) => setGroupBy(e.target.value)}>
          <Radio.Button value="author">按作者分组</Radio.Button>
          <Radio.Button value="keyword">按关键词分组</Radio.Button>
        </Radio.Group>
      </div>
      <Button type="primary" icon={<DownloadOutlined />} loading={loading} onClick={handleExport}>
        下载 .md
      </Button>
    </Card>
  );
}
