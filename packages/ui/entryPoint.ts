import { createRoot } from "react-dom/client";
import { App } from "./components";

function ensureRoot() {
  const newRoot = document.createElement(`div`);
  newRoot.setAttribute(`id`, `ROOT`);
  document.body.appendChild(newRoot);
  return newRoot;
}

const ROOT = document.body.querySelector(`#ROOT`) ?? ensureRoot();

createRoot(ROOT).render(App({}));
