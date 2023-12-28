"use client";

import NewTaskModal from "@/components/NewTaskModal";
import TasksTable from "./OverviewTasks";
import { TaskWithEvents } from "@/app/bandit/dashboard/page";
import { useState } from "react";

const DashBoard = ({ tasks }: { tasks: TaskWithEvents[] }) => {
  const [tasksState, setTasksState] = useState(tasks);
  return (
    <>
      <NewTaskModal updateTasks={setTasksState} userID={6} />
      <TasksTable tasks={tasksState} />
    </>
  );
};

export default DashBoard;
