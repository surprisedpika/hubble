"use-client";

import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

import styles from "./styles.module.scss"
import Key from "../key/key";

export default function Keys() {
  const [keys, setKeys] = useState(new Set());

  useEffect(() => {
    const syncKeys = () => {
      invoke<any[]>("keys").then((keys) => {
        setKeys(new Set(keys));
        console.log(keys);
      });
    };

    const syncClock = setInterval(syncKeys, 50);

    return () => clearInterval(syncClock);
  }, []);
  return (
    <div className={styles.keys}>
      <Key label="W" isPressed={keys.has("KeyW")} />
      <Key label="A" isPressed={keys.has("KeyA")} />
      <Key label="S" isPressed={keys.has("KeyS")} />
      <Key label="D" isPressed={keys.has("KeyD")} />
    </div>
  );
}
