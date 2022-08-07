/* Inspired by the https://github.com/reactjs/react-rails#ujs approach. */

import { createRoot } from "react-dom/client";
import ApplicationError from "lib/application_error";

/**
 * Refer to `importComponents`.
 */
export const COMPONENTS_CACHE: Record<string, any> = {};

const CONTAINER_NAME_ATTR = "data-react-class";
const CONTAINER_PROP_ATTR = "data-react-props";

/**
 * Custom webpack context that imports every (.tsx|.ts|.jsx|.js) file from ../components
 * and sticks them in the COMPONENTS_CACHE `Record` object that follows the following schema:
 *
 *     some_component/index => components/some_component/index.tsx
 */
export function importComponents() {
  importAll(require.context("../components", true, /\.(ts|tsx|js|jsx)$/));
}

/**
 * Queries for all target containers and mounts their respective React components.
 */
export function mountComponents() {
  const targets = document.querySelectorAll(`[${CONTAINER_NAME_ATTR}]`);

  targets.forEach(target => {
    const key = target.getAttribute(CONTAINER_NAME_ATTR);
    const component = COMPONENTS_CACHE[key];

    if (component == undefined) {
      throw new ApplicationError(`Unable to find component: "${key}.tsx" in the "components/" directory.`);
    }

    const rawProps = target.getAttribute(CONTAINER_PROP_ATTR);
    const container = createRoot(target);

    if (rawProps) {
      const props = JSON.parse(rawProps);
      container.render(component.default(props));
    } else {
      container.render(component.default());
    }
  });
}

function importAll(reqCtx: __WebpackModuleApi.RequireContext) {
  reqCtx.keys().forEach((key) => {
    const k = key.replace(/(?:\.\/|\.tsx|\.ts|\.jsx|\.js)/g, "");
    COMPONENTS_CACHE[k] = reqCtx(key);
  });
}
