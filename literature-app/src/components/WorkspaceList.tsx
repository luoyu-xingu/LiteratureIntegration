import { useState, useEffect } from 'react';
import { Button, Popconfirm, message } from 'antd';
import { DeleteOutlined, RightOutlined } from '@ant-design/icons';
import { deleteWorkspace, listWorkspaces } from '../api/workspace';
import type { Workspace } from '../types';
import { useNavigate } from 'react-router-dom';

export default function WorkspaceList() {
  const [workspaces, setWorkspaces] = useState<Workspace[]>([]);
  const navigate = useNavigate();

  const load = async () => {
    try {
      const data = await listWorkspaces();
      setWorkspaces(data);
    } catch {}
  };

  useEffect(() => {
    load();
  }, []);

  const handleDelete = async (id: string) => {
    try {
      await deleteWorkspace(id);
      message.success('删除成功');
      load();
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : '删除失败';
      message.error(msg);
    }
  };

  if (workspaces.length === 0) {
    return (
      <div
        className="animate-fade-in"
        style={{
          textAlign: 'center',
          padding: '80px 0',
          color: 'var(--text-muted)',
        }}
      >
        <div style={{ fontSize: 48, marginBottom: 16, opacity: 0.3 }}>📚</div>
        <div
          style={{
            fontFamily: 'var(--font-display)',
            fontSize: 24,
            color: 'var(--text-secondary)',
            marginBottom: 8,
          }}
        >
          尚无工作区
        </div>
        <div style={{ fontSize: 14 }}>
          在左侧点击「新建工作区」开始你的文献管理
        </div>
      </div>
    );
  }

  return (
    <div style={{ display: 'grid', gap: 12 }}>
      {workspaces.map((ws, i) => (
        <div
          key={ws.id}
          className="animate-fade-in"
          style={{
            animationDelay: `${i * 0.05}s`,
            background: 'var(--bg-surface)',
            border: '1px solid var(--border)',
            borderRadius: 'var(--radius-md)',
            padding: '16px 20px',
            display: 'flex',
            alignItems: 'center',
            gap: 16,
            cursor: 'pointer',
            transition: 'all var(--transition)',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.borderColor = 'var(--accent)';
            e.currentTarget.style.boxShadow = '0 0 16px var(--accent-dim)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.borderColor = 'var(--border)';
            e.currentTarget.style.boxShadow = 'none';
          }}
          onClick={() => navigate(`/workspace/${ws.id}`)}
        >
          <div
            style={{
              width: 40,
              height: 40,
              borderRadius: 10,
              background: 'var(--accent-dim)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              fontFamily: 'var(--font-display)',
              fontSize: 18,
              fontWeight: 700,
              color: 'var(--accent)',
              flexShrink: 0,
            }}
          >
            {ws.name.charAt(0).toUpperCase()}
          </div>
          <div style={{ flex: 1, minWidth: 0 }}>
            <div
              style={{
                fontSize: 15,
                fontWeight: 600,
                color: 'var(--text-primary)',
                marginBottom: 2,
              }}
            >
              {ws.name}
            </div>
            {ws.description && (
              <div
                style={{
                  fontSize: 13,
                  color: 'var(--text-muted)',
                  overflow: 'hidden',
                  textOverflow: 'ellipsis',
                  whiteSpace: 'nowrap',
                }}
              >
                {ws.description}
              </div>
            )}
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <Popconfirm
              title="确定删除此工作区？"
              onConfirm={(e) => {
                e?.stopPropagation();
                handleDelete(ws.id);
              }}
            >
              <Button
                type="text"
                danger
                icon={<DeleteOutlined />}
                size="small"
                onClick={(e) => e.stopPropagation()}
              />
            </Popconfirm>
            <RightOutlined
              style={{ color: 'var(--text-muted)', fontSize: 12 }}
            />
          </div>
        </div>
      ))}
    </div>
  );
}
