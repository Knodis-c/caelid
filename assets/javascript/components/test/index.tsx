import React, { ChangeEvent, useState } from "react";
import { passwordScore } from "caelid-wasm";

interface TestProps {
  a?: number;
  b?: number;
}

const Test: React.FC<TestProps> = ({ a = 3, b = 3 }) => {
  const [score, setScore] = useState<number>(0);

  const handleChange = (e: ChangeEvent) => {
    const { value: password } = e.target as HTMLInputElement;
    if (password.length > 0) {
      const score = passwordScore(password);
      setScore(score);
    }
  };

  return (
    <div>
      <input
        type="password"
        name="password"
        placeholder="password"
        onChange={handleChange}
      />
      <p>password score: {score}</p>
    </div>
  )
}

export default Test;
