import { useCallback, useState } from 'react';
import ForceGraph2D from 'react-force-graph-2d';
import { Card, List, Spin, Drawer } from 'antd';
import { useGraph } from '../hooks/useGraph';
import { getAuthorPapers } from '../api/author';
import type { Paper, GraphNode } from '../types';

interface Props {
  workspaceId: string;
}

export default function AuthorGraph({ workspaceId }: Props) {
  const { data, loading } = useGraph(workspaceId);
  const [selectedAuthor, setSelectedAuthor] = useState<{ id: string; name: string } | null>(null);
  const [authorPapers, setAuthorPapers] = useState<Paper[]>([]);
  const [drawerOpen, setDrawerOpen] = useState(false);

  const handleClickNode = useCallback(async (node: any) => {
    const graphNode = node as GraphNode;
    setSelectedAuthor({ id: graphNode.id, name: graphNode.name });
    try {
      const result = await getAuthorPapers(graphNode.id);
      setAuthorPapers(result.papers);
    } catch { }
    setDrawerOpen(true);
  }, []);

  const graphData = data ? {
    nodes: data.nodes.map(n => ({ ...n, val: Math.max(n.paper_count, 1) })),
    links: data.links,
  } : { nodes: [], links: [] };

  if (loading) return <Spin />;

  return (
    <Card>
      <div style={{ marginBottom: 8, fontSize: 12, color: '#999' }}>
        ● 一作 &nbsp; ○ 通讯作者 &nbsp; ◎ 两者兼有 &nbsp; 线条粗细 = 合著论文数
      </div>
      <ForceGraph2D
        graphData={graphData}
        nodeLabel="name"
        nodeVal="val"
        nodeColor={(node: any) => {
          const n = node as GraphNode;
          if (n.author_type === 'both') return '#722ed1';
          if (n.author_type === 'first') return '#1890ff';
          return '#52c41a';
        }}
        nodeCanvasObject={(node: any, ctx: any, globalScale: number) => {
          const label = node.name;
          const fontSize = 12 / globalScale;
          ctx.font = `${fontSize}px Sans-Serif`;
          ctx.textAlign = 'center';
          ctx.textBaseline = 'middle';

          const n = node as GraphNode;
          const radius = 4 + n.paper_count * 1.5;

          ctx.beginPath();
          ctx.arc(node.x, node.y, radius, 0, 2 * Math.PI);
          if (n.author_type === 'corresponding') {
            ctx.strokeStyle = '#52c41a';
            ctx.lineWidth = 2 / globalScale;
            ctx.stroke();
          } else {
            ctx.fillStyle = n.author_type === 'both' ? '#722ed1' : '#1890ff';
            ctx.fill();
          }

          ctx.fillStyle = '#333';
          ctx.fillText(label, node.x, node.y + radius + fontSize);
        }}
        linkWidth={(link: any) => Math.max(link.paper_count * 0.5, 0.5)}
        linkColor={() => '#ccc'}
        onNodeClick={handleClickNode}
        width={800}
        height={500}
      />
      <Drawer
        title={selectedAuthor?.name}
        open={drawerOpen}
        onClose={() => setDrawerOpen(false)}
      >
        <List
          dataSource={authorPapers}
          renderItem={(paper) => (
            <List.Item>
              <List.Item.Meta title={paper.title} description={`${paper.year} · ${paper.journal}`} />
            </List.Item>
          )}
        />
      </Drawer>
    </Card>
  );
}
