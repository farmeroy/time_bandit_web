"use client";

import { useState, useEffect } from "react";
import DisplayTimer from "./DisplayTimer";

const Timer = () => {
  const [isTimerOn, setIsTimerOn] = useState(false);
  const [time, setTime] = useState(0);

  useEffect(() => {
    let interval = setInterval(() => {
      if (!isTimerOn) {
        return;
      }
      setTime((state) => (state += 1));
    }, 1000);
    return () => {
      clearInterval(interval);
    };
  }, [isTimerOn]);

  return (
    <div className="w-full flex flex-col">
      <DisplayTimer time={time} />
      <div>
        <button onClick={() => setIsTimerOn((state) => !state)}>
          {isTimerOn ? "Stop" : "Start"}
        </button>
        <button>Cancel</button>
      </div>
    </div>
  );
};

export default Timer;
