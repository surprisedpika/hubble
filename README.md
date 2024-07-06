![Hubble Banner](https://raw.githubusercontent.com/surprisedpika/hubble/master/github-assets/hubble.png)

# Hubble

Hubble is an input viewer for Windows, MacOS, and Linux (probably).

![Controller Example](https://raw.githubusercontent.com/surprisedpika/hubble/master/github-assets/controller.gif)

## Layouts

Layouts are stored in a `layout.json` file and `layout.css` file. The JSON has two properties, warnUnknown and keys.

```ts
// layout.json
{
  "warnUnknown"?: boolean,
  "controller"?: boolean,
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

Defaults to true. When keys unknown to the program are encountered, show a popup.

### Controller

Set this if you wish the input display to poll controllers. Supported Controllers:

- Switch Pro using HID & Bluetooth
- Switch Pro (and possibly other controllers, I don't own any to check) using SteamInput

### Label

The inner text of the given key.

### Keys

The key or key that must be pressed to mark this key as pressed.

There are 4 types of key:

- Mouse Wheel events, prefixed by `mw`
- Mouse Button events, prefixed by `ms`
- Keyboard events, prefixed by `kb`
- Controller events, prefixed by `cb`

| Key Code             | Explanation                                                        |
| -------------------- | ------------------------------------------------------------------ |
| mw_Up                |                                                                    |
| mw_Down              |                                                                    |
| mw_Left              | Scrolling left (only supported by some mice)                       |
| mw_Right             | Scrolling right (only supported by some mice)                      |
| mw_Unknown           | Unknown wheel event                                                |
| ms_Left              |                                                                    |
| ms_Middle            |                                                                    |
| ms_Right             |                                                                    |
| ms_Unknown(`x`)      | Unknown click event (`x` is a u8)                                  |
| kb_Alt               | Alt on Linux and Windows, Option on MacOS                          |
| kb_AltGr             |                                                                    |
| kb_Backspace         |                                                                    |
| kb_CapsLock          |                                                                    |
| kb_ControlLeft       |                                                                    |
| kb_ControlRight      |                                                                    |
| kb_Delete            |                                                                    |
| kb_DownArrow         |                                                                    |
| kb_End               |                                                                    |
| kb_Escape            |                                                                    |
| kb_F1                |                                                                    |
| kb_F2                |                                                                    |
| kb_F3                |                                                                    |
| kb_F4                |                                                                    |
| kb_F5                |                                                                    |
| kb_F6                |                                                                    |
| kb_F7                |                                                                    |
| kb_F8                |                                                                    |
| kb_F9                |                                                                    |
| kb_F10               |                                                                    |
| kb_F11               |                                                                    |
| kb_F12               |                                                                    |
| kb_Home              |                                                                    |
| kb_LeftArrow         |                                                                    |
| kb_MetaLeft          | "Windows", "Super" or "Command" Left                               |
| kb_MetaRight         | "Windows", "Super" or "Command" Right                              |
| kb_PageDown          |                                                                    |
| kb_PageUp            |                                                                    |
| kb_Return            |                                                                    |
| kb_RightArrow        |                                                                    |
| kb_ShiftLeft         |                                                                    |
| kb_ShiftRight        |                                                                    |
| kb_Space             |                                                                    |
| kb_Tab               |                                                                    |
| kb_UpArrow           |                                                                    |
| kb_PrintScreen       |                                                                    |
| kb_ScrollLock        |                                                                    |
| kb_Pause             |                                                                    |
| kb_NumLock           |                                                                    |
| kb_BackQuote         |                                                                    |
| kb_Num1              |                                                                    |
| kb_Num2              |                                                                    |
| kb_Num3              |                                                                    |
| kb_Num4              |                                                                    |
| kb_Num5              |                                                                    |
| kb_Num6              |                                                                    |
| kb_Num7              |                                                                    |
| kb_Num8              |                                                                    |
| kb_Num9              |                                                                    |
| kb_Num0              |                                                                    |
| kb_Minus             |                                                                    |
| kb_Equal             |                                                                    |
| kb_KeyQ              |                                                                    |
| kb_KeyW              |                                                                    |
| kb_KeyE              |                                                                    |
| kb_KeyR              |                                                                    |
| kb_KeyT              |                                                                    |
| kb_KeyY              |                                                                    |
| kb_KeyU              |                                                                    |
| kb_KeyI              |                                                                    |
| kb_KeyO              |                                                                    |
| kb_KeyP              |                                                                    |
| kb_LeftBracket       |                                                                    |
| kb_RightBracket      |                                                                    |
| kb_KeyA              |                                                                    |
| kb_KeyS              |                                                                    |
| kb_KeyD              |                                                                    |
| kb_KeyF              |                                                                    |
| kb_KeyG              |                                                                    |
| kb_KeyH              |                                                                    |
| kb_KeyJ              |                                                                    |
| kb_KeyK              |                                                                    |
| kb_KeyL              |                                                                    |
| kb_SemiColon         |                                                                    |
| kb_Quote             |                                                                    |
| kb_BackSlash         |                                                                    |
| kb_IntlBackslash     |                                                                    |
| kb_KeyZ              |                                                                    |
| kb_KeyX              |                                                                    |
| kb_KeyC              |                                                                    |
| kb_KeyV              |                                                                    |
| kb_KeyB              |                                                                    |
| kb_KeyN              |                                                                    |
| kb_KeyM              |                                                                    |
| kb_Comma             |                                                                    |
| kb_Dot               |                                                                    |
| kb_Slash             |                                                                    |
| kb_Insert            |                                                                    |
| kb_KpReturn          |                                                                    |
| kb_KpMinus           |                                                                    |
| kb_KpPlus            |                                                                    |
| kb_KpMultiply        |                                                                    |
| kb_KpDivide          |                                                                    |
| kb_Kp0               |                                                                    |
| kb_Kp1               |                                                                    |
| kb_Kp2               |                                                                    |
| kb_Kp3               |                                                                    |
| kb_Kp4               |                                                                    |
| kb_Kp5               |                                                                    |
| kb_Kp6               |                                                                    |
| kb_Kp7               |                                                                    |
| kb_Kp8               |                                                                    |
| kb_Kp9               |                                                                    |
| kb_KpDelete          |                                                                    |
| kb_Function          |                                                                    |
| kb_Unknown(`x`)      | Unknown keyboard event (`x` is a u32)                              |
| cb_north             | X on Switch, △ on Playstation, Y on Xbox                           |
| cb_south             | B on Switch, X on Playstation, A on Xbox                           |
| cb_east              | A on Switch, O on Playstation, B on Xbox                           |
| cb_west              | A on Switch, ■ on Playstation, X on Xbox                           |
| cb_r_trigger         | ZR on Switch, R2 on Playstation, RT on Xbox                        |
| cb_l_trigger         | ZL on Switch, L2 on Playstation, LT on Xbox                        |
| cb_r_bumper          | R on Switch, R1 on Playstation, RB on Xbox                         |
| cb_l_bumper          | L on Switch, L1 on Playstation, LB on Xbox                         |
| cb_l_stick_click     |                                                                    |
| cb_r_stick_click     |                                                                    |
| cb_dpad_north        |                                                                    |
| cb_dpad_south        |                                                                    |
| cb_dpad_west         |                                                                    |
| cb_dpad_east         |                                                                    |
| cb_face_top_left     | Minus on Switch, Share on Playstation, View on Xbox                |
| cb_face_bottom_left  | Screenshot on Switch, Screenshot on Playstation, Share on Xbox     |
| cb_face_top_right    | Plus on Switch, Touchpad Click on Playstation, Xbox Button on Xbox |
| cb_face_bottom_right | Home on Switch, Options on Playstation, Menu on Xbox               |
| unknown(x)           | Unknown controller event (`x` is a uint)                           |

### PosX and PosY

The location of this key on the screen

### Classes

A list of CSS classes applied to this key. The CSS classes are defined in the `layout.css` file.

Additionally, each key is given a `.pressed` class when the key is pressed.

### Controllers

Currently, the only supported controller is the Nintendo Switch Pro Controller. Set the "controller" key to "switch_pro" in layout.json to enable controller polling.

Due to control sticks being analog, they are seperately handled. Any key can access the custom properties `--l-stick-x`, `--l-stick-y`, `--r-stick-x`, and `--r-stick-y`. These values are all floats ranging from -1 to 1 (the exact maximum and minimum megnitude is almost always lower, as it depends on the exact hardware and configuration. I've found values typically won't surpass 0.75 / -0.75, but your mileage may vary). -1, -1 is at the top left. On most control sticks `sqrt(x * x + y * y)` will not exceed `sqrt(2)`.

## Styling

![Advanced CSS Example](https://raw.githubusercontent.com/surprisedpika/hubble/master/github-assets/3d.gif)

The `.global` class can be used to style the div which wraps around all keys (for example, setting the background colour).

If you wish to use any external files in the styling, those files must either be uploaded to the internet, or converted to use the [data URI scheme](https://en.wikipedia.org/wiki/Data_URI_scheme).

If you wish for an element that is not a key, for additional styling, you can pass an empty array to `keys`, or only pass classes that do not style `.pressed`.

The default styles for a key are as follows:

```css
.key {
  position: absolute;
  display: flex;
  margin: 0;
  left: var(--pos-x, 0);
  top: var(--pos-y, 0);
}
```

There are no additional default styles for a pressed key.

## Using Hubble

Download Hubble from the releases page and open it. You will be prompted to select a folder, this is the layouts folder containing a `layout.css` and `layout.json` file. If you wish to work from an example, there are examples in the [Example Layouts](https://github.com/surprisedpika/hubble/tree/master/Example%20Layouts) folder.
