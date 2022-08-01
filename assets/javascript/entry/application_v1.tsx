import * as React from "react";
import * as ReactDOM from "react-dom";

interface TestProps {
  a?: number;
  b?: number;
}

const Test: React.FC<TestProps> = ({ a = 3, b = 3 }) => {
  return (
    <h1>{ a + b }</h1>
  )
}

window.addEventListener("load", (e) => {
  const target = document.querySelector("[data-react-class=\"test-class\"");
  const props = JSON.parse(target.getAttribute("data-react-props"));

  ReactDOM.render(<Test {...props} />, target);
});
