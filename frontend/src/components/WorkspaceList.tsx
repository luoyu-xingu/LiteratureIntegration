import { List, Button, Popconfirm, message } from 'antd';
import { DeleteOutlined } from '@ant-design/icons';
import { deleteWorkspace } from '../api/workspace';
import type { Workspace } from '../types';
import { useNavigate } from 'react-router-dom';
import { useState, useEffect } from 'react';
import { listWorkspaces } from '../api/workspace';

export default function WorkspaceList() {
  const [workspaces, setWorkspaces] = useState<Workspace[]>([]);
  const navigate = useNavigate();

  const load = async () => {
    try {
      const data = await listWorkspaces();
      setWorkspaces(data);
    } catch { }
  };

  useEffect(() => { load(); }, []);

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

  return (
    <List
      dataSource={workspaces}
      renderItem={(ws) => (
        <List.Item
          actions={[
            <Popconfirm title="确定删除？" onConfirm={() => handleDelete(ws.id)}>
              <Button type="text" danger icon={<DeleteOutlined />} />
            </Popconfirm>,
          ]}
        >
          <List.Item.Meta
            title={<a onClick={() => navigate(`/workspace/${ws.id}`)}>{ws.name}</a>}
            description={ws.description}
          />
        </List.Item>
      )}
    />
  );
}
