"use client";

import Keys from "@/components/keys/keys";
import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api";

import styles from "./styles.module.scss";

export type LayoutData = {
  label: string;
  keys: string | string[];
  posX: number;
  posY: number;
  classes: string | string[];
}[];

const LAYOUT_PATH = "layoutPath";

export default function Wrapper() {
  const [layout, setLayout] = useState<LayoutData>(JSON.parse("{}"));
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
    invoke<[string, string, string]>("get_layout", {"previousPath": path})
      .then(res => {
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
      <style dangerouslySetInnerHTML={{ __html: style }} />
      {layout && style && <Keys layout={layout} />}
      <button onClick={() => getLayout()} className={styles.button}>Change Layout</button>
    </div>
  );
}
