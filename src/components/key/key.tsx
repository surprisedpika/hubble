"use-client";

import styles from "./styles.module.scss"

interface props {
  label: string,
  isPressed: boolean
}

const getClasses = (isPressed: boolean) => `${styles.key} ${isPressed ? styles.pressed : ""}`

export default function Key(props: props) {
  return (
    <p className={getClasses(props.isPressed)}>{props.label}</p>
  )
}