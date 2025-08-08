'use client';
import { api, GrapeResponse } from '@/lib/api';

export function Controls({ onAction }: { onAction: (p: Promise<GrapeResponse>) => void }) {
  const Btn = (props: React.ButtonHTMLAttributes<HTMLButtonElement>) => (
    <button {...props} style={{ padding: 10, borderRadius: 8, border: '1px solid #ddd' }} />
  );
  const Grid = (props: React.HTMLAttributes<HTMLDivElement>) => (
    <div {...props} style={{ display: 'grid', gap: 10, gridTemplateColumns: 'repeat(3, minmax(0,1fr))' }} />
  );
  return (
    <Grid>
      <Btn onClick={() => onAction(api.skip())}>Skip</Btn>
      <Btn onClick={() => onAction(api.pause())}>Pause</Btn>
      <Btn onClick={() => onAction(api.resume())}>Resume</Btn>
      <Btn onClick={() => onAction(api.clear())}>Clear</Btn>
      <Btn onClick={() => onAction(api.loopSong())}>Loop Song</Btn>
      <Btn onClick={() => onAction(api.loopQueue())}>Loop Queue</Btn>
      <Btn onClick={() => onAction(api.shutdown())}>Shutdown</Btn>
    </Grid>
  );
}

