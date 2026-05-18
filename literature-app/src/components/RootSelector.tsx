import { useState } from 'react';
import { Button, Typography } from 'antd';
import { FolderOpenOutlined } from '@ant-design/icons';
import { selectRootDir } from '../api/app';

interface Props {
  onSelected: () => void;
}

export default function RootSelector({ onSelected }: Props) {
  const [loading, setLoading] = useState(false);

  const handleSelect = async () => {
    setLoading(true);
    try {
      await selectRootDir();
      onSelected();
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        height: '100vh',
        background: 'var(--bg-deep)',
        gap: 24,
      }}
    >
      <div style={{ fontSize: 64, opacity: 0.3 }}>📚</div>
      <Typography.Title
        level={2}
        style={{
          fontFamily: 'var(--font-display)',
          color: 'var(--text-primary)',
          margin: 0,
        }}
      >
        Literature Integration
      </Typography.Title>
      <Typography.Text style={{ color: 'var(--text-muted)', fontSize: 15 }}>
        选择一个文件夹作为根目录，所有工作区和论文将存储在此
      </Typography.Text>
      <Button
        type="primary"
        size="large"
        icon={<FolderOpenOutlined />}
        loading={loading}
        onClick={handleSelect}
        style={{ marginTop: 8 }}
      >
        选择根目录
      </Button>
    </div>
  );
}
