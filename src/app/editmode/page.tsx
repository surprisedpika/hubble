"use client";
import { LayoutData } from "@/components/wrapper/wrapper";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

import styles from "./styles.module.scss";

export default function Page() {
  const [layout, setLayout] = useState<LayoutData | null>(null);

  useEffect(() => {
    if (typeof window !== undefined) {
      const unlisten = listen("layoutData", (event) => {
        console.log("holy moly!");
        setLayout(event.payload!);
      });

      return () => {
        unlisten.then((fn) => fn());
      };
    }
  }, []);

  return (
    <div className={styles.main}>
      <p>Controller: {layout?.controller ? "Enabled" : "Disabled"}</p>
      <p>Warn Unknown: {layout?.warnUnknown ? "Enabled" : "Disabled"}</p>
      <></> {/* Controller enabled, warn unknown */}
      <ul>
        <li>
          {/* label, keys, posX, posY, classes */}
          <input></input>
          <input></input>
          <input></input>
          <input></input>
          <input></input>
        </li>
      </ul>
    </div>
  );
}
