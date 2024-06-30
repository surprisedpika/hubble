![Hubble Banner]("https://raw.githubusercontent.com/surprisedpika/hubble/master/hubble.png")

# Hubble

Hubble is an input viewer for Windows, MacOS, and Linux (probably).

## Layouts

Layouts are stored in a .json file and .css file. The JSON has two properties, warnUnknown and keys.

```ts
{
  "warnUnknown"?: boolean,
  "keys": {
    "label": string,
    "keys": string or string[],
    "posX": number,
    "posY": number,
    "classes": string or string[]
  }[]
}
```

### Warn Unknown

Defaults to true. When unknown keys are encountered, show a popup.

### Label

The inner text of the given key.

### Keys

The key or key that must be pressed to mark this key as pressed.

There are 3 types of key:

- Mouse Wheel events, prefixed by `mw`
- Mouse Button events, prefixed by `ms`
- Keyboard events, prefixed by `kb`

| Key Code         | Explanation                                   |
| ---------------- | --------------------------------------------- |
| mw_Up            |                                               |
| mw_Down          |                                               |
| mw_Left          | Scrolling left (only supported by some mice)  |
| mw_Right         | Scrolling right (only supported by some mice) |
| mw_Unknown(`x`)  | Unknown scroll event (`x` is a u8)            |
| ms_Left          |                                               |
| ms_Middle        |                                               |
| ms_Right         |                                               |
| ms_Unknown(`x`)  | Unknown click event (`x` is a u8)             |
| kb_Alt           | Alt on Linux and Windows, Option on MacOS     |
| kb_AltGr         |                                               |
| kb_Backspace     |                                               |
| kb_CapsLock      |                                               |
| kb_ControlLeft   |                                               |
| kb_ControlRight  |                                               |
| kb_Delete        |                                               |
| kb_DownArrow     |                                               |
| kb_End           |                                               |
| kb_Escape        |                                               |
| kb_F1            |                                               |
| kb_F2            |                                               |
| kb_F3            |                                               |
| kb_F4            |                                               |
| kb_F5            |                                               |
| kb_F6            |                                               |
| kb_F7            |                                               |
| kb_F8            |                                               |
| kb_F9            |                                               |
| kb_F10           |                                               |
| kb_F11           |                                               |
| kb_F12           |                                               |
| kb_Home          |                                               |
| kb_LeftArrow     |                                               |
| kb_MetaLeft      | "Windows", "Super" or "Command" Left          |
| kb_MetaRight     | "Windows", "Super" or "Command" Right         |
| kb_PageDown      |                                               |
| kb_PageUp        |                                               |
| kb_Return        |                                               |
| kb_RightArrow    |                                               |
| kb_ShiftLeft     |                                               |
| kb_ShiftRight    |                                               |
| kb_Space         |                                               |
| kb_Tab           |                                               |
| kb_UpArrow       |                                               |
| kb_PrintScreen   |                                               |
| kb_ScrollLock    |                                               |
| kb_Pause         |                                               |
| kb_NumLock       |                                               |
| kb_BackQuote     |                                               |
| kb_Num1          |                                               |
| kb_Num2          |                                               |
| kb_Num3          |                                               |
| kb_Num4          |                                               |
| kb_Num5          |                                               |
| kb_Num6          |                                               |
| kb_Num7          |                                               |
| kb_Num8          |                                               |
| kb_Num9          |                                               |
| kb_Num0          |                                               |
| kb_Minus         |                                               |
| kb_Equal         |                                               |
| kb_KeyQ          |                                               |
| kb_KeyW          |                                               |
| kb_KeyE          |                                               |
| kb_KeyR          |                                               |
| kb_KeyT          |                                               |
| kb_KeyY          |                                               |
| kb_KeyU          |                                               |
| kb_KeyI          |                                               |
| kb_KeyO          |                                               |
| kb_KeyP          |                                               |
| kb_LeftBracket   |                                               |
| kb_RightBracket  |                                               |
| kb_KeyA          |                                               |
| kb_KeyS          |                                               |
| kb_KeyD          |                                               |
| kb_KeyF          |                                               |
| kb_KeyG          |                                               |
| kb_KeyH          |                                               |
| kb_KeyJ          |                                               |
| kb_KeyK          |                                               |
| kb_KeyL          |                                               |
| kb_SemiColon     |                                               |
| kb_Quote         |                                               |
| kb_BackSlash     |                                               |
| kb_IntlBackslash |                                               |
| kb_KeyZ          |                                               |
| kb_KeyX          |                                               |
| kb_KeyC          |                                               |
| kb_KeyV          |                                               |
| kb_KeyB          |                                               |
| kb_KeyN          |                                               |
| kb_KeyM          |                                               |
| kb_Comma         |                                               |
| kb_Dot           |                                               |
| kb_Slash         |                                               |
| kb_Insert        |                                               |
| kb_KpReturn      |                                               |
| kb_KpMinus       |                                               |
| kb_KpPlus        |                                               |
| kb_KpMultiply    |                                               |
| kb_KpDivide      |                                               |
| kb_Kp0           |                                               |
| kb_Kp1           |                                               |
| kb_Kp2           |                                               |
| kb_Kp3           |                                               |
| kb_Kp4           |                                               |
| kb_Kp5           |                                               |
| kb_Kp6           |                                               |
| kb_Kp7           |                                               |
| kb_Kp8           |                                               |
| kb_Kp9           |                                               |
| kb_KpDelete      |                                               |
| kb_Function      |                                               |
| kb_Unknown(`x`)  | Unknown keyboard event (`x` is a u32)         |

### PosX and PosY

The location of this key on the screen

### Classes

A list of CSS classes applied to this key. The CSS classes are defined in the .css file.

Additionally, each key is given a `.pressed` class when the key is pressed.

## Styling

The `.global` class can be used to style the div which wraps around all keys (for example, setting the background colour).

If you wish to use any external files in the styling, those files must either be uploaded to the internet, or converted to use the [data URI scheme](https://en.wikipedia.org/wiki/Data_URI_scheme).
