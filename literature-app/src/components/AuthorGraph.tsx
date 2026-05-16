import { useCallback, useState } from 'react';
import ForceGraph2D from 'react-force-graph-2d';
import { List, Spin, Drawer } from 'antd';
import { useGraph } from '../hooks/useGraph';
import { getAuthorPapers } from '../api/author';
import type { Paper, GraphNode } from '../types';

interface Props {
  workspaceId: string;
}

export default function AuthorGraph({ workspaceId }: Props) {
  const { data, loading } = useGraph(workspaceId);
  const [selectedAuthor, setSelectedAuthor] = useState<{ name: string } | null>(null);
  const [authorPapers, setAuthorPapers] = useState<Paper[]>([]);
  const [drawerOpen, setDrawerOpen] = useState(false);

  const handleClickNode = useCallback(async (node: any) => {
    const graphNode = node as GraphNode;
    setSelectedAuthor({ name: graphNode.name });
    try {
      const result = await getAuthorPapers(graphNode.name);
      setAuthorPapers(result.papers);
    } catch {}
    setDrawerOpen(true);
  }, []);

  const graphData = data
    ? {
        nodes: data.nodes.map((n) => ({ ...n, val: Math.max(n.paper_count, 1) })),
        links: data.links,
      }
    : { nodes: [], links: [] };

  if (loading) {
    return (
      <div style={{ textAlign: 'center', padding: 60 }}>
        <Spin />
      </div>
    );
  }

  if (!data || data.nodes.length === 0) {
    return (
      <div
        className="animate-fade-in"
        style={{
          textAlign: 'center',
          padding: '60px 0',
          color: 'var(--text-muted)',
        }}
      >
        <div style={{ fontSize: 36, marginBottom: 12, opacity: 0.3 }}>🕸️</div>
        <div style={{ fontSize: 15 }}>导入论文后可查看作者关系网络</div>
      </div>
    );
  }

  return (
    <div className="animate-fade-in">
      <div
        style={{
          marginBottom: 12,
          display: 'flex',
          gap: 16,
          fontSize: 12,
          color: 'var(--text-muted)',
        }}
      >
        <span>
          <span
            style={{
              display: 'inline-block',
              width: 10,
              height: 10,
              borderRadius: '50%',
              background: '#c9a227',
              marginRight: 4,
            }}
          />
          一作
        </span>
        <span>
          <span
            style={{
              display: 'inline-block',
              width: 10,
              height: 10,
              borderRadius: '50%',
              border: '2px solid #4ade80',
              marginRight: 4,
            }}
          />
          通讯作者
        </span>
        <span>
          <span
            style={{
              display: 'inline-block',
              width: 10,
              height: 10,
              borderRadius: '50%',
              background: '#a855f7',
              marginRight: 4,
            }}
          />
          两者兼有
        </span>
      </div>

      <div
        style={{
          background: 'var(--bg-surface)',
          border: '1px solid var(--border)',
          borderRadius: 'var(--radius-md)',
          overflow: 'hidden',
        }}
      >
        <ForceGraph2D
          graphData={graphData}
          nodeLabel="name"
          nodeVal="val"
          nodeColor={(node: any) => {
            const n = node as GraphNode;
            if (n.author_type === 'both') return '#a855f7';
            if (n.author_type === 'first') return '#c9a227';
            return '#4ade80';
          }}
          nodeCanvasObject={(node: any, ctx: any, globalScale: number) => {
            const label = node.name;
            const fontSize = 12 / globalScale;
            ctx.font = `${fontSize}px Source Sans 3, sans-serif`;
            ctx.textAlign = 'center';
            ctx.textBaseline = 'middle';

            const n = node as GraphNode;
            const radius = 4 + n.paper_count * 1.5;

            ctx.beginPath();
            ctx.arc(node.x, node.y, radius, 0, 2 * Math.PI);
            if (n.author_type === 'corresponding') {
              ctx.strokeStyle = '#4ade80';
              ctx.lineWidth = 2 / globalScale;
              ctx.stroke();
            } else {
              ctx.fillStyle = n.author_type === 'both' ? '#a855f7' : '#c9a227';
              ctx.fill();
            }

            ctx.fillStyle = '#e8e5e0';
            ctx.fillText(label, node.x, node.y + radius + fontSize);
          }}
          linkWidth={(link: any) => Math.max(link.paper_count * 0.5, 0.5)}
          linkColor={() => 'rgba(46, 46, 82, 0.8)'}
          onNodeClick={handleClickNode}
          width={800}
          height={500}
          backgroundColor="#111125"
        />
      </div>

      <Drawer
        title={
          <span style={{ fontFamily: 'var(--font-display)', fontSize: 18 }}>
            {selectedAuthor?.name}
          </span>
        }
        open={drawerOpen}
        onClose={() => setDrawerOpen(false)}
      >
        <List
          dataSource={authorPapers}
          renderItem={(paper) => (
            <List.Item>
              <List.Item.Meta
                title={paper.title}
                description={`${paper.year} · ${paper.journal || '—'}`}
              />
            </List.Item>
          )}
        />
      </Drawer>
    </div>
  );
}
