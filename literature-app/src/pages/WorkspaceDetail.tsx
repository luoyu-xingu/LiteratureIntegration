import { useState } from 'react';
import { useParams } from 'react-router-dom';
import { Tabs, Button, Space } from 'antd';
import { PlusOutlined, FileTextOutlined, ApartmentOutlined } from '@ant-design/icons';
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
    <div className="animate-fade-in">
      <div
        style={{
          marginBottom: 24,
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
        }}
      >
        <h2
          style={{
            fontFamily: 'var(--font-display)',
            fontSize: 28,
            fontWeight: 700,
            margin: 0,
          }}
        >
          工作区详情
        </h2>
        <Space>
          <Button
            type="primary"
            icon={<PlusOutlined />}
            onClick={() => setShowImport(true)}
          >
            导入论文
          </Button>
        </Space>
      </div>

      <Tabs
        activeKey={activeTab}
        onChange={setActiveTab}
        items={[
          {
            key: 'papers',
            label: (
              <span style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                <FileTextOutlined />
                论文列表
              </span>
            ),
            children: <PaperList papers={papers} loading={loading} />,
          },
          {
            key: 'graph',
            label: (
              <span style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                <ApartmentOutlined />
                作者网络图
              </span>
            ),
            children: id ? <AuthorGraph workspaceId={id} /> : null,
          },
        ]}
      />

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
