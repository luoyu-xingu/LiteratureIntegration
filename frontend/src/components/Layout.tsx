import { useState } from 'react';
import { Layout as AntLayout, Menu, Button } from 'antd';
import { BookOutlined, PlusOutlined, SearchOutlined } from '@ant-design/icons';
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
      <Sider
        width={260}
        style={{
          background: 'var(--bg-base)',
          borderRight: '1px solid var(--border)',
          display: 'flex',
          flexDirection: 'column',
        }}
      >
        <div
          style={{
            padding: '20px 20px 16px',
            borderBottom: '1px solid var(--border)',
          }}
        >
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: 10,
            }}
          >
            <div
              style={{
                width: 32,
                height: 32,
                borderRadius: 8,
                background: 'linear-gradient(135deg, var(--accent), #8b6914)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                fontSize: 16,
                color: '#0c0c18',
                flexShrink: 0,
              }}
            >
              <BookOutlined />
            </div>
            <div>
              <div
                style={{
                  fontFamily: 'var(--font-display)',
                  fontSize: 18,
                  fontWeight: 700,
                  color: 'var(--text-primary)',
                  lineHeight: 1.2,
                }}
              >
                Literature
              </div>
              <div
                style={{
                  fontSize: 11,
                  color: 'var(--text-muted)',
                  letterSpacing: '0.1em',
                  textTransform: 'uppercase',
                }}
              >
                Integration
              </div>
            </div>
          </div>
        </div>

        <div style={{ flex: 1, overflow: 'auto', padding: '8px 0' }}>
          <div
            style={{
              padding: '0 16px 8px',
              fontSize: 11,
              color: 'var(--text-muted)',
              letterSpacing: '0.08em',
              textTransform: 'uppercase',
              fontWeight: 600,
            }}
          >
            工作区
          </div>
          <Menu
            mode="inline"
            selectedKeys={workspaceId ? [workspaceId] : []}
            onClick={({ key }) => navigate(`/workspace/${key}`)}
            items={menuItems}
            style={{ border: 'none' }}
          />
        </div>

        <div style={{ padding: '12px 16px', borderTop: '1px solid var(--border)' }}>
          <Button
            type="dashed"
            icon={<PlusOutlined />}
            block
            onClick={() => setShowForm(true)}
          >
            新建工作区
          </Button>
        </div>
      </Sider>

      <Content
        style={{
          background: 'var(--bg-deep)',
          overflow: 'auto',
          position: 'relative',
        }}
      >
        <div
          style={{
            position: 'absolute',
            top: 16,
            right: 24,
            zIndex: 10,
          }}
        >
          {workspaceId && (
            <Button
              type="text"
              icon={<SearchOutlined />}
              onClick={() => navigate(`/workspace/${workspaceId}/search`)}
              style={{
                color: 'var(--text-muted)',
                fontSize: 16,
              }}
            />
          )}
        </div>
        <div style={{ padding: '32px 40px', maxWidth: 1200 }}>
          <Outlet />
        </div>
      </Content>

      <WorkspaceForm
        open={showForm}
        onClose={() => setShowForm(false)}
        onCreated={reload}
      />
    </AntLayout>
  );
}
