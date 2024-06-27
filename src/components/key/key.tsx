"use-client";

import styles from "./styles.module.scss";

interface props {
  label: string;
  isPressed: boolean;
  posX: number;
  posY: number;
  classes: string | string[];
}

export default function Key(props: props) {
  const getClasses = () =>
  `${styles.key} ${props.isPressed ? "pressed" : ""} ${typeof props.classes === "string" ? props.classes : props.classes.join(" ")}`;
  return (
    <p
      className={getClasses()}
      style={
        {
          "--pos-x": `${props.posX}px`,
          "--pos-y": `${props.posY}px`,
        } as React.CSSProperties
      }
    >
      {props.label}
    </p>
  );
}
