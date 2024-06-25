"use-client";

import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

export default function Home() {
  const [keys, setKeys] = useState([] as string[]);
  
  useEffect(() => {
    const syncKeys = () => {
      invoke<any[]>("keys").then((keys) => setKeys(keys))
    }

    const syncClock = setInterval(syncKeys, 50);

    return () => clearInterval(syncClock);
  }, [])
  return (
    <div>
      <h1>What the scallop</h1>
      <ul>
        {keys.map((a, b) => {
          return (<li key={b}>{a}</li>)
        })}
      </ul>
    </div>
  );
}