import { useState } from 'react';
import { Layout as AntLayout, Menu, Button } from 'antd';
import { BookOutlined, PlusOutlined } from '@ant-design/icons';
import { useNavigate, Outlet, useLocation } from 'react-router-dom';
import WorkspaceForm from './WorkspaceForm';
import { useWorkspaces } from '../hooks/useWorkspaces';

const { Sider, Content } = AntLayout;

export default function Layout() {
  const navigate = useNavigate();
  const location = useLocation();
  const [showForm, setShowForm] = useState(false);
  const { workspaces, reload } = useWorkspaces();

  const workspaceId = location.pathname.startsWith('/workspace/')
    ? location.pathname.split('/')[2]
    : undefined;

  const menuItems = workspaces.map((ws) => ({
    key: ws.id,
    label: ws.name,
  }));

  return (
    <AntLayout style={{ height: '100vh' }}>
      <Sider width={250} style={{ background: '#fff', borderRight: '1px solid #f0f0f0' }}>
        <div style={{ padding: '16px', borderBottom: '1px solid #f0f0f0' }}>
          <h2 style={{ margin: 0, fontSize: '16px' }}>
            <BookOutlined /> LiteratureIntegration
          </h2>
        </div>
        <Menu
          mode="inline"
          selectedKeys={workspaceId ? [workspaceId] : []}
          onClick={({ key }) => navigate(`/workspace/${key}`)}
          items={menuItems}
        />
        <div style={{ padding: '12px 16px' }}>
          <Button type="dashed" icon={<PlusOutlined />} block onClick={() => setShowForm(true)}>
            新建工作区
          </Button>
        </div>
      </Sider>
      <Content style={{ padding: '24px', overflow: 'auto' }}>
        <Outlet />
      </Content>
      <WorkspaceForm
        open={showForm}
        onClose={() => setShowForm(false)}
        onCreated={reload}
      />
    </AntLayout>
  );
}
