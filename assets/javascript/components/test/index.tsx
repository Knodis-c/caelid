import React from "react";

interface TestProps {
  a?: number;
  b?: number;
}

const Test: React.FC<TestProps> = ({ a = 3, b = 3 }) => {
  return (
    <h1>{ a + b }</h1>
  )
}

export default Test;
