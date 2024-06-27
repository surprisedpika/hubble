"use client";

import Keys from "@/components/keys/keys";
import { ChangeEvent, useEffect, useState } from "react";

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
    const jsonFile = localStorage.getItem("layoutJSONURI");
    const cssFile = localStorage.getItem("layoutCSSURI");
    if (jsonFile !== null) {
      setLayout(JSON.parse(jsonFile));
    }
    if (cssFile !== null) {
      setStyle(cssFile);
    }
  }, []);

  const readFile = (file: File) => {
    const reader = new FileReader();
    reader.onload = (e) => {
      const res = e.target?.result;
      if (res === null || res === undefined || res instanceof ArrayBuffer) {
        return;
      }
      if (file.name.endsWith(".json")) {
        localStorage.setItem("layoutJSONURI", res);
        setLayout(JSON.parse(res));
      } else {
        localStorage.setItem("layoutCSSURI", res);
        setStyle(res);
      }
    };
    reader.readAsText(file);
  };

  const handleFileChange = (event: ChangeEvent<HTMLInputElement>) => {
    //TODO user cancels file select
    const files = event.target.files;
    if (files === null) {
      return;
    }
    readFile(files[0]);
  };

  return (
    <div>
      <style dangerouslySetInnerHTML={{ __html: style }} />
      <input type="file" accept=".css" onChange={handleFileChange} />
      <input type="file" accept=".json" onChange={handleFileChange} />
      {layout && style && <Keys layout={layout} />}
    </div>
  );
}
