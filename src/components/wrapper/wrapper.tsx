"use client";

import Keys from "@/components/keys/keys";
import { ChangeEvent, useState } from "react";

export type LayoutData = {
  label: string;
  keys: string | string[];
  posX: number;
  posY: number;
  classes: string | string[];
}[];

export default function Wrapper() {
  const [layout, setLayout] = useState(JSON.parse("{}"));
  const [style, setStyle] = useState<string>("");
  const handleFileChange = (event: ChangeEvent<HTMLInputElement>) => {
    const files = event.target.files;
    if (files === null) {
      return;
    }
    const reader = new FileReader();
    reader.onload = (e) => {
      const res = e.target?.result;
      if (res === null || res === undefined || res instanceof ArrayBuffer) {
        return;
      }
      if (files[0].name.endsWith(".json")) {
        setLayout(JSON.parse(res));
      } else {
        setStyle(res);
      }
    };
    reader.readAsText(files[0]);
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
