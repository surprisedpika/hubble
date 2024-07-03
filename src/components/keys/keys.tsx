"use-client";

import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

import styles from "./styles.module.scss";

import Key from "@/components/key/key";
import { LayoutData } from "@/components/wrapper/wrapper";
import { listen } from "@tauri-apps/api/event";
import { Controller, isKeyPressed, localToGlobalKey } from "./functions";

interface props {
  layout: LayoutData | null;
}

export default function Keys(props: props) {
  const [globalPressedKeys, setGlobalPressedKeys] = useState<Set<string>>(
    new Set()
  );
  const [localPressedKeys, setLocalPressedKeys] = useState<Set<string>>(
    new Set()
  );
  const [controller, setController] = useState<Controller | null>(null);
  const [unknownKey, setUnknownKey] = useState("");

  useEffect(() => {
    invoke<Controller | null>("controller").then(data => setController(data));
    const sync = () => {
      invoke<string[] | null>("keys").then((keys) => {
        if (!Array.isArray(keys)) {
          setGlobalPressedKeys(new Set());
          return;
        }
        keys.forEach((key) => {
          if (key && typeof key === "string" && key.includes("Unknown")) {
            setUnknownKey(key);
          }
          if (localPressedKeys.has(key)) {
            setLocalPressedKeys((before) => {
              before.delete(key);
              return before;
            });
          }
        });
        setGlobalPressedKeys(new Set(keys));
      });
    };

    listen<string>("unknownKey", (payload) => {
      setUnknownKey(payload.payload);
    });

    const syncClock = setInterval(sync, 50);

    const keydownCallback = (e: KeyboardEvent) => {
      const k = localToGlobalKey(e.code);
      setLocalPressedKeys((before) => before.add(k));
    };
    const keyupCallback = (e: KeyboardEvent) => {
      const k = localToGlobalKey(e.code);
      if (globalPressedKeys.has(k)) {
        invoke<undefined>("unstick_key", { key: k });
      }
      setLocalPressedKeys((before) => {
        before.delete(k);
        return before;
      });
    };

    document.addEventListener("keydown", keydownCallback);
    document.addEventListener("keyup", keyupCallback);

    // On unmount
    return () => {
      clearInterval(syncClock);
      document.removeEventListener("keydown", keydownCallback);
      document.removeEventListener("keyup", keyupCallback);
    };
  }, [globalPressedKeys, localPressedKeys]);

  return (
    <div className={`${styles.keys} global`}>
      {Array.isArray(props.layout?.keys) &&
        props.layout.keys.map((key, index) => {
          const isPressed = isKeyPressed(
            key.keys,
            globalPressedKeys,
            localPressedKeys,
            controller
          );
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
      {(props.layout?.warnUnknown ?? true) && unknownKey && (
        <p className={styles.warning}>
          Unknown Key: <code>{unknownKey}</code>
        </p>
      )}
    </div>
  );
}
