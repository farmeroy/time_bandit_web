"use client";
import { TaskWithEvents } from "@/app/bandit/dashboard/page";
import Link from "next/link";

const TasksTable = ({ tasks }: { tasks: TaskWithEvents[] }) => {
  return (
    <div className="">
      <table className="table">
        <thead>
          <tr>
            <th></th>
            <th>Task Name</th>
            <th>Created</th>
            <th>Description</th>
          </tr>
        </thead>
        <tbody>
          {tasks.map((task) => (
            <tr key={task.task.id} className="hover">
              <td>
                <Link
                  href={`/bandit/tasks/${task.task.id}`}
                  className="btn btn-primary m-2"
                >
                  View Task
                </Link>
              </td>
              <td>{task.task.name}</td>
              <td>{task.task.created_on.toString().substring(0, 10)}</td>
              <td>{task.task.description}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default TasksTable;
