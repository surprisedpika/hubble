"use client";
import { LayoutData } from "@/components/wrapper/wrapper";
import { useEffect, useState } from "react";
import { emit, listen } from "@tauri-apps/api/event";

import styles from "./styles.module.scss";

export default function Page() {
  const [layout, setLayout] = useState<LayoutData | null>(null);

  const updateLayout = async (data: Partial<LayoutData>) => {
    const newData = { ...layout, ...data };
    await emit("newLayout", newData);
    setLayout(newData);
  };

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
      <p>{layout?.controller ? "a" : "b"}</p>
      <input
        type="checkbox"
        checked={layout?.controller ?? false}
        onChange={() =>
          updateLayout({
            ...{ controller: !layout?.controller },
          })
        }
      ></input>
      <input
        type="checkbox"
        checked={layout?.warnUnknown ?? false}
        onChange={() =>
          updateLayout({
            ...{ warnUnknown: !layout?.warnUnknown },
          })
        }
      ></input>
      {/* Controller enabled, warn unknown */}
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
