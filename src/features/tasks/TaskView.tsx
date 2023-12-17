"use client";
import { IconEdit } from "@tabler/icons-react";
import { Task } from "@/app/bandit/dashboard/page";
import { useState } from "react";
import TaskStart from "./TaskStart";
import { formatTimer } from "@/utils/time";
import TaskEventLineChart from "./TaskEventsLineChart";

export interface TaskEvent {
  id: number;
  uuid: string;
  user_id: number;
  task_id: number;
  date_began: Date;
  duration: number;
  notes?: string;
}

const TaskView = ({ task, events }: { task: Task; events: TaskEvent[] }) => {
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
      <div className="w-full h-96 px-4">
        <TaskEventLineChart taskEvents={events} />
      </div>
      <div>
        {events.length < 1 ? (
          <p>No events to display</p>
        ) : (
          <>
            <div className="p-2">
              <table className="table">
                <thead>
                  <tr>
                    <th>Date</th>
                    <th>Duration</th>
                    <th>Notes</th>
                  </tr>
                </thead>
                <tbody>
                  {eventsState.map((event) => (
                    <tr key={event.id}>
                      <td>{new Date(event.date_began).toDateString()}</td>
                      <td>{formatTimer(event.duration)}</td>
                      <td>{event.notes}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </>
        )}
      </div>
    </div>
  );
};

export default TaskView;
