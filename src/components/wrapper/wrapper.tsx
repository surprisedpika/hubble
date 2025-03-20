"use client";
import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";

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

const writeLayout = async (data: LayoutData) => {
  await invoke("set_layout", {data: JSON.stringify(data), pathStr: localStorage.getItem(LAYOUT_PATH) })
}

export default function Wrapper() {
  const [layout, setLayout] = useState<LayoutData | null>(null);
  const [style, setStyle] = useState<string>("");
  const hasInit = useRef(false);

  const openEditMode = async () => {
    if (typeof window !== undefined) {
      const { WebviewWindow } = await import("@tauri-apps/api/window");
      const webview = new WebviewWindow("editmode", {
        url: "editmode",
        title: "Edit Layout",
      });
      webview.once("tauri://created", () => {
        setTimeout(() => {
          emit("layoutData", layout);
        }, 1000);
      });
    }
  };

  useEffect(() => {
    if (hasInit.current === false) {
      // Run on mount
      hasInit.current = true;
      const path = localStorage.getItem(LAYOUT_PATH);
      getLayout(path ?? undefined);
    }

    if (typeof window !== undefined) {
      const unlisten = listen("newLayout", (event) => {
        const newData: LayoutData = event.payload!;
        writeLayout(newData);
        setLayout(newData);
      });

      return () => {
        unlisten.then((fn) => fn());
      };
    }
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
        <button className="button" onClick={() => openEditMode()}>
          Edit Layout
        </button>
        <button onClick={() => getLayout()} className="button">
          Change Layout
        </button>
      </div>
    </div>
  );
}
