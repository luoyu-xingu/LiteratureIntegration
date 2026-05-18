import WorkspaceList from '../components/WorkspaceList';

export default function WorkspacesPage() {
  return (
    <div className="animate-fade-in">
      <h2
        style={{
          fontFamily: 'var(--font-display)',
          fontSize: 28,
          fontWeight: 700,
          marginBottom: 24,
          color: 'var(--text-primary)',
        }}
      >
        我的工作区
      </h2>
      <WorkspaceList />
    </div>
  );
}
