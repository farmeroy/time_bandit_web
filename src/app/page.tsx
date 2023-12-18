import Link from "next/link";

export default async function Home() {
  return (
    <>
      <h1>Time Bandit</h1>
      <div className="w-[500px]">
        <p>
          Your freelancing activities, daily chores, and desperate attempts at
          self-improvement are robbing you of your time! <br /> Track those time
          bandits, catch them, and take a hold of your most precious asset.
        </p>
      </div>
      <div>
        <Link href="/login">Login</Link>
        <Link href="/register">Register</Link>
      </div>
    </>
  );
}
