"use client";

import { api, GrapeResponse } from "@/lib/api";
import { useEffect, useState, useCallback } from "react";
import { Play, Pause, SkipForward, Repeat1, Repeat, Power, Trash2 } from "lucide-react";

type StatusPayload = {
    playing?: boolean;
    loop_song?: boolean;
    loop_queue?: boolean;
};

export function Controls({ onAction }: { onAction: (p: Promise<GrapeResponse>) => void }) {
    const [isPlaying, setIsPlaying] = useState<boolean>(true);
    const [loopSong, setLoopSong] = useState<boolean>(false);
    const [loopQueue, setLoopQueue] = useState<boolean>(false);
    useEffect(() => {
        const statusFn = (api as any).status as undefined | (() => Promise<GrapeResponse>);
        if (typeof statusFn === "function") {
            statusFn()
                .then((r) => {
                    const s = (r?.data ?? {}) as StatusPayload;
                    if (typeof s.playing === "boolean") setIsPlaying(s.playing);
                    if (typeof s.loop_song === "boolean") setLoopSong(s.loop_song);
                    if (typeof s.loop_queue === "boolean") setLoopQueue(s.loop_queue);
                })
                .catch(() => void 0);
        }
    }, []);

    const act = useCallback(
        (p: Promise<GrapeResponse>, after?: () => void) => {
            onAction(p);
            after?.();
        },
        [onAction]
    );

    const IconBtn = (props: React.ButtonHTMLAttributes<HTMLButtonElement> & { active?: boolean }) => {
        const { active, className, ...rest } = props;
        return <button {...rest} className={`icon-btn ${active ? "icon-btn-active" : ""} ${className ?? ""}`} />;
    };

    useEffect(() => {
        const onKey = (e: KeyboardEvent) => {
            const tag = (e.target as HTMLElement)?.tagName;
            if (tag === "INPUT" || tag === "TEXTAREA") return;

            if (e.code === "Space") {
                e.preventDefault();
                if (isPlaying) {
                    act(api.pause(), () => setIsPlaying(false));
                } else {
                    act(api.resume(), () => setIsPlaying(true));
                }
            } else if (e.code === "ArrowRight") {
                act(api.skip());
            } else {
                const k = e.key?.toLowerCase?.();
                if (k === "l") act(api.loopSong(), () => setLoopSong((v) => !v));
                if (k === "q") act(api.loopQueue(), () => setLoopQueue((v) => !v));
            }
        };

        window.addEventListener("keydown", onKey);
        return () => window.removeEventListener("keydown", onKey);
    }, [isPlaying, act]);

    return (
        <section className="card controls">
            <div className="controls-grid">
                {isPlaying ? (
                    <IconBtn
                        title="Pause"
                        aria-label="Pause"
                        onClick={() => act(api.pause(), () => setIsPlaying(false))}>
                        <Pause />
                    </IconBtn>
                ) : (
                    <IconBtn title="Play" aria-label="Play" onClick={() => act(api.resume(), () => setIsPlaying(true))}>
                        <Play />
                    </IconBtn>
                )}

                <IconBtn title="Skip" aria-label="Skip" onClick={() => act(api.skip())}>
                    <SkipForward />
                </IconBtn>

                <IconBtn title="Clear Queue" aria-label="Clear Queue" onClick={() => act(api.clear())}>
                    <Trash2 />
                </IconBtn>

                <IconBtn
                    title="Loop Song"
                    aria-label="Loop Song"
                    active={loopSong}
                    onClick={() => act(api.loopSong(), () => setLoopSong((v) => !v))}>
                    <Repeat1 />
                </IconBtn>

                <IconBtn
                    title="Loop Queue"
                    aria-label="Loop Queue"
                    active={loopQueue}
                    onClick={() => act(api.loopQueue(), () => setLoopQueue((v) => !v))}>
                    <Repeat />
                </IconBtn>

                <IconBtn title="Shutdown Daemon" aria-label="Shutdown Daemon" onClick={() => act(api.shutdown())}>
                    <Power />
                </IconBtn>
            </div>

            <p className="hint">
                <kbd>Space</kbd> Play/Pause · <kbd>→</kbd> Skip · <kbd>L</kbd> Loop Song · <kbd>Q</kbd> Loop Queue
            </p>
        </section>
    );
}
