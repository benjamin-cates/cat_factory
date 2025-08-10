import initTurbo, * as turbo from "./pkg/turbo_genesis_impl_wasm_bindgen.js";

/**************************************************/
/* BOOTLOADER IMAGE URLS                          */
/**************************************************/

const TURBO_LOGO_URL =
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAAASCAYAAACkctvyAAAAAXNSR0IArs4c6QAAARxJREFUWIXdWcsOxCAIrI3//8vsycbYzcLAiGTnjIOUx2DargkiIlcArbWmcc02ms9hi95r9WH1h3J5eFc+V4C/yC0fdIZmn1EUKM+MKOftPbwb1so7CUbh3lnV/w0s3xlY78q4u4hI2Q5goXqSe5Tg1KhAxJzNxUxq9wijx5HnnDVQix2TSwMi/OEO8CIaqHft83JZC+i1Zira+NIA74dhtmW1Dcha0Z57QyKcOX4qYaeQHxtBGqo8wKLQOqZbjf8VzG1KRARNfEoHIEFFx1XkPOtxhdibNSBjjlt8aDae1/gAci6KEQftJcxMEPNjsPd6Np4EnJ7/SJDsLkDASMbMYeqA3eOn8prKvtvKRxVh5AcKQ2x3VDqqQ9FYP+Ko6Ck9Nrm3AAAAAElFTkSuQmCC";
const LOADING_IMAGE_URL =
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACUAAAAFCAYAAADCMiU1AAAAAXNSR0IArs4c6QAAAIpJREFUKJGVkjEKAzEMBHchXe7gujQpUub/b8oDXLifNDowimUuCwJ5bFlrIwFoIkJXOIMyn+XV+tStMiRpP3PbXvHQLuk+4eO9zfax6idJU1OSZLtHQd76VOeLTx/1qoxFfRtd/jgHtggSf0ZkvgEPoI8858ABtLTXo/4N9OU8XJ2Rf2cqPyr3+wJvIvPpXApPMAAAAABJRU5ErkJggg==";
const CLICK_TO_PLAY_IMGAGE_URL =
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEQAAAAGCAYAAAB6gzjVAAAAAXNSR0IArs4c6QAAARtJREFUOI2VlDtuxDAMRB9ci8iWLrZIlRTJfXwAu3Plw/gshvYIOUEOkD4Gkp5ppAWt1cchIMDkcIajDwyAhsBEmudqLV5OI+We0fwPt9bf2gtAp6o7ICLCvu+pYG6ABoEqL+IFDQKGiFhNjZqtQxER+r5veq5oSuQevHrvFdgBt23ba/iO9YO4rdV43ntV1UvES5syei7Nc/MtL655nl9anu1K8DiH6LUDcM49qSpd130C76UNpFHiDcPA7Xb7XpblDfg9IXXvcc7ReBwAjOPINE0/67p+AM9neitzFbjcc3N6DviyNXOCDzdQ4sUBBstG7UYDr/hCAnYN68AteL62Xn0aOUAz9VJPWiv15+ae8XKGZ7klD8X/IsAf/YRX5UHT7BoAAAAASUVORK5CYII=";
const ERROR_IMAGE_URL =
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABsAAAAFCAYAAAC0CJe+AAAAAXNSR0IArs4c6QAAAGxJREFUKJGNjkEKgDAMBKfgzRZ8gA/o/1/Wg/f1EjW2Gg0UlumQbBIImwQJ4IlF3P9F/mS5ME4BZkFLsETclpYjd0VOH4EETdB8o4PJHYr4W775BqqgdtIq2AT5J8/2+mOXb0BRu6/sFg/M5x2RCGV7WoVq5QAAAABJRU5ErkJggg==";
const PLEASE_TRY_AGAIN_IMAGE_URL =
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAFUAAAAFCAYAAAA0e6CtAAAAAXNSR0IArs4c6QAAANxJREFUOI2lkl0OwyAMg0Ha21qpB+AAvf+ZdoBK6/u3hwYpi2LYDxJqMI5toBWg2Ki11l4D+HXHFDdiiq80fu2LObPcmfY/Z/Memf/N6rWUcgeOWuumAjluDLMmZkfkKg2x/0x4Wc61ryeeo6xkfYqf8N7vjWscNjePZyadK/px+Ba5SmNwAdOco7yj/CEr9v30bAROf9Brz8AGtFlI4+7AHrBm05stDn+MNNQFJN4NOIFlxleaIutpeXfg/ICfPUj/iZBhcGOGqTpyR3jmr9bqwUd6Mw2V9cu6AbwAVsYUIXubCLIAAAAASUVORK5CYII=";

/**************************************************/
/* GAMEPAD SUPPORT                                */
/**************************************************/

class GamepadManager {
  constructor(canvas) {
    this.canvas = canvas;
    this.gamepads = {};
    this.prevButtonStates = {};
    this.axisStates = {
      ArrowUp: false,
      ArrowDown: false,
      ArrowLeft: false,
      ArrowRight: false,
    };
    this.init();
  }

  init() {
    window.addEventListener("gamepadconnected", (e) =>
      this.onGamepadConnected(e)
    );
    window.addEventListener("gamepaddisconnected", (e) =>
      this.onGamepadDisconnected(e)
    );
    this.poll();
  }

  onGamepadConnected(event) {
    const gamepad = event.gamepad;
    console.log(
      `Gamepad connected at index ${gamepad.index}: ${gamepad.id}. ${gamepad.buttons.length} buttons, ${gamepad.axes.length} axes.`
    );
    this.gamepads[gamepad.index] = gamepad;
    this.prevButtonStates[gamepad.index] = gamepad.buttons.map(
      (button) => button.pressed
    );
  }

  onGamepadDisconnected(event) {
    const gamepad = event.gamepad;
    console.log(
      `Gamepad disconnected from index ${gamepad.index}: ${gamepad.id}`
    );
    delete this.gamepads[gamepad.index];
    delete this.prevButtonStates[gamepad.index];
  }

  poll() {
    const connectedGamepads = navigator.getGamepads
      ? navigator.getGamepads()
      : navigator.webkitGetGamepads
      ? navigator.webkitGetGamepads()
      : [];

    for (let gp of connectedGamepads) {
      if (gp) {
        if (!this.gamepads[gp.index]) {
          this.onGamepadConnected({ gamepad: gp });
        } else {
          this.updateGamepadState(gp);
        }
      }
    }

    requestAnimationFrame(() => this.poll());
  }

  updateGamepadState(gamepad) {
    const prevStates = this.prevButtonStates[gamepad.index];
    gamepad.buttons.forEach((button, index) => {
      if (button.pressed !== prevStates[index]) {
        if (button.pressed) {
          this.dispatchButtonEvent(gamepad, index, "keydown");
        } else {
          this.dispatchButtonEvent(gamepad, index, "keyup");
        }
        this.prevButtonStates[gamepad.index][index] = button.pressed;
      }
    });

    // Handle axes (e.g., left stick)
    this.handleAxes(gamepad);
  }

  dispatchButtonEvent(gamepad, buttonIndex, eventType) {
    let keyEvent;
    console.log(buttonIndex);
    switch (buttonIndex) {
      case 0: // A
        keyEvent = new KeyboardEvent(eventType, { key: "z", code: "KeyZ" });
        break;
      case 1: // B
        keyEvent = new KeyboardEvent(eventType, { key: "x", code: "KeyX" });
        break;
      case 2: // X
        keyEvent = new KeyboardEvent(eventType, { key: "c", code: "KeyC" });
        break;
      case 3: // Y
        keyEvent = new KeyboardEvent(eventType, { key: "v", code: "KeyV" });
        break;
      case 8: // Select
        keyEvent = new KeyboardEvent(eventType, {
          key: "Enter",
          code: "Enter",
        });
        break;
      case 9: // Start
        keyEvent = new KeyboardEvent(eventType, {
          key: " ",
          code: "Space",
        });
        break;
      case 12: // D-pad Up
        keyEvent = new KeyboardEvent(eventType, {
          key: "ArrowUp",
          code: "ArrowUp",
        });
        break;
      case 13: // D-pad Down
        keyEvent = new KeyboardEvent(eventType, {
          key: "ArrowDown",
          code: "ArrowDown",
        });
        break;
      case 14: // D-pad Left
        keyEvent = new KeyboardEvent(eventType, {
          key: "ArrowLeft",
          code: "ArrowLeft",
        });
        break;
      case 15: // D-pad Right
        keyEvent = new KeyboardEvent(eventType, {
          key: "ArrowRight",
          code: "ArrowRight",
        });
        break;
      // Add more mappings as needed
      default:
        return; // Unmapped button
    }

    this.canvas.dispatchEvent(keyEvent);
  }

  handleAxes(gamepad) {
    const threshold = 0.5;
    // Example: Left Stick Horizontal (axes[0]), Vertical (axes[1])
    const x = gamepad.axes[0];
    const y = gamepad.axes[1];

    // Horizontal
    if (x > threshold) {
      if (!this.axisStates.ArrowRight) {
        this.dispatchAxisEvent("ArrowRight", "keydown");
        this.axisStates.ArrowRight = true;
      }
    } else {
      if (this.axisStates.ArrowRight) {
        this.dispatchAxisEvent("ArrowRight", "keyup");
        this.axisStates.ArrowRight = false;
      }
    }

    if (x < -threshold) {
      if (!this.axisStates.ArrowLeft) {
        this.dispatchAxisEvent("ArrowLeft", "keydown");
        this.axisStates.ArrowLeft = true;
      }
    } else {
      if (this.axisStates.ArrowLeft) {
        this.dispatchAxisEvent("ArrowLeft", "keyup");
        this.axisStates.ArrowLeft = false;
      }
    }

    // Vertical
    if (y > threshold) {
      if (!this.axisStates.ArrowDown) {
        this.dispatchAxisEvent("ArrowDown", "keydown");
        this.axisStates.ArrowDown = true;
      }
    } else {
      if (this.axisStates.ArrowDown) {
        this.dispatchAxisEvent("ArrowDown", "keyup");
        this.axisStates.ArrowDown = false;
      }
    }

    if (y < -threshold) {
      if (!this.axisStates.ArrowUp) {
        this.dispatchAxisEvent("ArrowUp", "keydown");
        this.axisStates.ArrowUp = true;
      }
    } else {
      if (this.axisStates.ArrowUp) {
        this.dispatchAxisEvent("ArrowUp", "keyup");
        this.axisStates.ArrowUp = false;
      }
    }
  }

  dispatchAxisEvent(key, eventType) {
    const event = new KeyboardEvent(eventType, { key: key, code: key });
    this.canvas.dispatchEvent(event);
  }
}

class TouchGamepad {
  constructor() {
    this.buttons = new Array(16)
      .fill(null)
      .map(() => ({ pressed: false, value: 0 }));
    this.axes = [0, 0, 0, 0]; // Left stick X, Y, Right stick X, Y
    this.connected = true;
    this.id = "Touch Gamepad";
    this.index = 0;
    this.mapping = "standard";
    this.timestamp = performance.now();

    this.listeners = [];
    this.activeButtons = new Set();

    this.init();
  }

  static init() {
    new TouchGamepad();
  }

  init() {
    const overlay = document.getElementById("touch-gamepad-overlay");

    // Toggle visibility on touch
    document.body.addEventListener("touchstart", () => {
      overlay.classList.remove("hidden");
    });

    // Add touch event listeners to all buttons
    const buttons = overlay.querySelectorAll("[data-button]");
    this.buttonElements = {};
    this.isTracking = false;
    this.dpadThreshold = 60; // pixels - distance threshold for diagonal presses
    this.dpadMinDistance = 40; // pixels - minimum distance from center to activate multiple buttons

    buttons.forEach((button) => {
      const buttonIndex = parseInt(button.dataset.button);
      this.buttonElements[buttonIndex] = button;

      // Touch events
      button.addEventListener("touchstart", (e) => {
        e.preventDefault();
        this.pressButton(buttonIndex);
        button.classList.add("pressed");
      });

      button.addEventListener("touchend", (e) => {
        e.preventDefault();
        this.releaseButton(buttonIndex);
        button.classList.remove("pressed");
      });

      button.addEventListener("touchcancel", (e) => {
        e.preventDefault();
        this.releaseButton(buttonIndex);
        button.classList.remove("pressed");
      });

      // Mouse events for desktop testing
      button.addEventListener("mousedown", (e) => {
        e.preventDefault();
        this.pressButton(buttonIndex);
        button.classList.add("pressed");
      });

      button.addEventListener("mouseup", (e) => {
        e.preventDefault();
        this.releaseButton(buttonIndex);
        button.classList.remove("pressed");
      });

      button.addEventListener("mouseleave", (e) => {
        this.releaseButton(buttonIndex);
        button.classList.remove("pressed");
      });

      // Handle mouse enter for gliding
      button.addEventListener("mouseenter", (e) => {
        if (e.buttons === 1) {
          // Left mouse button is down
          this.pressButton(buttonIndex);
          button.classList.add("pressed");
        }
      });
    });

    // Helper function to get button center point
    this.getButtonCenter = (button) => {
      const rect = button.getBoundingClientRect();
      return {
        x: rect.left + rect.width / 2,
        y: rect.top + rect.height / 2,
      };
    };

    // Helper function to get D-pad center point
    this.getDpadCenter = () => {
      const dpadContainer = document.querySelector(".dpad-container");
      const rect = dpadContainer.getBoundingClientRect();
      return {
        x: rect.left + rect.width / 2,
        y: rect.top + rect.height / 2,
      };
    };

    // Helper function to get distance between two points
    this.getDistance = (p1, p2) => {
      return Math.sqrt(Math.pow(p2.x - p1.x, 2) + Math.pow(p2.y - p1.y, 2));
    };

    // Helper function to check for nearby D-pad buttons with smart minimum distance
    this.checkDpadProximity = (touchX, touchY) => {
      const dpadButtons = [12, 13, 14, 15]; // up, down, left, right
      const buttonDistances = [];
      const dpadCenter = this.getDpadCenter();

      // Get distances to all D-pad buttons
      dpadButtons.forEach((buttonIndex) => {
        const button = this.buttonElements[buttonIndex];
        if (button) {
          const center = this.getButtonCenter(button);
          const distance = this.getDistance({ x: touchX, y: touchY }, center);

          if (distance <= this.dpadThreshold) {
            buttonDistances.push({ buttonIndex, distance });
          }
        }
      });

      // Sort by distance (closest first)
      buttonDistances.sort((a, b) => a.distance - b.distance);

      if (buttonDistances.length === 0) {
        return [];
      }

      // Always include the closest button
      const result = [buttonDistances[0].buttonIndex];

      // Check if we're far enough from center to allow multiple buttons
      const distanceFromCenter = this.getDistance(
        { x: touchX, y: touchY },
        dpadCenter
      );
      if (distanceFromCenter >= this.dpadMinDistance) {
        // Include other buttons within threshold
        for (let i = 1; i < buttonDistances.length; i++) {
          result.push(buttonDistances[i].buttonIndex);
        }
      }

      return result;
    };

    // Helper function to check non-dpad buttons
    this.checkNonDpadButtons = (touchX, touchY) => {
      const nonDpadButtons = [0, 1, 2, 3, 8, 9]; // A, B, X, Y, Select, Start
      const elementAtPoint = document.elementFromPoint(touchX, touchY);

      if (elementAtPoint && elementAtPoint.dataset.button) {
        const buttonIndex = parseInt(elementAtPoint.dataset.button);
        if (nonDpadButtons.includes(buttonIndex)) {
          return [buttonIndex];
        }
      }

      return [];
    };

    // Global touch tracking
    document.addEventListener("touchstart", (e) => {
      this.isTracking = true;
    });

    document.addEventListener("touchend", (e) => {
      // Only stop tracking if no touches remain
      if (e.touches.length === 0) {
        this.isTracking = false;
        // Release all buttons
        this.activeButtons.forEach((buttonIndex) => {
          const button = this.buttonElements[buttonIndex];
          this.releaseButton(buttonIndex);
          button.classList.remove("pressed");
        });
      }
    });

    // Global touch move handler for gliding over buttons
    document.addEventListener("touchmove", (e) => {
      if (!this.isTracking) return;

      e.preventDefault();

      const currentlyPressed = new Set();

      // Get all active touches
      for (let i = 0; i < e.touches.length; i++) {
        const touch = e.touches[i];

        // Check D-pad proximity
        const nearbyDpadButtons = this.checkDpadProximity(
          touch.clientX,
          touch.clientY
        );
        nearbyDpadButtons.forEach((buttonIndex) => {
          currentlyPressed.add(buttonIndex);

          if (!this.activeButtons.has(buttonIndex)) {
            this.pressButton(buttonIndex);
            this.buttonElements[buttonIndex].classList.add("pressed");
          }
        });

        // Check non-D-pad buttons
        const nonDpadButtons = this.checkNonDpadButtons(
          touch.clientX,
          touch.clientY
        );
        nonDpadButtons.forEach((buttonIndex) => {
          currentlyPressed.add(buttonIndex);

          if (!this.activeButtons.has(buttonIndex)) {
            this.pressButton(buttonIndex);
            this.buttonElements[buttonIndex].classList.add("pressed");
          }
        });
      }

      // Release buttons that are no longer being touched or in proximity
      this.activeButtons.forEach((buttonIndex) => {
        if (!currentlyPressed.has(buttonIndex)) {
          this.releaseButton(buttonIndex);
          this.buttonElements[buttonIndex].classList.remove("pressed");
        }
      });
    });

    // Global mouse move handler for gliding over buttons
    overlay.addEventListener("mousemove", (e) => {
      if (e.buttons === 1) {
        // Left mouse button is down
        // Check D-pad proximity
        const nearbyDpadButtons = this.checkDpadProximity(e.clientX, e.clientY);
        nearbyDpadButtons.forEach((buttonIndex) => {
          if (!this.activeButtons.has(buttonIndex)) {
            this.pressButton(buttonIndex);
            this.buttonElements[buttonIndex].classList.add("pressed");
          }
        });

        // Check non-D-pad buttons
        const nonDpadButtons = this.checkNonDpadButtons(e.clientX, e.clientY);
        nonDpadButtons.forEach((buttonIndex) => {
          if (!this.activeButtons.has(buttonIndex)) {
            this.pressButton(buttonIndex);
            this.buttonElements[buttonIndex].classList.add("pressed");
          }
        });
      }
    });

    // Global mouse up to release all buttons
    document.addEventListener("mouseup", () => {
      this.activeButtons.forEach((buttonIndex) => {
        const button = this.buttonElements[buttonIndex];
        this.releaseButton(buttonIndex);
        button.classList.remove("pressed");
      });
    });

    // Override navigator.getGamepads to include our virtual gamepad
    this.originalGetGamepads = navigator.getGamepads.bind(navigator);
    navigator.getGamepads = () => {
      const gamepads = this.originalGetGamepads();
      const result = Array.from(gamepads);
      result[0] = this; // Place our virtual gamepad at index 0
      return result;
    };

    // Dispatch gamepad events
    this.dispatchConnectedEvent();
  }

  pressButton(index) {
    if (!this.activeButtons.has(index)) {
      this.activeButtons.add(index);
      this.buttons[index].pressed = true;
      this.buttons[index].value = 1;
      this.timestamp = performance.now();
      this.dispatchButtonEvent("gamepadbuttondown", index);
    }
  }

  releaseButton(index) {
    if (this.activeButtons.has(index)) {
      this.activeButtons.delete(index);
      this.buttons[index].pressed = false;
      this.buttons[index].value = 0;
      this.timestamp = performance.now();
      this.dispatchButtonEvent("gamepadbuttonup", index);
    }
  }

  dispatchButtonEvent(type, buttonIndex) {
    const event = new CustomEvent(type, {
      detail: {
        gamepad: this,
        button: buttonIndex,
      },
    });
    window.dispatchEvent(event);
  }

  dispatchConnectedEvent() {
    const event = new CustomEvent("gamepadconnected", {
      detail: { gamepad: this },
    });
    window.dispatchEvent(event);
  }

  // Standard gamepad API methods
  vibrate(value) {
    // Not implemented for virtual gamepad
    return Promise.resolve();
  }
}

/**************************************************/
/* WASM IMPORT PROXY                              */
/**************************************************/

// This proxy prevents WebAssembly.LinkingError from being thrown
// prettier-ignore
window.createWasmImportsProxy = (target = {}) => {
  console.log(target);
  return new Proxy(target, {
    get: (target, namespace) => {
      // Stub each undefined namespace with a Proxy
      target[namespace] = target[namespace] ?? new Proxy({}, {
        get: (_, prop) => {
          // Generate a sub function for any accessed property
          return (...args) => {
            console.log(`Calling ${namespace}.${prop} with arguments:`, args);
            // Implement the actual function logic here
          };
        }
      });
      return target[namespace];
    }
  })
};

/**************************************************/
/* GLOBAL STUBS                                   */
/**************************************************/

window.turboSolUser = window.turboSolUser ?? (() => null);
window.turboSolGetAccount = window.turboSolGetAccount ?? (async () => {});
window.turboSolSignAndSendTransaction =
  window.turboSolSignAndSendTransaction ?? (async () => {});

/**************************************************/
/* FETCH WITH PROGRESS UTIL                       */
/**************************************************/

async function fetchWithProgress(init, cb = () => {}) {
  const res = await fetch(init);
  const contentEncoding = res.headers.get("content-encoding");
  const contentLength =
    res.headers.get("content-length") ??
    res.headers.get("x-goog-stored-content-length");
  // If there's no content-length or if the response is encoded,
  // we can't trust preallocation.
  if (!contentLength || contentEncoding) {
    let loaded = 0;
    const chunks = [];
    const reader = res.body.getReader();
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      loaded += value.length;
      cb(loaded);
      chunks.push(value);
    }
    // Combine the chunks into one Uint8Array.
    const total = chunks.reduce((acc, chunk) => acc + chunk.length, 0);
    const body = new Uint8Array(total);
    let offset = 0;
    for (const chunk of chunks) {
      body.set(chunk, offset);
      offset += chunk.length;
      cb(offset, total);
    }
    return body;
  }
  // When content-length is available and there's no content-encoding,
  // preallocate the buffer.
  const total = parseInt(contentLength, 10);
  let loaded = 0;
  const body = new Uint8Array(new ArrayBuffer(total));
  const reader = res.body.getReader();
  while (true) {
    const { done, value: chunk } = await reader.read();
    if (done) break;
    body.set(chunk, loaded);
    loaded += chunk.length;
    cb(loaded, total);
  }
  return body;
}

/**************************************************/
/* WAIT FOR USER INTERACTION                      */
/**************************************************/

function waitForUserInteraction() {
  return new Promise((resolve) => {
    const events = ["click", "touchstart", "keydown"];
    const handler = (event) => {
      events.forEach((e) => window.removeEventListener(e, handler));
      resolve(event);
    };
    events.forEach((e) => window.addEventListener(e, handler));
  });
}

//**************************************************/
/* IMAGE UTILS                                     */
/***************************************************/

function loadImage(src) {
  return new Promise((resolve) => {
    const img = new Image();
    img.src = src;
    img.onload = () => resolve(img);
  });
}

function getAspectRatio(a, b) {
  return a / b;
}

function center(screen, item) {
  return Math.floor((screen - item) / 2);
}

//**************************************************/
/* RUN TURBO GAME                                  */
/***************************************************/

async function run() {
  // Disable double-tap zoom on mobile
  document.addEventListener("dblclick", (e) => e.preventDefault());

  // Initialize the touch gamepad
  const isTouchGamepadOverlayEnabled = false;
  if (isTouchGamepadOverlayEnabled) {
    TouchGamepad.init();
  }

  // Initialize a temporary 2D context canvas for loading state.
  const loadingCanvas = document.createElement("canvas");
  const player = document.getElementById("player");
  player?.appendChild(loadingCanvas);
  const ctx = loadingCanvas.getContext("2d");

  // Simple function to draw loading bar progress
  function drawProgressBar(progress) {
    // Progress fill
    if (progress > 0) {
      ctx.fillStyle = "#007BFF";
      ctx.fillRect(barX, barY, Math.floor(barWidth * progress), barHeight);
    }
  }

  // Set canvas size
  loadingCanvas.width = 387;
  loadingCanvas.height = 387;
  const screenWidth = loadingCanvas.width;
  const screenHeight = loadingCanvas.height;

  // Update background color
  loadingCanvas.style.backgroundColor = "#000000";
  ctx.fillStyle = "#000000";
  ctx.fillRect(0, 0, screenWidth, screenHeight);

  // Draw TURBO logo
  const turboLogoImage = await loadImage(TURBO_LOGO_URL);
  const logoWidth = turboLogoImage.width;
  const logoHeight = turboLogoImage.height;
  const logoX = center(screenWidth, logoWidth);
  const logoY = center(screenHeight, logoHeight);
  ctx.drawImage(turboLogoImage, logoX, logoY, logoWidth, logoHeight);

  // Draw "LOADING" text
  const loadingImage = await loadImage(LOADING_IMAGE_URL);
  const loadingWidth = loadingImage.width;
  const loadingHeight = loadingImage.height;
  const loadingX = center(screenWidth, loadingWidth);
  const loadingY = logoY + 52;
  ctx.drawImage(loadingImage, loadingX, loadingY);

  // Prograss bar settings
  const barWidth = logoWidth;
  const barHeight = 2;
  const barX = center(screenWidth, barWidth);
  const barY = loadingY + loadingHeight + 8;
  let progress = 0;

  // Update progress bar to 0%
  drawProgressBar(progress / 100);

  // Download Turbo's WASM runtime.
  console.log("Loading runtime...");
  const runtime = await fetchWithProgress(
    "pkg/turbo_genesis_impl_wasm_bindgen_bg.wasm",
    (loaded, total) => {
      if (!total) return;
      // Update progress bar up to 0-50%
      progress = (loaded / total) * 50;
      drawProgressBar(progress / 100);
    }
  );

  // Update progress bar to 75%
  progress = 75;
  drawProgressBar(progress / 100);

  // Initalize Turbo's WASM runtime.
  console.log("Initializing runtime...");
  await initTurbo({
    module_or_path: runtime.buffer,
  });

  // Update progress bar to 80%
  progress = 80;
  drawProgressBar(progress / 100);

  // Fetch Turbo File.
  console.log("Loading game data...");
  const turbofile = await fetchWithProgress("main.turbo", (loaded, total) => {
    if (!total) return;
    // Update progress bar to 80-85%
    progress = Math.max(80, (loaded / total) * 85);
    drawProgressBar(progress / 100);
  });

  // Update progress bar to 90%
  progress = 90;
  drawProgressBar(progress / 100);

  // Decode Turbo File contents.
  console.log(`Decompressing game data...`);
  const contents = turbo.decode_turbofile_v0_contents(
    new Uint8Array(turbofile)
  );

  // Update progress bar to 93%
  progress = 93;
  drawProgressBar(progress / 100);

  // Initialize context.
  const canvas = document.createElement("canvas");
  canvas.width = 512;
  canvas.height = 288;

  // Update progress bar to 96%
  progress = 96;
  drawProgressBar(progress / 100);

  // Initialize Gamepad Support.
  console.log("Initializing gamepad...");
  const gamepadManager = new GamepadManager(canvas);
  gamepadManager.poll();

  // Update progress bar to 99%
  progress = 99;
  drawProgressBar(progress / 100);

  // Display prompt to click if game has audio.
  // User interaction is required to play back audio without errors.
  if (contents.has_audio()) {
    const clickToPlayImage = await loadImage(CLICK_TO_PLAY_IMGAGE_URL);
    const clickWidth = clickToPlayImage.width;
    const clickHeight = clickToPlayImage.height;
    const clickX = center(screenWidth, clickWidth);
    const clickY = logoY + 52;
    ctx.fillStyle = "#5200FF";
    ctx.fillRect(0, 0, screenWidth, screenHeight);
    ctx.drawImage(turboLogoImage, logoX, logoY, logoWidth, logoHeight);
    ctx.drawImage(clickToPlayImage, clickX, clickY, clickWidth, clickHeight);
    loadingCanvas.style.backgroundColor = "#5200FF";
    await waitForUserInteraction();
  }

  // Append game canvas.
  player?.removeChild(loadingCanvas);
  player?.appendChild(canvas);

  // Set up turboGameEvent listener.
  window.addEventListener("turboGameEvent", (e) => {
    console.log(e.detail);
  });

  // Run game.
  await turbo.run(canvas, contents);
}

try {
  await run();
} catch (err) {
  console.error("Turbo failed to initialize", err);

  // Clear the screen
  const loadingCanvas = document.querySelector("canvas");
  const ctx = loadingCanvas?.getContext("2d");
  if (ctx) {
    const screenWidth = loadingCanvas.width;
    const screenHeight = loadingCanvas.height;

    // Update background color
    ctx.fillStyle = "#000000";
    ctx.fillRect(0, 0, screenWidth, screenHeight);
    loadingCanvas.style.backgroundColor = "#000000";

    // Re-render logo
    const turboLogoImage = await loadImage(TURBO_LOGO_URL);
    const logoWidth = turboLogoImage.width;
    const logoHeight = turboLogoImage.height;
    const logoX = center(screenWidth, logoWidth);
    const logoY = center(screenHeight, logoHeight);
    ctx.drawImage(turboLogoImage, logoX, logoY, logoWidth, logoHeight);

    // Load images
    const [errorImage, pleaseTryAgainImage] = await Promise.all([
      loadImage(ERROR_IMAGE_URL),
      loadImage(PLEASE_TRY_AGAIN_IMAGE_URL),
    ]);

    // Draw "error" smaller and centered, just below logo
    const errorWidth = errorImage.width;
    const errorHeight = errorImage.height;
    const errorX = center(screenWidth, errorWidth);
    const errorY = logoY + logoHeight + 8;
    ctx.drawImage(errorImage, errorX, errorY, errorWidth, errorHeight);

    // Draw pleaseTryAgainImage smaller and centered, 85% of the way down
    const ptaHeight = pleaseTryAgainImage.height;
    const ptaWidth = pleaseTryAgainImage.width;
    const ptaX = center(screenWidth, ptaWidth);
    const ptaY = logoY + 52;
    ctx.drawImage(pleaseTryAgainImage, ptaX, ptaY, ptaWidth, ptaHeight);
  }
}
