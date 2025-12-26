import axios from 'axios';

const api = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add token to requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Auth
export const auth = {
  login: (email: string, password: string) =>
    api.post('/auth/login', { email, password }),
  register: (email: string, password: string) =>
    api.post('/auth/register', { email, password }),
};

// Projects
export const projects = {
  list: () => api.get('/projects'),
  create: (name: string, description?: string) =>
    api.post('/projects', { name, description }),
  get: (id: string) => api.get(`/projects/${id}`),
  update: (id: string, data: any) => api.put(`/projects/${id}`, data),
  delete: (id: string) => api.delete(`/projects/${id}`),
};

// Assets
export const assets = {
  list: (projectId: string) => api.get(`/assets?project_id=${projectId}`),
  upload: (projectId: string, file: File) => {
    const formData = new FormData();
    formData.append('project_id', projectId);
    formData.append('file', file);
    return api.post('/assets/upload', formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });
  },
  delete: (id: string) => api.delete(`/assets/${id}`),
};

// Compositions
export const compositions = {
  create: (projectId: string, name: string, timeline: any) =>
    api.post('/compositions', { project_id: projectId, name, timeline }),
  get: (id: string) => api.get(`/compositions/${id}`),
  update: (id: string, data: any) => api.put(`/compositions/${id}`, data),
};

// Render
export const render = {
  start: (projectId: string, compositionData: any) =>
    api.post('/render', { project_id: projectId, composition_data: compositionData }),
  status: (jobId: string) => api.get(`/render/${jobId}/status`),
};

// Templates
export const templates = {
  list: () => api.get('/templates'),
  create: (name: string, description: string, timelineTemplate: any) =>
    api.post('/templates', { name, description, timeline_template: timelineTemplate }),
  apply: (id: string, projectId: string, variables: any) =>
    api.post(`/templates/${id}/apply`, { project_id: projectId, variables }),
};

// AI Tools
export const aiTools = {
  // Auto-distribute overlay images across video duration
  autoDistributeOverlays: (images: File[], videoDuration: number) => {
    const formData = new FormData();
    formData.append('video_duration', videoDuration.toString());
    images.forEach((img) => formData.append('image', img));
    return api.post('/ai/overlay/auto-distribute', formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });
  },

  // Align text to audio (forced alignment with Aeneas)
  alignSubtitles: (text: string, audio: File, language: string) => {
    const formData = new FormData();
    formData.append('text', text);
    formData.append('audio', audio);
    formData.append('language', language); // "tur" or "eng"
    return api.post('/ai/subtitle/align', formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });
  },

  // Auto-transcribe audio using Whisper
  transcribeAudio: (audio: File, language?: string) => {
    const formData = new FormData();
    formData.append('audio', audio);
    if (language) formData.append('language', language);
    return api.post('/ai/subtitle/transcribe', formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });
  },
};

export default api;
