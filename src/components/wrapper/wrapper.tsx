"use client";
import React, { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";
import { transform } from "@babel/standalone";

import Keys from "@/components/keys/keys";

import styles from "./styles.module.scss";
import { jsx } from "react/jsx-runtime";

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
  await invoke("set_layout", {
    data: JSON.stringify(data),
    pathStr: localStorage.getItem(LAYOUT_PATH),
  });
  React.createElement("");
};

export default function Wrapper() {
  const [layout, setLayout] = useState<LayoutData | null>(null);
  const [style, setStyle] = useState<string>("");
  const [d, setD] = useState(<>ERM 2</>);
  const hasInit = useRef(false);

  const openEditMode = async () => {
    if (typeof window !== undefined) {
      const { WebviewWindow } = await import("@tauri-apps/api/window");
      new WebviewWindow("editmode", {
        url: "editmode",
        title: "Edit Layout",
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
      const unlistenNewLayout = listen("newLayout", (event) => {
        const newData: LayoutData = event.payload!;
        writeLayout(newData);
        setLayout(newData);
      });

      const unlistenEditOpened = listen("editModeLoaded", () => {
        emit("layoutData", layout);
      });

      return () => {
        unlistenNewLayout.then((fn) => fn());
        unlistenEditOpened.then((fn) => fn());
      };
    }
  }, [layout]);

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
        <button onClick={() => invoke("main")}>erm</button>
        <button
          onClick={() => {
            let fileContents = "setD(<p>hi</p>)"

            let jsCode = transform(fileContents, {
              presets: [["react", { runtime: "automatic" }]],
              sourceType: "unambiguous",
            }).code;

            if (jsCode) {
              jsCode = jsCode.replace(
                /require\(["']react\/jsx-runtime["']\)/g,
                "React"
              );
              const a = {
                jsx: jsx
              }
              new Function('React', 'setD', jsCode)(a, setD);
            }
          }}
        >
          {d}
        </button>
      </div>
    </div>
  );
}
