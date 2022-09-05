/* Inspired by the https://github.com/reactjs/react-rails#ujs approach. */

import React from "react";
import ReactDOM from "react-dom/client";
import ApplicationError from "lib/application_error";

/**
 * Refer to `importComponents`.
 */
export const COMPONENTS_CACHE: Record<string, any> = {};

export const CONTAINER_NAME_ATTR = "data-react-class";
export const CONTAINER_PROP_ATTR = "data-react-props";

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
 * React components that utilize WASM modules are asynchronously mounted since WASM modules
 * must be asynchronously built.
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
    const container = ReactDOM.createRoot(target);

    if (rawProps) {
      const props = JSON.parse(rawProps);

      if (typeof component === "object" && typeof component.then === "function")
        // @ts-ignore
        component.then(c => container.render(React.createElement(c.default, props)));
      else
        container.render(React.createElement(component.default, props));

    } else {
      if (typeof component === "object" && typeof component.then === "function")
        // @ts-ignore
        component.then(c => container.render(React.createElement(c.default)));
      else
        container.render(React.createElement(component.default));
    }
  });
}

function importAll(reqCtx: __WebpackModuleApi.RequireContext) {
  reqCtx.keys().forEach((key) => {
    const k = key.replace(/(?:\.\/|\.tsx|\.ts|\.jsx|\.js)/g, "");
    COMPONENTS_CACHE[k] = reqCtx(key);
  });
}
