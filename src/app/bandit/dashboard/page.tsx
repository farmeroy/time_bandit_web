import TasksTable from "@/features/dashboard/OverviewTasks";
import ClientWrapper from "@/components/ClientWrapper";
import { Suspense } from "react";

// Load data here and pass it to a client component... DUH!

const Dashboard = () => {
  return (
    <ClientWrapper>
      <Suspense>
        <TasksTable />
      </Suspense>
    </ClientWrapper>
  );
};

export default Dashboard;
