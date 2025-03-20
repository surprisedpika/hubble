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
    emit("editModeLoaded", layout);

    if (typeof window !== undefined) {
      const unlisten = listen("layoutData", (event) => {
        setLayout(event.payload!);
      });

      return () => {
        unlisten.then((fn) => fn());
      };
    }
  }, [layout]);

  return (
    <div className={styles.main}>
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
      <label htmlFor="warnUnknown">
        Display warning when unknown keys are pressed
      </label>
      <br></br>
      <br></br>
      <table>
        <thead>
          <tr>
            <td>Label</td>
            <td>X Position</td>
            <td>Y Position</td>
            <td>CSS Classes</td>
            <td>Keys</td>
          </tr>
        </thead>
        <tbody>
          {layout?.keys?.map((key, index) => {
            return (
              <tr key={index}>
                <td>
                  <input
                    type="text"
                    value={key.label}
                    onChange={(event) => {
                      if (layout.keys === undefined) {
                        return;
                      }

                      let newKeys = [...layout.keys];
                      newKeys[index].label = event.currentTarget.value;
                      updateLayout({ keys: newKeys });
                    }}
                  ></input>
                </td>
                <td>
                  <input
                    type="number"
                    value={key.posX}
                    onChange={(event) => {
                      if (layout.keys === undefined) {
                        return;
                      }

                      let newKeys = [...layout.keys];
                      if (event.currentTarget.value === "") {
                        event.currentTarget.value = "0";
                      }
                      newKeys[index].posX = Number.parseFloat(
                        event.currentTarget.value
                      );
                      updateLayout({ keys: newKeys });
                    }}
                  ></input>
                </td>
                <td>
                  <input
                    type="number"
                    value={key.posY}
                    onChange={(event) => {
                      if (layout.keys === undefined) {
                        return;
                      }

                      let newKeys = [...layout.keys];
                      if (event.currentTarget.value === "") {
                        event.currentTarget.value = "0";
                      }
                      newKeys[index].posY = Number.parseFloat(
                        event.currentTarget.value
                      );
                      updateLayout({ keys: newKeys });
                    }}
                  ></input>
                </td>
                <td>
                  <input
                    type="text"
                    value={key.classes}
                    onChange={(event) => {
                      if (layout.keys === undefined) {
                        return;
                      }

                      let newKeys = [...layout.keys];
                      newKeys[index].classes = event.currentTarget.value;
                      updateLayout({ keys: newKeys });
                    }}
                  ></input>
                </td>
                <td>
                  <input
                    type="text"
                    value={
                      typeof key.keys === "string"
                        ? key.keys
                        : key.keys.join(" ")
                    }
                    onChange={(event) => {
                      if (layout.keys === undefined) {
                        return;
                      }

                      let newKeys = [...layout.keys];
                      newKeys[index].keys =
                        event.currentTarget.value.split(" ");
                      updateLayout({ keys: newKeys });
                    }}
                  ></input>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
      <button
        className="button"
        onClick={() => {
          if (layout === null || layout.keys === undefined) {
            return;
          }

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
      <button
        className="button"
        onClick={() => {
          if (layout === null || layout.keys === undefined) {
            return;
          }

          let newKeys = [...layout.keys];
          newKeys.pop();
          updateLayout({ keys: newKeys });
        }}
      >
        Remove Element
      </button>
    </div>
  );
}
