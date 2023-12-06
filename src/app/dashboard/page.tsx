"use client";
import { useEffect, useState } from "react";

const getTasks = async () => {
  const url = `http://localhost:8080/tasks`;

  try {
    let res = await fetch(url, {
      method: "GET",
      mode: "cors",
      credentials: "include",
      headers: {
        "Access-Control-Allow-Credentials": "true",
      },
    });
    return await res.json();
  } catch (e) {
    console.log(`${e}`);
  }
};

const Dashboard = () => {
  const [tasks, setTasks] = useState(null);
  useEffect(() => {
    const fetchData = async () => {
      const userTasks = await getTasks();
      setTasks(userTasks);
    };
    fetchData();
  }, []);

  return <div>{JSON.stringify(tasks)}</div>;
};

export default Dashboard;
