"use client";

import Keys from "@/components/keys/keys";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";

import styles from "./styles.module.scss";

export type LayoutData = {
  label: string;
  keys: string | string[];
  posX: number;
  posY: number;
  classes: string | string[];
}[];

export default function Wrapper() {
  const [layout, setLayout] = useState<LayoutData>(JSON.parse("{}"));
  const [style, setStyle] = useState<string>("");

  useEffect(() => {
    const jsonFile = localStorage.getItem("layoutJSON");
    const cssFile = localStorage.getItem("layoutCSS");
    if (jsonFile !== null && cssFile !== null) {
      setLayout(JSON.parse(jsonFile) as LayoutData);
      setStyle(cssFile);
      return;
    }
    getLayout();
  }, []);

  const getLayout = async () => {
    invoke<[string, string]>("get_layout")
      .then(res => {
        if (res === null) {
          console.error("Theme could not be read!");
          return;
        }
        const [json, css] = res;
        localStorage.setItem("layoutJSON", json);
        localStorage.setItem("layoutCSS", css);
        setLayout(JSON.parse(json));
        setStyle(css);
      })
      .catch(console.error);
  };

  return (
    <div>
      <style dangerouslySetInnerHTML={{ __html: style }} />
      {layout && style && <Keys layout={layout} />}
      <button onClick={getLayout} className={styles.button}>Change Layout</button>
    </div>
  );
}
