"use client";

// Source - https://stackoverflow.com/a/68928267
// Posted by Daniel Loiterton, modified by community. See post 'Timeline' for change history
// Retrieved 2026-07-24, License - CC BY-SA 4.0

import React, { useEffect, useRef, useState } from "react";

type InputProps = React.ComponentProps<"input">;
const ControlledInput: React.FC<InputProps> = (props) => {
  const { value, onChange, ...rest } = props;
  const [cursor, setCursor] = useState<number | null>(null);
  const ref = useRef<HTMLInputElement>(null);

  useEffect(() => {
    ref.current?.setSelectionRange(cursor, cursor);
  }, [ref, cursor, value]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setCursor(e.target.selectionStart);
    onChange?.(e);
  };

  return <input ref={ref} value={value} onChange={handleChange} {...rest} />;
};

export default ControlledInput;
