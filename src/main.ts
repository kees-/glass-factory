import { invoke } from "@tauri-apps/api/core";

let greetInputEl: HTMLInputElement | null;
let pregenCommandInputEl: HTMLInputElement | null;
let pregenCommandOutputEl: HTMLTextAreaElement | null;

async function contour() {
  if (greetInputEl && greetInputEl.value) {
    invoke("contour", {
      imgPath: greetInputEl.value,
    });
  }
}

async function generate_command() {
  if (pregenCommandInputEl && pregenCommandInputEl.value) {
    invoke("generate_command", {
      textFragment: pregenCommandInputEl.value,
    }).then((i) => (pregenCommandOutputEl.value = i));
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    contour();
  });

  pregenCommandInputEl = document.querySelector("#pregen-command-input");
  pregenCommandOutputEl = document.querySelector("#pregen-command-output");
  document
    .querySelector("#pregen-command-submit")
    ?.addEventListener("click", (e) => {
      e.preventDefault();
      generate_command();
    });
});
