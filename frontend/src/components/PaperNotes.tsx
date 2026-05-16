import { useState } from 'react';
import { Input, Button, message } from 'antd';
import { EditOutlined } from '@ant-design/icons';
import ReactMarkdown from 'react-markdown';
import { updatePaper } from '../api/paper';

interface Props {
  paperId: string;
  initialNotes: string;
}

const markdownStyles: React.CSSProperties = {
  lineHeight: 1.7,
  color: '#333',
};

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
    <div>
      <div style={{ marginBottom: 8, fontWeight: 'bold' }}>我的笔记:</div>
      {editing ? (
        <>
          <Input.TextArea
            rows={8}
            value={notes}
            onChange={(e) => setNotes(e.target.value)}
            placeholder="支持 Markdown 格式，例如：&#10;# 标题&#10;**粗体** *斜体*&#10;- 列表项&#10;> 引用"
          />
          <div style={{ marginTop: 8 }}>
            <Button type="primary" loading={saving} onClick={handleSave}>保存</Button>
            <Button style={{ marginLeft: 8 }} onClick={() => { setEditing(false); setNotes(initialNotes); }}>取消</Button>
          </div>
        </>
      ) : (
        <div
          style={{
            cursor: 'pointer',
            minHeight: 40,
            padding: 16,
            background: '#fafafa',
            borderRadius: 8,
            border: '1px solid #f0f0f0',
            position: 'relative',
          }}
          onClick={() => setEditing(true)}
        >
          {notes ? (
            <div style={markdownStyles} className="markdown-body">
              <ReactMarkdown>{notes}</ReactMarkdown>
            </div>
          ) : (
            <div style={{ color: '#bbb', display: 'flex', alignItems: 'center', gap: 6 }}>
              <EditOutlined />
              点击添加笔记（支持 Markdown 格式）
            </div>
          )}
        </div>
      )}
    </div>
  );
}
