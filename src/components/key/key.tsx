"use-client";

import styles from "./styles.module.scss";

interface props {
  label: string | null;
  isPressed: boolean | null;
  posX: number | null;
  posY: number | null;
  classes: string | null;
}

export default function Key(props: props) {
  const getClasses = () =>
    `${styles.key} ${props.isPressed ? "pressed" : ""} ${
      typeof props.classes === "string"
        ? props.classes
        : ""
    }`;
  return (
    <p
      className={getClasses()}
      style={
        {
          "--pos-x": `${props.posX ? props.posX : 0}px`,
          "--pos-y": `${props.posY ? props.posY : 0}px`,
        } as React.CSSProperties
      }
    >
      {props.label}
    </p>
  );
}
