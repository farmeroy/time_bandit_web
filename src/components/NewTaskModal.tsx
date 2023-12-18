"use client";

import { useState, Dispatch, SetStateAction } from "react";

interface CreateNewTaskModalProps {
  userID: number;
  // updateTasks: Dispatch<SetStateAction<Task[]>>;
}

export interface NewTask {
  user_id: number;
  name: string;
  description: string;
}

const CreateNewTaskModal = ({
  userID, // updateTasks,
}: CreateNewTaskModalProps) => {
  const [notes, setNotes] = useState("");
  const [taskName, setTaskName] = useState("");

  const handleCreateTask = async () => {
    const url = `//localhost:8080/tasks/add_task`;
    const newTask: NewTask = {
      name: taskName,
      user_id: userID,
      description: notes,
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
        body: JSON.stringify(newTask),
      });
      if (res.ok) {
        const createdTask = await res.json();
        // updateTasks((state) => [...state, createdTask]);
        console.log("OK");
        closeModal(`create-task-modal`);
      }
    } catch (err) {
      console.error(err);
    }
  };
  const openModal = (id: string) => {
    const element = document.getElementById(id) as HTMLDialogElement | null;
    if (element != null) {
      element.showModal();
    }
  };

  const closeModal = (id: string) => {
    const element = document.getElementById(id) as HTMLDialogElement | null;
    if (element != null) {
      element.close();
    }
  };

  return (
    <div>
      <button onClick={() => openModal(`create-task-modal`)}>
        Create New Task
      </button>
      <dialog id={`create-task-modal`} className="modal">
        <div className="flex flex-col justify-between modal-box">
          <form
            onSubmit={(event) => {
              event.preventDefault();
              handleCreateTask();
            }}
            className="flex flex-col"
          >
            <label htmlFor="task-name">Name:</label>
            <input
              id="task-name"
              name="task-name"
              value={taskName}
              onChange={(event) => setTaskName(event.target.value)}
            />
            <textarea
              className="textarea textarea-primary"
              placeholder="Add any notes here"
              name="event-name"
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
            />
            <div className="join py-2">
              <button type="submit" className="btn btn-primary">
                Ok
              </button>
              <button
                type="button"
                className="btn btn-neutral join-item"
                onClick={() => closeModal(`create-task-modal`)}
              >
                Cancel
              </button>
            </div>
          </form>
        </div>
      </dialog>
    </div>
  );
};

export default CreateNewTaskModal;
