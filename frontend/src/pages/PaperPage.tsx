import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { Spin } from 'antd';
import PaperDetailComponent from '../components/PaperDetail';
import PaperNotes from '../components/PaperNotes';
import { getPaper } from '../api/paper';
import type { PaperDetail as PaperDetailType } from '../types';

export default function PaperPage() {
  const { id } = useParams<{ id: string }>();
  const [detail, setDetail] = useState<PaperDetailType | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!id) return;
    setLoading(true);
    getPaper(id).then(setDetail).finally(() => setLoading(false));
  }, [id]);

  if (loading) return <Spin />;
  if (!detail) return <div>论文未找到</div>;

  return (
    <div>
      <PaperDetailComponent detail={detail} />
      <div style={{ marginTop: 24 }}>
        <PaperNotes paperId={detail.paper.id} initialNotes={detail.paper.user_notes || ''} />
      </div>
    </div>
  );
}
