import { createRoot } from "react-dom/client";
import { App } from "./components";

import "./styles.scss";

const ROOT =
  document.body.querySelector(`#ROOT`) ??
  (() => {
    const newRoot = document.createElement(`div`);
    newRoot.setAttribute(`id`, `ROOT`);
    document.body.appendChild(newRoot);
    return newRoot;
  })();

createRoot(ROOT).render(App({}));
