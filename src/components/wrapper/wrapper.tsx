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
    classes: string;
  }[];
}

const LAYOUT_PATH = "layoutPath";

const openEditMode = async () => {
  if (typeof window !== undefined) {
    const { WebviewWindow } = await import("@tauri-apps/api/window");
    const webview = new WebviewWindow("editmode", {
      url: "editmode",
    });
    webview.show();
  }
}

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
    console.log("hi");
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
        const newLayout = JSON.parse(json) as LayoutData;
        setLayout(newLayout);
        if (newLayout.controller == true) {
          invoke("start_controller_polling");
        } else {
          invoke("stop_controller_polling");
        }
        setStyle(css);
      })
      .catch(console.error);
  };

  return (
    <div>
      {/* Can ignore "dangerous" as the css is user-defined and the app does not connect to a server */}
      <style dangerouslySetInnerHTML={{ __html: style }} />
      {layout && style && <Keys layout={layout} />}
      <div className={styles.controls}>
        <button
          className={styles.button}
          onClick={() => openEditMode()}
        >
          Edit Layout
        </button>
        <button onClick={() => getLayout()} className={styles.button}>
          Change Layout
        </button>
      </div>
    </div>
  );
}
