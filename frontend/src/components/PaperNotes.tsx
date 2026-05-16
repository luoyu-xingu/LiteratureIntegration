import { useState } from 'react';
import { Input, Button, message } from 'antd';
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
    <div>
      <div style={{ marginBottom: 8, fontWeight: 'bold' }}>我的笔记:</div>
      {editing ? (
        <>
          <Input.TextArea rows={4} value={notes} onChange={(e) => setNotes(e.target.value)} />
          <div style={{ marginTop: 8 }}>
            <Button type="primary" loading={saving} onClick={handleSave}>保存</Button>
            <Button style={{ marginLeft: 8 }} onClick={() => setEditing(false)}>取消</Button>
          </div>
        </>
      ) : (
        <div
          style={{ cursor: 'pointer', minHeight: 40, padding: 8, background: '#fafafa', borderRadius: 4 }}
          onClick={() => setEditing(true)}
        >
          {notes || '点击添加笔记...'}
        </div>
      )}
    </div>
  );
}
