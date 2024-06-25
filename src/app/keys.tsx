"use-client";

import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

export default function Home() {
  const emptyArray: any[] = []
  const [keys, setKeys] = useState(emptyArray);
  
  useEffect(() => {
    const syncKeys = () => {
      invoke<any[]>("keys").then((keys) => setKeys(keys))
    }

    const syncClock = setInterval(syncKeys, 100);

    return () => clearInterval(syncClock);
  }, [])
  return (
    <div>
      <h1>What the scallop</h1>
      <p>{keys}</p>
    </div>
  );
}

// invoke<any[]>("keys").then((keys) => console.log(keys));