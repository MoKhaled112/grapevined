"use client";
import { useEffect, useState } from "react";
import { api, GrapeResponse } from "@/lib/api";
import { Controls } from "@/components/Controls";
import "./styles.css";

export default function Page() {
    const [daemon, setDaemon] = useState<string>("—");
    const [path, setPath] = useState("");
    const [msg, setMsg] = useState<string>("");

    useEffect(() => {
        api.ping()
            .then((r) => setDaemon((r.data as any)?.daemon ?? "unknown"))
            .catch(() => setDaemon("unreachable"));
    }, []);

    async function handle(res: Promise<GrapeResponse>) {
        const r = await res;
        setMsg(r.status === "OK" ? "OK" : r.errmsg ?? "ERR");

        setTimeout(() => setMsg(""), 1800);
    }

    return (
        <main className="wrap">
            <header className="header">
                <div>
                    <h1>Grapevine</h1>
                    <p className="muted">
                        Daemon:{" "}
                        <span className={`badge ${daemon === "unreachable" ? "badge-bad" : "badge-ok"}`}>{daemon}</span>
                    </p>
                </div>
            </header>
            <section className="card">
                {" "}
                <div className="row">
                    <input
                        placeholder="Absolute path to song or .m3u"
                        value={path}
                        onChange={(e) => setPath(e.target.value)}
                        style={{ flex: 1, padding: 8, border: "1px solid #ddd", borderRadius: 8 }}
                        className="input"
                    />
                    <button
                        className="btn"
                        title="Add Song"
                        onClick={() => handle(api.addQueue(path))}
                        disabled={!path}>
                        Add Song
                    </button>
                    <button
                        className="btn btn-ghost"
                        title="Add Playlist (.m3u)"
                        onClick={() => handle(api.addPlaylist(path))}
                        disabled={!path}>
                        Add Playlist
                    </button>
                </div>
            </section>

            <Controls onAction={(p) => handle(p)} />
        </main>
    );
}
