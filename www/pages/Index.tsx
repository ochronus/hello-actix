import { Head, Link } from "@inertiajs/react";
import { useState } from "react";

type AuthUser = { id: string } | null;

type Props = {
  version: string;
  message: string;
  auth?: {
    user: AuthUser;
  };
};

export default function Index({ message, version, auth }: Props) {
  const user = auth?.user ?? null;

  const [count, setCount] = useState(0);
  const increment = () => setCount((prev) => ++prev);

  return (
    <>
      <Head>
        <title>Hello, from inertia-rust!</title>
        <meta name="description" content="Just a mocked head... Ha!" />
      </Head>

      <main className="w-full h-full flex flex-col items-center justify-center gap-8 px-6">
        {/* Auth status and actions */}
        <section className="w-full max-w-3xl rounded-2xl bg-white/10 p-6">
          <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
            <div>
              <h2 className="text-2xl font-black">Auth state</h2>
              {user ? (
                <p className="text-white/80 mt-1">
                  Logged in as <span className="font-semibold">{user.id}</span>
                </p>
              ) : (
                <p className="text-white/80 mt-1">You are not logged in.</p>
              )}
            </div>

            <div className="flex flex-wrap gap-3">
              {!user ? (
                <>
                  {/* Post to /login so the server attaches identity and redirects */}
                  <Link
                    href="/login"
                    method="post"
                    as="button"
                    className="
                      px-5 py-2 rounded-lg
                      bg-purple-700 hover:bg-purple-800 active:bg-purple-900
                      transition font-semibold
                    "
                  >
                    Log in
                  </Link>
                </>
              ) : (
                <>
                  {/* Post to /logout so the server clears identity and redirects */}
                  <Link
                    href="/logout"
                    method="post"
                    as="button"
                    className="
                      px-5 py-2 rounded-lg
                      bg-red-600 hover:bg-red-700 active:bg-red-800
                      transition font-semibold
                    "
                  >
                    Log out
                  </Link>
                </>
              )}
              <Link
                href="/contact"
                className="px-5 py-2 rounded-lg bg-white/10 hover:bg-white/15 transition font-medium"
              >
                Contact
              </Link>
            </div>
          </div>
        </section>

        {/* Demo content */}
        <section className="w-full max-w-3xl text-center">
          <h1 className="text-6xl font-black mb-3">
            Yeah!
            <br />
            Inertia-Rust v{version}
          </h1>
          <p className="text-xl font-medium mb-8">{message}</p>

          <div className="flex flex-col items-center gap-4 w-fit mx-auto">
            <div className="rounded-2xl bg-white/10 h-20 w-full grid place-items-center px-10">
              <span className="font-black text-4xl">{count}</span>
            </div>
            <button
              className="
                p-7 py-4 rounded-xl bg-purple-700 hover:bg-purple-800 active:bg-purple-900
                transition-all duration-100 ring-0 ring-purple-600/25 focus:ring-8 outline-none
                select-none font-medium text-xl cursor-default
              "
              onClick={increment}
            >
              Taste this state!
            </button>
          </div>
        </section>
      </main>
    </>
  );
}
