export type GrapeResponse = {
  status: 'OK' | 'ERR';
  errmsg?: string;
  data?: unknown;
}

async function post(path: string, body?: unknown): Promise<GrapeResponse> {
  const res = await fetch(`/api${path}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: body ? JSON.stringify(body) : undefined,
  });
  if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
  return res.json();
}

export const api = {
  ping: () => fetch('/api/ping').then(r => r.json()),
  status: () => fetch('/api/status').then(r => r.json()),
  skip: () => post('/skip'),
  pause: () => post('/pause'),
  resume: () => post('/resume'),
  clear: () => post('/clear'),
  loopSong: () => post('/loop/song'),
  loopQueue: () => post('/loop/queue'),
  addQueue: (path: string) => post('/queue', { path }),
  addPlaylist: (path: string) => post('/playlist', { path }),
  shutdown: () => post('/shutdown'),
};

