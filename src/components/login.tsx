"use client";
import { useRouter } from "next/navigation";
import { useState } from "react";

const Login = () => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  const router = useRouter();

  const handleSubmit = async (e: React.SyntheticEvent) => {
    e.preventDefault();

    const url = `//localhost:8080/users/login`;

    try {
      let res = await fetch(url, {
        method: "POST",
        mode: "cors",
        headers: {
          "Content-Type": "application/json",
          "Access-Control-Allow-Credentials": "true",
          "Access-Control-Allow-Origin": `http:${url}`,
        },
        credentials: "include",
        body: JSON.stringify({
          email: email,
          password: password,
        }),
      });
      if (res.ok) {
        router.push("/dashboard");
      } else {
        console.log({ res });
      }
    } catch (err) {
      console.log(`Error: ${e}`);
    }
  };
  return (
    <form onSubmit={handleSubmit}>
      <div className="flex justify-between">
        <label htmlFor="email">Email</label>
        <input
          className="text-black"
          onChange={(e) => setEmail(e.target.value)}
          type="email"
          name="email"
          id="email"
          required
        />
      </div>
      <div className="flex justify-between">
        <label htmlFor="password">Password</label>
        <input
          className="text-black"
          onChange={(e) => setPassword(e.target.value)}
          type="password"
          name="password"
          id="password"
        />
      </div>
      <button type="submit">Submit</button>
    </form>
  );
};

export default Login;
