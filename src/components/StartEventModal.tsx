"use client";

import { Task } from "@/app/bandit/dashboard/page";
import { Event as TaskEvent } from "@/features/tasks/TaskView";
import { useEffect, useState, Dispatch, SetStateAction } from "react";

interface StartEventModalProps {
  task: Task;
  onCancel: () => void;
  onConfirm: () => void;
  updateEvents: Dispatch<SetStateAction<TaskEvent[]>>;
}

interface NewEvent {
  user_id: number;
  task_id: number;
  date_began: Date;
  duration: number;
  notes: string;
}

const StartEventModal = ({
  task,
  onCancel,
  onConfirm,
  updateEvents,
}: StartEventModalProps) => {
  const [isTimerOn, setIsTimerOn] = useState(false);
  const [notes, setNotes] = useState("");
  // time should be context level
  // make a new event type at the context level
  // which runs on the nav bar as well,
  // or in a little window in the corner?
  const [time, setTime] = useState(0);
  const handleFinishEvent = async () => {
    const duration = time;
    setTime(0);
    setIsTimerOn(false);

    const url = `//localhost:8080/events/add_event`;
    const newEvent: NewEvent = {
      user_id: task.user_id,
      task_id: task.id,
      date_began: new Date(Date.now()),
      duration: duration,
      notes: notes,
    };
    try {
      const res = await fetch(url, {
        method: "POST",
        mode: "cors",
        headers: {
          "Content-Type": "application/json",
          "Access-Control-Allow-Credentials": "true",
          "Access-Control-Allow-Origin": `http:${url}`,
        },
        credentials: "include",
        body: JSON.stringify(newEvent),
      });
      if (res.ok) {
        const createdEvent = await res.json();
        updateEvents((state) => [...state, createdEvent]);
        onConfirm();
      }
    } catch (err) {
      console.error(err);
    }
  };

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
    <div className="flex flex-col justify-between">
      <p>{formatTimer(time)}</p>
      <p className="text-lg">
        Lets work on: <span className="text-accent">{task.name}</span>
      </p>
      <form
        onSubmit={(event) => event.preventDefault()}
        className="flex flex-col"
      >
        <textarea
          className="textarea textarea-primary"
          placeholder="Add any notes here"
          name="event-name"
          value={notes}
          onChange={(e) => setNotes(e.target.value)}
        />
        <div className="join py-2">
          {!isTimerOn ? (
            <button
              className="btn btn-accent join-item"
              onClick={() => setIsTimerOn(true)}
            >
              Start
            </button>
          ) : (
            <button
              className="btn btn-success join-item"
              onClick={handleFinishEvent}
            >
              Done
            </button>
          )}
          <button className="btn btn-neutral join-item" onClick={onCancel}>
            Cancel
          </button>
        </div>
      </form>
    </div>
  );
};

export default StartEventModal;
