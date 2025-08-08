'use client';
import { useEffect, useState } from 'react';
import { api, GrapeResponse } from '@/lib/api';
import { Controls } from '@/components/Controls';

export default function Page() {
  const [daemon, setDaemon] = useState<string>('—');
  const [path, setPath] = useState('');
  const [msg, setMsg] = useState<string>('');

  useEffect(() => {
    api.ping().then(r => setDaemon((r.data as any)?.daemon ?? 'unknown')).catch(() => setDaemon('unreachable'));
  }, []);

  async function handle(res: Promise<GrapeResponse>) {
    const r = await res;
    setMsg(r.status === 'OK' ? 'OK' : (r.errmsg ?? 'ERR'));
    setTimeout(() => setMsg(''), 1500);
  }

  return (
    <main style={{ padding: 24, maxWidth: 720, margin: '0 auto' }}>
      <h1 style={{ marginBottom: 4 }}>Grapevine</h1>
      <p style={{ opacity: 0.7, marginBottom: 16 }}>Daemon: {daemon}</p>

      <div style={{ display: 'flex', gap: 8, marginBottom: 16 }}>
        <input
          placeholder="Absolute path to song or .m3u"
          value={path}
          onChange={e => setPath(e.target.value)}
          style={{ flex: 1, padding: 8, border: '1px solid #ddd', borderRadius: 8 }}
        />
        <button onClick={() => handle(api.addQueue(path))} disabled={!path}>Add Song</button>
        <button onClick={() => handle(api.addPlaylist(path))} disabled={!path}>Add Playlist</button>
      </div>

      <Controls onAction={p => handle(p)} />
      {!!msg && <div style={{ marginTop: 16, opacity: 0.9 }}>{msg}</div>}
    </main>
  );
}

