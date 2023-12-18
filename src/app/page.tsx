import Link from "next/link";
import { redirect } from "next/navigation";
import { getSession } from "@/utils/Session";

export default async function Home() {
  // this isn't working because getSession doesn't return any kind of session object.
  // it returns the userID if there is a user, otherwise an empty object. The empty object returns true, redirects to bandit, and then there are more issues resulting from that.
  // const session = await getSession();

  // if (session) {
  //   redirect("/bandit");
  // }
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
