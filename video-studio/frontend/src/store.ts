import { create } from 'zustand';

interface User {
  id: string;
  email: string;
}

interface Project {
  id: string;
  name: string;
  description?: string;
  created_at: string;
  updated_at: string;
}

interface Asset {
  id: string;
  filename: string;
  asset_type: string;
  file_path: string;
  duration?: number;
  width?: number;
  height?: number;
}

interface Store {
  // Auth
  isAuthenticated: boolean;
  user: User | null;
  token: string | null;
  setAuth: (token: string, user: User) => void;
  logout: () => void;

  // Projects
  projects: Project[];
  currentProject: Project | null;
  setProjects: (projects: Project[]) => void;
  setCurrentProject: (project: Project | null) => void;

  // Assets
  assets: Asset[];
  setAssets: (assets: Asset[]) => void;

  // Timeline
  timeline: any;
  setTimeline: (timeline: any) => void;
}

export const useStore = create<Store>((set) => ({
  // Auth
  isAuthenticated: !!localStorage.getItem('token'),
  user: null,
  token: localStorage.getItem('token'),
  setAuth: (token, user) => {
    localStorage.setItem('token', token);
    set({ isAuthenticated: true, user, token });
  },
  logout: () => {
    localStorage.removeItem('token');
    set({ isAuthenticated: false, user: null, token: null });
  },

  // Projects
  projects: [],
  currentProject: null,
  setProjects: (projects) => set({ projects }),
  setCurrentProject: (project) => set({ currentProject: project }),

  // Assets
  assets: [],
  setAssets: (assets) => set({ assets }),

  // Timeline
  timeline: { layers: [], duration: 60 },
  setTimeline: (timeline) => set({ timeline }),
}));
