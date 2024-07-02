"use-client";

import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

import styles from "./styles.module.scss";

import Key from "@/components/key/key";
import { LayoutData } from "@/components/wrapper/wrapper";
import { listen } from "@tauri-apps/api/event";

interface props {
  layout: LayoutData | null;
}

function convertKey(key: string): string {
  if (key.startsWith("Arrow")) {
    return key.replace("Arrow", "kb_").concat("Arrow");
  }
  if (key === "Period") {
    return "kb_Dot";
  }
  if (key.startsWith("Digit")) {
    return key.replace("Digit", "kb_Num");
  }
  if (key.startsWith("Alt")) {
    return "kb_Alt";
  }
  if (key.startsWith("Bracket")) {
    return key.replace("Bracket", "kb_").concat("Bracket");
  }
  if (key === "Backquote") {
    return "kb_BackQuote";
  }
  if (key === "Semicolon") {
    return "kb_SemiColon";
  }
  if (key.includes("Enter")) {
    if (key.length === 11) {
      // NumpadEnter
      return "kb_KpReturn";
    }
    // Enter
    return "kb_Return";
  }
  if (key.startsWith("Numpad")) {
    if (key.endsWith("Minus")) {
      return "kb_KpSubtract";
    }
    if (key.endsWith("Add")) {
      return "kb_KpPlus";
    }
    return key.replace("Numpad", "kb_Kp");
  }
  return "kb_".concat(key);
}

export default function Keys(props: props) {
  const [globalPressedKeys, setGlobalPressedKeys] = useState<Set<string>>(
    new Set()
  );
  const [localPressedKeys, setLocalPressedKeys] = useState<Set<string>>(
    new Set()
  );
  const [unknownKey, setUnknownKey] = useState("");

  useEffect(() => {
    const syncKeys = () => {
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

    const syncClock = setInterval(syncKeys, 50);

    const keydownCallback = (e: KeyboardEvent) => {
      const k = convertKey(e.code);
      setLocalPressedKeys((before) => before.add(k));
    };
    const keyupCallback = (e: KeyboardEvent) => {
      const k = convertKey(e.code);
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
          const isPressed =
            typeof key.keys === "string"
              ? globalPressedKeys.has(key.keys) ||
                localPressedKeys.has(key.keys)
              : key.keys.some((k) => globalPressedKeys.has(k));
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
