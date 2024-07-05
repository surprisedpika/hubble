"use client";
import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import Keys from "@/components/keys/keys";

import styles from "./styles.module.scss";

export interface LayoutData {
  warnUnknown?: boolean;
  controller?: boolean;
  keys?: {
    label: string;
    keys: string | string[];
    posX: number;
    posY: number;
    classes: string | string[];
  }[];
}

const LAYOUT_PATH = "layoutPath";

export default function Wrapper() {
  const [layout, setLayout] = useState<LayoutData | null>(null);
  const [style, setStyle] = useState<string>("");
  const hasInit = useRef(false);

  // Run on mount
  useEffect(() => {
    if (hasInit.current) return;
    hasInit.current = true;
    const path = localStorage.getItem(LAYOUT_PATH);
    getLayout(path ?? undefined);
  }, []);

  const getLayout = async (path?: string) => {
    await invoke<[string, string, string] | null>("get_layout", {
      previousPath: path,
    })
      .then((res) => {
        if (res === null) {
          console.error("Theme could not be read!");
          return;
        }
        const [json, css, path] = res;
        localStorage.setItem(LAYOUT_PATH, path);
        setLayout(JSON.parse(json));
        setStyle(css);
      })
      .catch(console.error);
  };

  return (
    <div>
      {/* Can ignore "dangerous" as the css is user-defined and the app does not connect to a server */}
      <style dangerouslySetInnerHTML={{ __html: style }} />
      {layout && style && <Keys layout={layout} />}
      <button onClick={() => getLayout()} className={styles.button}>
        Change Layout
      </button>
    </div>
  );
}
