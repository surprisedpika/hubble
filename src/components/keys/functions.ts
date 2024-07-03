export function localToGlobalKey(key: string): string {
  if (key.startsWith("Arrow")) {
    return key.replace("Arrow", "kb_").concat("Arrow");
  }
  if (key === "Period") {
    return "kb_Dot";
  }
  if (key.startsWith("Digit")) {
    return key.replace("Digit", "kb_Num");
  }
  if (key.startsWith("Alt")) {
    return "kb_Alt";
  }
  if (key.startsWith("Bracket")) {
    return key.replace("Bracket", "kb_").concat("Bracket");
  }
  if (key === "Backquote") {
    return "kb_BackQuote";
  }
  if (key === "Semicolon") {
    return "kb_SemiColon";
  }
  if (key.includes("Enter")) {
    if (key.length === 11) {
      // NumpadEnter
      return "kb_KpReturn";
    }
    // Enter
    return "kb_Return";
  }
  if (key.startsWith("Numpad")) {
    if (key.endsWith("Minus")) {
      return "kb_KpSubtract";
    }
    if (key.endsWith("Add")) {
      return "kb_KpPlus";
    }
    return key.replace("Numpad", "kb_Kp");
  }
  return "kb_".concat(key);
}

function isControllerButtonPressed(b: string, controller: Controller) {
  const button = b.substring(3);
  if (button.startsWith("unknown")) {
    let match = button.match(/\((\d+)\)/);
    if (match === null) {
      throw new Error(`Error parsing layout.json syntax at button: ${button}`);
    }
    let i = Number(match[1]);
    if (i < 0 || !Number.isInteger(i)) {
      throw new Error(`Error parsing layout.json syntax at button: ${button}`);
    }
    if (controller.unknown.length - 1 < i) {
      return false;
    }
    return controller.unknown[i];
  }
  if (button == "l_stick" || button == "r_stick") {
    return false;
  }
  return !!controller[button as keyof Controller];
}

export function isKeyPressed(
  keys: string | string[],
  globalPressedKeys: Set<string>,
  localPressedKeys: Set<string>,
  controller: Controller | null
): boolean {
  if (typeof keys === "string") {
    if (keys.startsWith("cb_")) {
      if (controller === null) {
        return false;
      }
      return isControllerButtonPressed(keys, controller);
    }
    if (globalPressedKeys.has(keys)) {
      return true;
    }
    if (localPressedKeys.has(keys)) {
      return true;
    }
  } else {
    if (
      keys.some((key) => {
        globalPressedKeys.has(key) ||
          localPressedKeys.has(key) ||
          (controller !== null && isControllerButtonPressed(key, controller));
      })
    ) {
      return true;
    }
  }
  return false;
}

export interface Controller {
  north: boolean;
  south: boolean;
  east: boolean;
  west: boolean;

  r_trigger: boolean;
  l_trigger: boolean;
  r_bumper: boolean;
  l_bumper: boolean;

  l_stick_click: boolean;
  r_stick_click: boolean;

  d_up: boolean;
  d_down: boolean;
  d_left: boolean;
  d_right: boolean;

  face_left_top: boolean;
  face_left_bottom: boolean;
  face_right_top: boolean;
  face_right_bottom: boolean;

  l_stick: [number, number];
  r_stick: [number, number];

  unknown: boolean[];
}
