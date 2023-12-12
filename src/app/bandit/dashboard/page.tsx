import TasksTable from "@/features/dashboard/OverviewTasks";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";

export interface Task {
  id: number;
  uuid: string;
  user_id: number;
  name: string;
  description: string;
  created_on: Date;
}

const getTasks = async () => {
  const url = `http://localhost:8080/tasks`;
  const request = new Request(url);
  const cookieStore = cookies();
  try {
    const cookie = cookieStore.get("time_bandit_auth_token_v1");
    let res = await fetch(request, {
      method: "GET",
      mode: "cors",
      credentials: "include",
      headers: {
        "Access-Control-Allow-Credentials": "true",
        Cookie: `time_bandit_auth_token_v1=${cookie?.value}`,
      },
      cache: "force-cache",
    });
    if (res.ok) {
      return res.json();
    } else {
      const error = new Error("UNAUTHORIZED");
      throw error;
    }
  } catch (e) {
    JSON.stringify(e);
    console.error(e);
  }
};

const Dashboard = async () => {
  const tasks: Task[] = await getTasks();
  if (!tasks) {
    redirect("/");
  }
  return <TasksTable tasks={tasks} />;
};

export default Dashboard;
