"use client";
import { IconEdit } from "@tabler/icons-react";
import { Task } from "@/app/bandit/dashboard/page";
import { useState } from "react";
import TaskStart from "./TaskStart";

export interface Event {
  id: number;
  uuid: string;
  user_id: number;
  task_id: number;
  date_began: Date;
  duration: number;
  notes?: string;
}

const TaskView = ({ task, events }: { task: Task; events: Event[] }) => {
  const [editDescription, setEditDescription] = useState(false);
  const [eventsState, setEventsState] = useState(events);
  return (
    <div className="w-full flex flex-col items-center">
      <h2 className=" text-xl">Task: {task.name}</h2>
      <TaskStart task={task} updateEvents={setEventsState} />
      <div className="flex flex-wrap rounded-md m-2 p-2">
        <button
          className="mx-1"
          onClick={() => setEditDescription((state) => !state)}
        >
          <IconEdit />
        </button>
        <div className="mx-1">
          {editDescription ? (
            <textarea defaultValue={task.description}></textarea>
          ) : (
            <p>{task.description}</p>
          )}
        </div>
      </div>
      <div>
        {events.length < 1 ? (
          <p>No events to display</p>
        ) : (
          <div>
            {eventsState.map((event) => (
              <div key={event.id} className="flex">
                <p>{event.id}</p>
                <p>{event.duration}</p>
                <p>{event.notes}</p>
              </div>
            ))}
          </div>
        )}{" "}
      </div>
    </div>
  );
};

export default TaskView;
