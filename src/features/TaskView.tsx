"use client";
import { IconEdit } from "@tabler/icons-react";
import { Task } from "@/app/bandit/dashboard/page";
import { useState } from "react";

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
  return (
    <div className="w-full flex flex-col">
      <h2 className="flex self-center text-xl">Task: {task.name}</h2>
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
            {events.map((event) => (
              <p key={event.id}>{event.id}</p>
            ))}
          </div>
        )}{" "}
      </div>
    </div>
  );
};

export default TaskView;
