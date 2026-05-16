import { useState } from 'react';
import { Modal, Input, message } from 'antd';
import { ImportOutlined } from '@ant-design/icons';
import { importPaper } from '../api/paper';

interface Props {
  workspaceId: string;
  open: boolean;
  onClose: () => void;
  onImported: () => void;
}

export default function PaperImport({ workspaceId, open, onClose, onImported }: Props) {
  const [identifier, setIdentifier] = useState('');
  const [loading, setLoading] = useState(false);

  const handleOk = async () => {
    if (!identifier.trim()) return;
    setLoading(true);
    try {
      await importPaper(workspaceId, identifier.trim());
      message.success('导入成功');
      setIdentifier('');
      onImported();
      onClose();
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : '导入失败';
      message.error(msg);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal
      title={
        <span style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <ImportOutlined style={{ color: 'var(--accent)' }} />
          导入论文
        </span>
      }
      open={open}
      onOk={handleOk}
      onCancel={onClose}
      confirmLoading={loading}
    >
      <div style={{ marginBottom: 16 }}>
        <Input
          placeholder="输入 DOI 或 arXiv ID"
          value={identifier}
          onChange={(e) => setIdentifier(e.target.value)}
          size="large"
          style={{ fontSize: 15 }}
        />
      </div>
      <div
        style={{
          color: 'var(--text-muted)',
          fontSize: 13,
          lineHeight: 1.6,
          background: 'var(--bg-elevated)',
          padding: '12px 16px',
          borderRadius: 'var(--radius-sm)',
          border: '1px solid var(--border)',
        }}
      >
        <div style={{ fontWeight: 500, marginBottom: 4, color: 'var(--text-secondary)' }}>
          支持的格式：
        </div>
        <div>• DOI: <code style={{ color: 'var(--accent)' }}>10.1038/s41586-020-2649-2</code></div>
        <div>• arXiv: <code style={{ color: 'var(--accent)' }}>2301.12345</code></div>
      </div>
    </Modal>
  );
}
