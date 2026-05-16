import { Descriptions, Tag } from 'antd';
import ReactMarkdown from 'react-markdown';
import type { PaperDetail as PaperDetailType } from '../types';

interface Props {
  detail: PaperDetailType;
}

export default function PaperDetail({ detail }: Props) {
  const { paper, first_author, corresponding_author, keywords } = detail;

  return (
    <Descriptions bordered column={1}>
      <Descriptions.Item label="标题">{paper.title}</Descriptions.Item>
      <Descriptions.Item label="年份">{paper.year}</Descriptions.Item>
      <Descriptions.Item label="期刊">{paper.journal}</Descriptions.Item>
      <Descriptions.Item label="DOI">{paper.doi}</Descriptions.Item>
      <Descriptions.Item label="arXiv">{paper.arxiv_id}</Descriptions.Item>
      <Descriptions.Item label="一作">{first_author?.name}</Descriptions.Item>
      <Descriptions.Item label="通讯作者">{corresponding_author?.name}</Descriptions.Item>
      <Descriptions.Item label="关键词">
        {keywords.map((k) => <Tag key={k.id}>{k.name}</Tag>)}
      </Descriptions.Item>
      <Descriptions.Item label="Abstract">
        {paper.abstract_text ? (
          <div className="markdown-body" style={{ lineHeight: 1.7 }}>
            <ReactMarkdown>{paper.abstract_text}</ReactMarkdown>
          </div>
        ) : (
          '暂无'
        )}
      </Descriptions.Item>
    </Descriptions>
  );
}
