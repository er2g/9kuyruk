import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { Toaster } from 'react-hot-toast';
import Editor from './pages/Editor';
import Projects from './pages/Projects';
import Templates from './pages/Templates';
import Automation from './pages/Automation';
import Login from './pages/Login';
import { useStore } from './store';

function App() {
  const { isAuthenticated } = useStore();

  if (!isAuthenticated) {
    return (
      <>
        <Toaster position="top-right" />
        <Routes>
          <Route path="/login" element={<Login />} />
          <Route path="*" element={<Navigate to="/login" replace />} />
        </Routes>
      </>
    );
  }

  return (
    <>
      <Toaster position="top-right" />
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Projects />} />
          <Route path="/editor/:projectId" element={<Editor />} />
          <Route path="/templates" element={<Templates />} />
          <Route path="/automation" element={<Automation />} />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </BrowserRouter>
    </>
  );
}

export default App;
