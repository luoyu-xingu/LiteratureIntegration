import { List, Tag } from 'antd';
import type { Paper } from '../types';
import { useNavigate } from 'react-router-dom';

interface Props {
  papers: Paper[];
  loading: boolean;
}

export default function PaperList({ papers, loading }: Props) {
  const navigate = useNavigate();

  return (
    <List
      loading={loading}
      dataSource={papers}
      renderItem={(paper) => (
        <List.Item
          style={{ cursor: 'pointer' }}
          onClick={() => navigate(`/paper/${paper.id}`)}
        >
          <List.Item.Meta
            title={paper.title}
            description={
              <div>
                <div>{paper.year} · {paper.journal}</div>
                {paper.doi && <Tag>DOI: {paper.doi}</Tag>}
                {paper.arxiv_id && <Tag>arXiv: {paper.arxiv_id}</Tag>}
              </div>
            }
          />
        </List.Item>
      )}
    />
  );
}
