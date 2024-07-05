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

  dpad_up: boolean;
  dpad_down: boolean;
  dpad_left: boolean;
  dpad_right: boolean;

  face_top_left: boolean;
  face_bottom_left: boolean;
  face_top_right: boolean;
  face_bottom_right: boolean;

  l_stick: [number, number];
  r_stick: [number, number];

  unknown: boolean[];
}

/**
 *
 * @param key Name of key as given by `document.addEventListener("keydown" | "keyup")`
 * @returns Name of key as given by rdev enums (with custom prefixes)
 */
export function localToGlobalKey(key: string): string {
  // Turn ArrowRight into kb_RightArrow
  if (key.startsWith("Arrow")) {
    return key.replace("Arrow", "kb_").concat("Arrow");
  }
  if (key === "Period") {
    return "kb_Dot";
  }
  // Turn DigitX to kb_NumX
  if (key.startsWith("Digit")) {
    return key.replace("Digit", "kb_Num");
  }
  // AltGr, etc. etc.
  if (key.startsWith("Alt")) {
    return "kb_Alt";
  }
  // Turn BracketLeft to kb_LeftBracket
  if (key.startsWith("Bracket")) {
    console.log(key.replace("Bracket", "kb_").concat("Bracket"));
    return key.replace("Bracket", "kb_").concat("Bracket");
  }
  if (key === "Backquote") {
    return "kb_BackQuote";
  }
  if (key === "Semicolon") {
    return "kb_SemiColon";
  }
  // Handles both NumpadEnter and Enter
  if (key.includes("Enter")) {
    if (key === "NumpadEnter") {
      return "kb_KpReturn";
    }
    return "kb_Return";
  }
  // Handles all other Numpad Keys (Numpad -> Kp)
  if (key.startsWith("Numpad")) {
    // Special case with different operator names
    if (key.endsWith("Minus")) {
      return "kb_KpSubtract";
    }
    // Special case with different operator names
    if (key.endsWith("Add")) {
      return "kb_KpPlus";
    }
    return key.replace("Numpad", "kb_Kp");
  }
  // Default case
  return "kb_".concat(key);
}

/**
 * @param b Name of key as defined in `layout.json` 
 * @param controller Current controller state
 */
function isControllerButtonPressed(b: string, controller: Controller) {
  // b will be something like "cb_someControllerButton"
  const button = b.substring(3);
  // Special unknown handling as it is a bool array rather than just a bool
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
  // Special l_stick and r_stick handling as they are not buttons (use l_stick_click and r_stick_click instead)
  if (button === "l_stick" || button === "r_stick") {
    return false;
  }
  // Double negation coerces undefined values (ie key does not exist) to false
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
    // If the window is not in focus
    if (globalPressedKeys.has(keys)) {
      return true;
    }
    // If the window is in focus
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