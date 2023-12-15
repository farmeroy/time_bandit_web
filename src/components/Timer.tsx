"use client";

import { useState, useEffect } from "react";

const Timer = ({
  isOn = false,
  startTime = 0,
}: {
  isOn: boolean;
  startTime: number;
}) => {
  // this should be a context level variable so it can be displayed
  // and accessed over the app
  const [time, setTime] = useState(startTime);

  useEffect(() => {
    let interval = setInterval(() => {
      if (!isOn) {
        setTime(0);
        return;
      }
      setTime((state) => (state += 1));
    }, 1000);
    return () => {
      clearInterval(interval);
    };
  }, [isOn]);

  // format time
  // divide time into hours minutes and seconds
  const formatTimer = (totalSeconds: number) => {
    let secs = totalSeconds % 60;
    let mins = Math.floor(totalSeconds / 60) % 60;
    let hrs = Math.floor(totalSeconds / 60 / 60);
    return `${hrs}:${mins > 9 ? mins : "0" + mins}:${
      secs > 9 ? secs : "0" + secs
    }`;
  };
  return (
    <div className="w-full flex flex-col">
      <p className="">{formatTimer(time)}</p>
    </div>
  );
};

export default Timer;
