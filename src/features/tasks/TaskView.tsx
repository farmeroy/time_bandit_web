"use client";
import { IconEdit } from "@tabler/icons-react";
import { Task } from "@/app/bandit/dashboard/page";
import { FormEvent, useState } from "react";
import TaskEventStart from "./TaskEventStart";
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

interface EditDescriptionFormElements extends HTMLFormControlsCollection {
  description: HTMLTextAreaElement;
}

interface EditDescriptionForm extends HTMLFormElement {
  readonly elements: EditDescriptionFormElements;
}

const TaskView = ({ task, events }: { task: Task; events: TaskEvent[] }) => {
  const [editDescription, setEditDescription] = useState(false);
  const [taskDescription, setTaskDescription] = useState(task.description);
  const [eventsState, setEventsState] = useState(events);

  const handleUpdateDescription = (event: FormEvent<EditDescriptionForm>) => {
    event.preventDefault();
    setTaskDescription(event.currentTarget.elements.description.value);
    setEditDescription(false);
  };
  return (
    <div className="w-full flex flex-col items-center">
      <h2 className=" text-xl">Task: {task.name}</h2>
      <TaskEventStart task={task} updateEvents={setEventsState} />
      <div className="flex flex-wrap rounded-md m-2 p-2">
        <div className="mx-1 flex">
          {editDescription ? (
            <form onSubmit={handleUpdateDescription}>
              <textarea
                name="description"
                defaultValue={taskDescription}
              ></textarea>
              <div>
                <button type="submit">Save</button>
                <button type="button" onClick={() => setEditDescription(false)}>
                  Cancel
                </button>
              </div>
            </form>
          ) : (
            <>
              <button className="mx-1" onClick={() => setEditDescription(true)}>
                <IconEdit />
              </button>
              <p>{taskDescription}</p>
            </>
          )}
        </div>
      </div>
      <div className="p-4">
        <h2>Event Logs</h2>
        {events.length < 1 ? (
          <p>No events to display</p>
        ) : (
          <>
            <div className="w-full lg:w-[800px] h-96">
              <TaskEventLineChart taskEvents={eventsState} />
            </div>
            <div>
              <table className="table ">
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
                      <td>{new Date(event.date_began).toLocaleDateString()}</td>
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
