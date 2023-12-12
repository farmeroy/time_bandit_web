import TaskView from "@/features/TaskView";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";

const getEvents = async (id: number) => {
  const url = `http://localhost:8080/tasks/${id}`;
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
    return e;
  }
};

const TaskPage = async ({ params }: { params: { id: number } }) => {
  const res = await getEvents(params.id);
  console.log({ res: res.status });
  if (!res) {
    redirect("/");
  }
  return <TaskView task={res.task} events={res.events} />;
};

export default TaskPage;
