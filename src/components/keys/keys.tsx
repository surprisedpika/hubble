"use-client";

import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

import styles from "./styles.module.scss";

import Key from "@/components/key/key";
import { LayoutData } from "@/components/wrapper/wrapper";
import { listen } from "@tauri-apps/api/event";

interface props {
  layout: LayoutData | undefined;
}

export default function Keys(props: props) {
  const [pressedKeys, setPressedKeys] = useState<Set<string>>(new Set());
  const [unknownKey, setUnknownKey] = useState("");

  useEffect(() => {
    const syncKeys = () => {
      invoke<string[]>("keys").then((keys) => {
        keys.forEach(key => {
          if (key.includes("Unknown")) {
            setUnknownKey(key);
          }
        })
        setPressedKeys(new Set(keys));
        if (keys.length > 0) {
          console.log(keys);
        }
      });
    };

    listen<string>('unknownKey', (payload) => {
      setUnknownKey(payload.payload);
    });

    const syncClock = setInterval(syncKeys, 50);

    return () => clearInterval(syncClock);
  }, []);
  return (
    <div className={`${styles.keys} global`}>
      {Array.isArray(props.layout) &&
        props.layout.map((key, index) => {
          const isPressed =
            typeof key.keys === "string"
              ? pressedKeys.has(key.keys)
              : key.keys.some((k) => pressedKeys.has(k));
          return (
            <Key
              key={index}
              label={key.label}
              isPressed={isPressed}
              classes={key.classes}
              posX={key.posX}
              posY={key.posY}
            />
          );
        })}
        {unknownKey && <p className={styles.warning}>Unknown Key: <code>{unknownKey}</code></p>}
    </div>
  );
}
