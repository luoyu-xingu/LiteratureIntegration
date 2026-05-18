import { useState } from 'react';
import { Input, Button, message } from 'antd';
import { EditOutlined, SaveOutlined, CloseOutlined } from '@ant-design/icons';
import ReactMarkdown from 'react-markdown';
import { updatePaper } from '../api/paper';

interface Props {
  paperId: string;
  initialNotes: string;
}

export default function PaperNotes({ paperId, initialNotes }: Props) {
  const [notes, setNotes] = useState(initialNotes);
  const [editing, setEditing] = useState(false);
  const [saving, setSaving] = useState(false);

  const handleSave = async () => {
    setSaving(true);
    try {
      await updatePaper(paperId, { user_notes: notes });
      message.success('保存成功');
      setEditing(false);
    } catch {
      message.error('保存失败');
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="animate-fade-in">
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          marginBottom: 12,
        }}
      >
        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: 8,
            fontSize: 13,
            color: 'var(--text-secondary)',
            fontWeight: 500,
          }}
        >
          <EditOutlined style={{ color: 'var(--accent)' }} />
          我的笔记
        </div>
        {!editing && notes && (
          <Button
            type="text"
            size="small"
            icon={<EditOutlined />}
            onClick={() => setEditing(true)}
          >
            编辑
          </Button>
        )}
      </div>

      {editing ? (
        <div>
          <Input.TextArea
            rows={8}
            value={notes}
            onChange={(e) => setNotes(e.target.value)}
            placeholder="支持 Markdown 格式，例如：&#10;# 标题&#10;**粗体** *斜体*&#10;- 列表项&#10;> 引用"
            style={{ marginBottom: 12 }}
          />
          <div style={{ display: 'flex', gap: 8 }}>
            <Button
              type="primary"
              icon={<SaveOutlined />}
              loading={saving}
              onClick={handleSave}
              size="small"
            >
              保存
            </Button>
            <Button
              icon={<CloseOutlined />}
              onClick={() => {
                setEditing(false);
                setNotes(initialNotes);
              }}
              size="small"
            >
              取消
            </Button>
          </div>
        </div>
      ) : (
        <div
          onClick={() => setEditing(true)}
          style={{
            cursor: 'pointer',
            minHeight: 60,
            padding: notes ? 20 : 24,
            background: 'var(--bg-surface)',
            border: '1px solid var(--border)',
            borderRadius: 'var(--radius-md)',
            transition: 'all var(--transition)',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.borderColor = 'var(--border-light)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.borderColor = 'var(--border)';
          }}
        >
          {notes ? (
            <div className="markdown-body">
              <ReactMarkdown>{notes}</ReactMarkdown>
            </div>
          ) : (
            <div
              style={{
                color: 'var(--text-muted)',
                display: 'flex',
                alignItems: 'center',
                gap: 8,
                fontSize: 14,
              }}
            >
              <EditOutlined />
              点击添加笔记（支持 Markdown 格式）
            </div>
          )}
        </div>
      )}
    </div>
  );
}
