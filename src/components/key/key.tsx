"use-client";

import styles from "./styles.module.scss";

interface props {
  label: string;
  isPressed: boolean;
  posX: number;
  posY: number;
}

const getClasses = (isPressed: boolean) =>
  `${styles.key} ${isPressed ? styles.pressed : ""}`;

export default function Key(props: props) {
  return (
    <p
      className={getClasses(props.isPressed)}
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
