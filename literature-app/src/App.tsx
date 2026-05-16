import { useState, useEffect } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { getRootDir } from './api/app';
import Layout from './components/Layout';
import RootSelector from './components/RootSelector';
import WorkspacesPage from './pages/WorkspacesPage';
import WorkspaceDetail from './pages/WorkspaceDetail';
import PaperPage from './pages/PaperPage';
import SearchPage from './pages/SearchPage';
import './styles/global.css';

export default function App() {
  const [hasRoot, setHasRoot] = useState(false);
  const [checking, setChecking] = useState(true);

  useEffect(() => {
    getRootDir()
      .then(() => setHasRoot(true))
      .catch(() => setHasRoot(false))
      .finally(() => setChecking(false));
  }, []);

  if (checking) return null;

  if (!hasRoot) {
    return <RootSelector onSelected={() => setHasRoot(true)} />;
  }

  return (
    <BrowserRouter>
      <Routes>
        <Route element={<Layout />}>
          <Route path="/" element={<WorkspacesPage />} />
          <Route path="/workspace/:id" element={<WorkspaceDetail />} />
          <Route path="/workspace/:id/search" element={<SearchPage />} />
          <Route path="/paper/:id" element={<PaperPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
