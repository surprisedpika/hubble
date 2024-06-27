"use-client";

import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

import styles from "./styles.module.scss";

import Key from "@/components/key/key";
import { LayoutData } from "@/components/wrapper/wrapper";

interface props {
  layout: LayoutData | undefined;
}

export default function Keys(props: props) {
  const [pressedKeys, setPressedKeys] = useState<Set<string>>(new Set());

  useEffect(() => {
    const syncKeys = () => {
      invoke<string[]>("keys").then((keys) => {
        setPressedKeys(new Set(keys));
        if (keys.length > 0) {
          console.log(keys);
        }
      });
    };

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
    </div>
  );
}
