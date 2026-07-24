"use client";
import { LayoutData, PayloadData } from "@/components/wrapper/wrapper";
import { useEffect, useRef, useState } from "react";
import { emit, listen } from "@tauri-apps/api/event";

import styles from "./styles.module.scss";
import ControlledInput from "@/components/ControlledInput/ControlledInput";
import ControlledTextArea from "@/components/ControlledTextArea/ControlledTextArea";

export default function Page() {
  const [layout, setLayout] = useState<LayoutData | null>(null);
  const [style, setStyle] = useState<string>("");
  const hasInit = useRef(false);

  const updateLayout = async (data: Partial<LayoutData> | string) => {
    let newLayout;
    let newStyle;
    if (typeof data === "string") {
      newLayout = layout;
      newStyle = data;
    } else {
      newLayout = { ...layout, ...data };
      newStyle = style;
    }
    const newData: PayloadData = { layout: newLayout ?? {}, style: newStyle };
    await emit("newLayout", newData);
    setLayout(newLayout);
    setStyle(newStyle);
  };

  useEffect(() => {
    if (hasInit.current === false) {
      // Run on mount
      hasInit.current = true;
      emit("editModeLoaded", layout);
    }

    if (typeof window !== undefined) {
      const unlisten = listen("layoutData", (event) => {
        const data = event.payload as PayloadData;
        setLayout(data.layout);
        setStyle(data.style);
      });

      return () => {
        unlisten.then((fn) => fn());
      };
    }
  }, [layout]);

  return (
    <div className={styles.main}>
      <h2>Layout Editor</h2>
      <input
        id="controller"
        type="checkbox"
        checked={layout?.controller ?? false}
        onChange={() => updateLayout({ controller: !layout?.controller })}
      ></input>
      <label htmlFor="controller">Poll for controllers</label>
      <br></br>
      <input
        id="warnUnknown"
        type="checkbox"
        checked={layout?.warnUnknown ?? false}
        onChange={() => updateLayout({ warnUnknown: !layout?.warnUnknown })}
      ></input>
      <label htmlFor="warnUnknown">Display warning when unknown keys are pressed</label>
      <br></br>
      <br></br>
      <table>
        <thead>
          <tr>
            <td>Label</td>
            <td>X Position</td>
            <td>Y Position</td>
            <td>CSS Classes</td>
            <td>Keys (Comma Separated)</td>
          </tr>
        </thead>
        <tbody>
          {layout?.keys?.map((key, index) => {
            return (
              <tr key={index}>
                <td>
                  <ControlledInput
                    type="text"
                    value={key.label}
                    onChange={(event) => {
                      if (layout.keys === undefined) return;
                      let newKeys = [...layout.keys];
                      newKeys[index].label = event.currentTarget.value;
                      updateLayout({ keys: newKeys });
                    }}
                  ></ControlledInput>
                </td>
                <td>
                  <ControlledInput
                    type="text"
                    inputMode="numeric"
                    value={key.posX}
                    onChange={(event) => {
                      if (layout.keys === undefined) return;
                      let newKeys = [...layout.keys];
                      if (event.currentTarget.value === "") {
                        event.currentTarget.value = "0";
                      }
                      newKeys[index].posX = Number.parseFloat(event.currentTarget.value);
                      updateLayout({ keys: newKeys });
                    }}
                  ></ControlledInput>
                </td>
                <td>
                  <ControlledInput
                    type="text"
                    inputMode="numeric"
                    value={key.posY}
                    onChange={(event) => {
                      if (layout.keys === undefined) return;
                      let newKeys = [...layout.keys];
                      if (event.currentTarget.value === "") {
                        event.currentTarget.value = "0";
                      }
                      newKeys[index].posY = Number.parseFloat(event.currentTarget.value);
                      updateLayout({ keys: newKeys });
                    }}
                  ></ControlledInput>
                </td>
                <td>
                  <ControlledInput //Classes input
                    type="text"
                    value={key.classes}
                    onChange={(event) => {
                      if (layout.keys === undefined) return;
                      let newKeys = [...layout.keys];
                      newKeys[index].classes = event.currentTarget.value;
                      updateLayout({ keys: newKeys });
                    }}
                  ></ControlledInput>
                </td>
                <td>
                  <ControlledInput //Keys input
                    type="text"
                    value={typeof key.keys === "string" ? key.keys : key.keys.join(",")}
                    onChange={(event) => {
                      if (layout.keys === undefined) return;
                      let newKeys = [...layout.keys];
                      newKeys[index].keys = event.currentTarget.value.split(",");
                      updateLayout({ keys: newKeys });
                    }}
                  ></ControlledInput>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
      <button //Add element button
        className="button"
        onClick={() => {
          if (layout === null || layout.keys === undefined) return;
          let newKeys = [...layout.keys];
          newKeys.push({
            label: "",
            keys: "",
            posX: 0,
            posY: 0,
            classes: "",
          });
          updateLayout({ keys: newKeys });
        }}
      >
        Add Element
      </button>
      <button //Remove element button
        className="button"
        onClick={() => {
          if (layout === null || layout.keys === undefined) return;
          let newKeys = [...layout.keys];
          newKeys.pop();
          updateLayout({ keys: newKeys });
        }}
      >
        Remove Element
      </button>
      <h3>CSS</h3>
      <ControlledTextArea
        placeholder=".key { color: red }"
        spellCheck="false"
        value={style}
        onChange={async (e) => {
          updateLayout(e.target.value);
        }}
      ></ControlledTextArea>
    </div>
  );
}
