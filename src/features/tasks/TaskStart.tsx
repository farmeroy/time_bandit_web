"use client";
import { Task } from "@/app/bandit/dashboard/page";
import StartEventModal from "@/components/StartEventModal";
import { TaskEvent } from "./TaskView";

const TaskStart = ({
  task,
  updateEvents,
}: {
  task: Task;
  updateEvents: (arg0: TaskEvent[]) => void;
}) => {
  // Start button opens a modal
  // this modal will already start a timer
  // and includes a text area to add notes
  // if you move away from this screen,
  // I would like to maintain a view on
  // the window (and accross the app)
  // that shows the timer and the name of the task
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
      <button
        className="m-1 btn btn-primary btn-sm focus:ring-1"
        onClick={() => openModal(`task-start-modal-${task.id}`)}
      >
        Begin
      </button>
      <dialog id={`task-start-modal-${task.id}`} className="modal">
        <div className="modal-box">
          <StartEventModal
            task={task}
            onCancel={() => closeModal(`task-start-modal-${task.id}`)}
            onConfirm={() => closeModal(`task-start-modal-${task.id}`)}
            updateEvents={updateEvents}
          />
        </div>
      </dialog>
    </div>
  );
};

export default TaskStart;
