import { cookies } from "next/headers";
import { redirect } from "next/navigation";

export const getSession = async () => {
  const url = `http://localhost:8080/auth`;
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
    let res = JSON.stringify(e);
    console.error(e);
    return res;
  }
};

const Session = async ({ children }: { children: React.ReactNode }) => {
  const session = await getSession();
  console.log({ session });
  if (!session) {
    // session is undefined if there is an error
    redirect("/");
  }

  return <div>{children}</div>;
};

export default Session;
