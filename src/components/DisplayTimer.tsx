"use client";
import { useState, useEffect } from "react";

const DisplayTimer = ({ time }: { time: number }) => {
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

export default DisplayTimer;
