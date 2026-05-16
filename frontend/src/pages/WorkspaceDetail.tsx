import { useState } from 'react';
import { useParams } from 'react-router-dom';
import { Tabs, Button, Space } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import PaperList from '../components/PaperList';
import PaperImport from '../components/PaperImport';
import AuthorGraph from '../components/AuthorGraph';
import { usePapers } from '../hooks/usePapers';

export default function WorkspaceDetail() {
  const { id } = useParams<{ id: string }>();
  const { papers, loading, reload } = usePapers(id);
  const [showImport, setShowImport] = useState(false);
  const [activeTab, setActiveTab] = useState('papers');

  return (
    <div>
      <div style={{ marginBottom: 16, display: 'flex', justifyContent: 'space-between' }}>
        <h2 style={{ margin: 0 }}>工作区详情</h2>
        <Space>
          <Button type="primary" icon={<PlusOutlined />} onClick={() => setShowImport(true)}>
            导入论文
          </Button>
        </Space>
      </div>

      <Tabs activeKey={activeTab} onChange={setActiveTab} items={[
        { key: 'papers', label: '论文列表', children: <PaperList papers={papers} loading={loading} /> },
        { key: 'graph', label: '作者网络图', children: id ? <AuthorGraph workspaceId={id} /> : null },
      ]} />

      {id && (
        <PaperImport
          workspaceId={id}
          open={showImport}
          onClose={() => setShowImport(false)}
          onImported={reload}
        />
      )}
    </div>
  );
}
