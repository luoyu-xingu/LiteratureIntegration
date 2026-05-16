import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { Spin, Button } from 'antd';
import { ArrowLeftOutlined } from '@ant-design/icons';
import PaperDetailComponent from '../components/PaperDetail';
import PaperNotes from '../components/PaperNotes';
import { getPaper } from '../api/paper';
import type { PaperDetail as PaperDetailType } from '../types';
import { useNavigate } from 'react-router-dom';

export default function PaperPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [detail, setDetail] = useState<PaperDetailType | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!id) return;
    setLoading(true);
    getPaper(id).then(setDetail).finally(() => setLoading(false));
  }, [id]);

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: 80 }}>
        <Spin />
      </div>
    );
  }

  if (!detail) {
    return (
      <div
        className="animate-fade-in"
        style={{
          textAlign: 'center',
          padding: 80,
          color: 'var(--text-muted)',
        }}
      >
        论文未找到
      </div>
    );
  }

  return (
    <div className="animate-fade-in">
      <Button
        type="text"
        icon={<ArrowLeftOutlined />}
        onClick={() => navigate(-1)}
        style={{
          marginBottom: 20,
          color: 'var(--text-secondary)',
          padding: '4px 0',
        }}
      >
        返回
      </Button>
      <PaperDetailComponent detail={detail} />
      <div style={{ marginTop: 32 }}>
        <PaperNotes
          paperId={detail.paper.id}
          initialNotes={detail.user_notes || ''}
        />
      </div>
    </div>
  );
}
