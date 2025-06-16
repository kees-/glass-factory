import { invoke } from "@tauri-apps/api/core";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function contour() {
  if (greetMsgEl && greetInputEl) {
    invoke("contour", {
      imgPath: greetInputEl.value,
    });
  }
}

// async function addDependencyEl(command: string) {
//   invoke("process_exists", { cmd: command });
// }

// (window as any).addDependencyEl = addDependencyEl;
// console.log("Hello");

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    contour();
  });
});
