import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Layout from './components/Layout';
import WorkspacesPage from './pages/WorkspacesPage';
import WorkspaceDetail from './pages/WorkspaceDetail';
import PaperPage from './pages/PaperPage';
import SearchPage from './pages/SearchPage';

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<Layout />}>
          <Route path="/" element={<WorkspacesPage />} />
          <Route path="/workspace/:id" element={<WorkspaceDetail />} />
          <Route path="/paper/:id" element={<PaperPage />} />
          <Route path="/workspace/:id/search" element={<SearchPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
