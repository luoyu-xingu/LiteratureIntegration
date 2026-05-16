import { useState } from 'react';
import { Modal, Input, message } from 'antd';
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
    <Modal title="导入论文" open={open} onOk={handleOk} onCancel={onClose} confirmLoading={loading}>
      <div style={{ marginBottom: 12 }}>
        <Input
          placeholder="输入 DOI 或 arXiv ID"
          value={identifier}
          onChange={(e) => setIdentifier(e.target.value)}
        />
      </div>
      <div style={{ color: '#999', fontSize: 12 }}>
        示例: DOI: 10.1038/s41586-020-2649-2 | arXiv: 2301.12345
      </div>
    </Modal>
  );
}
