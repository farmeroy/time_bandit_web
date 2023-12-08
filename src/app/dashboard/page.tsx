import { cookies } from "next/headers";
import { redirect } from "next/navigation";

const getTasks = async () => {
  const url = `http://localhost:8080/tasks`;
  const request = new Request(url);
  const cookieStore = cookies();
  const cookie = cookieStore.get("time_bandit_auth_token_v1");
  try {
    let res = await fetch(request, {
      method: "GET",
      mode: "cors",
      credentials: "include",
      headers: {
        "Access-Control-Allow-Credentials": "true",
        Cookie: `time_bandit_auth_token_v1=${cookie?.value || ""}`,
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
  }
};

const Dashboard = async () => {
  const tasks = await getTasks();
  if (!tasks) {
    redirect("/");
  }
  console.log({ tasks });
  return <div>{JSON.stringify(tasks)}</div>;
};

export default Dashboard;
