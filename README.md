# IMGUI vs RMG

This repo is to guide an informal presentation I'm giving to my fellow frontend
coworkers about Immediate Mode GUI (IMGUI) vs Retained Mode GUI paradigms (RMG).

## Running Examples

You must have rust installed on your machine. There are two separate "crates":
egui_app (the IMGUI demo) and iced_app (the RMG demo). From the root of either
app, run with:

```sh
cargo run --release
```

There is a also a 3rd project showcasing how one might build an IMGUI with HTML
canvas in the ./canvas folder. You can simply open
[./canvas/index.html](./canvas/index.html) in your browser to see this.

# IMGUI

IMGUI works by reconstructing the UI every frame. The UI elements are not stored
as persistent objects; instead, the rendering and logic happen on-demand.

## Core Ideas

- The UI is a series of function calls made every frame.
- Each function handles rendering, input, and state storage in one pass.

### Breakdown

**1. Function Calls Per Frame**

```js
// Application state
let count = 0

// Called every frame
function run(ui) {
  // Add text to screen
  ui.text(count)

  // Add button to screen and check if it has been clicked
  if (ui.button('Increment').clicked()) {
    count++
  }
}
```

**2. State Management**

- The UI does _not_ store a tree of UI objects. Instead, state is managed
  externally.

**3. Rendering**

- The UI functions generate draw commands (e.g., drawRect, drawText) which are
  batched and sent to the GPU at the end of the frame

**4. Event Handling**

- The UI reads input state (mouse, keyboard) from the global input system each
  frame
- No event listeners; instead, UI functions check input state when called

### Pros

- No complex object lifetime management
- Easy to modify UI dynamically
- WOrks well for tools and debug interfaces

### Cons

- UI must be rebuilt every frame
- Cannot easily retain UI structure between frames

# RMG

RMG stores the UI as an object hierarchy that persists across frames. UI updates
only happen when necessary.

## Core Ideas

- The UI is defined as a **tree of objects**
- Objects retain state and are updated when properties change
- Rendering is often decoupled from logic, with a "dirty" ststem to avoid
  unnecessary redraws

### Breakdown

**1. Object Tree**

- UI elemnents are objects stored in memory

**2. State & Event Handling**

- Each object stores its state internally
- Event handlers (e.g onclick) modify internal state and trigger re-renders

```js
// Button lives in memory, decoupled from UI
const button = new Button('Click Me')

// Event is registered
button.onClick(() => {
  // do something
})

// Elsewhere, this button reference is added to UI
```

**3. Rendering**

- UI elements mark themselves as "dirty" when changed
- A render pass updates only dirty elements, reducing unnecessary redraws

**4. Event Propagation**

- Uses and event-driven system
- Events bubble up or down the UI tree, allowing structed event handling

#### Pros

- More effecient rendering when UI changes infrequently
- Easier to animate transitions
- Good for complex applications like games and productivity tools

#### Cons

- More overhead to manage UI objects
- Requires explicit state management and lifecycle handling

# Implementation Comparison: How to Detect Click?

## IMGUI Click Detection

```js
function Button(label) {
  const mousePosition = getMousePosition()
  const mousePressed = isMousedPressed()

  const buttonRect = computeBounds(label)
  const hovered = isPointInRect(mousePosition, buttonRect)

  return hovered && mousePressed
}

// Usage:
if (Button('Click Me')) {
  // button was clicked, do something
}
```

## RMG Click Detection

```js
class Button {
  label = ''
  hovered = false
  pressed = false

  // Note: this is not called every frame.
  // The framework's "manager" will perform a hit test
  // to determine if this Button is an event target
  // (mouse enters bounds or leaves bounds)
  handleMouseEvent(event) {
    if (event.type === 'mouseMove') {
      this.hovered = isPointInRect(event.position, this.bounds())
    } else if (event.type === 'mouseDown') {
      if (this.hovered) {
        this.pressed = true
      }
    } else if (event.type === 'mouseUp') {
      if (this.pressed && this.hovered) {
        this.onClick()
      }
      this.pressed = false
    }
  }

  // This would more likely be inherited from
  // a more generic "Element" class
  bounds() {
    // ...
  }

  onClick() {
    // ...
  }
}
```
