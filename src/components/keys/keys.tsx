"use-client";

import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

import styles from "./styles.module.scss";
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
      <Key label="W" isPressed={keys.has("kb_KeyW")} />
      <Key label="A" isPressed={keys.has("kb_KeyA")} />
      <Key label="S" isPressed={keys.has("kb_KeyS")} />
      <Key label="D" isPressed={keys.has("kb_KeyD")} />
      <Key
        label="^"
        isPressed={keys.has("mw_Up") || keys.has("ms_Unknown(2)")} // For when using stray macro
      />
      <Key label="L" isPressed={keys.has("ms_Left")} />
    </div>
  );
}
