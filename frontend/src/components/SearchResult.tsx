import { List, Tag } from 'antd';
import type { Paper, AuthorWithPapers } from '../types';
import { useNavigate } from 'react-router-dom';

interface Props {
  results: Paper[] | AuthorWithPapers[] | null;
  mode: 'keyword' | 'author';
}

export default function SearchResult({ results, mode }: Props) {
  const navigate = useNavigate();

  if (!results) return null;

  if (mode === 'keyword') {
    const papers = results as Paper[];
    return (
      <List
        dataSource={papers}
        renderItem={(paper) => (
          <List.Item style={{ cursor: 'pointer' }} onClick={() => navigate(`/paper/${paper.id}`)}>
            <List.Item.Meta
              title={paper.title}
              description={`${paper.year} · ${paper.journal}`}
            />
          </List.Item>
        )}
      />
    );
  }

  const authorResults = results as AuthorWithPapers[];
  return (
    <List
      dataSource={authorResults}
      renderItem={(item) => (
        <List.Item>
          <List.Item.Meta
            title={item.author.name}
            description={`${item.papers.length} 篇论文`}
          />
          <div>
            {item.papers.map((p) => (
              <Tag key={p.id} style={{ cursor: 'pointer' }} onClick={() => navigate(`/paper/${p.id}`)}>
                {p.title}
              </Tag>
            ))}
          </div>
        </List.Item>
      )}
    />
  );
}
