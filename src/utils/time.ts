export const formatTimer = (totalSeconds: number) => {
  let secs = totalSeconds % 60;
  let mins = Math.floor(totalSeconds / 60) % 60;
  let hrs = Math.floor(totalSeconds / 60 / 60);
  return `${hrs}:${mins > 9 ? mins : "0" + mins}:${
    secs > 9 ? secs : "0" + secs
  }`;
};
