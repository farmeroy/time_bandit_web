import { cookies } from "next/headers";
import { redirect } from "next/navigation";

interface Task {
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

const TasksTable = async () => {
  const tasks: Task[] = await getTasks();
  if (!tasks) {
    redirect("/");
  }
  return (
    <div>
      <h2>Tasks</h2>
      <table className="table table-pin-rows">
        <thead>
          <tr>
            <th>Name</th>
            <th>Created</th>
            <th>Description</th>
          </tr>
        </thead>
        <tbody>
          {tasks.map((task) => (
            <tr key={task.id}>
              <td>{task.name}</td>
              <td>{task.created_on.toString().substring(0, 10)}</td>
              <td>{task.description}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
  return <div>{JSON.stringify(tasks)}</div>;
};

export default TasksTable;
