import Timer from "@/components/Timer";
import Login from "../components/login";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-around p-24">
      <h1>Time Bandit</h1>
      <p>
        Your freelancing activities, daily chores, and desperate attempts at
        self-improvement are robbing you of your time! Track those time bandits,
        catch them, and take a hold of your most precoius asset.
      </p>
      <div>
        <Timer />
        <Login />
      </div>
    </main>
  );
}
