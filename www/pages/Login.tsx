import { Head, Link, usePage } from "@inertiajs/react";

type AuthUser = { id: string } | null;
type PageProps = { auth?: { user: AuthUser } };

export default function Login() {
  const { auth } = usePage<PageProps>().props;
  const user = auth?.user ?? null;

  return (
    <>
      <Head>
        <title>Login</title>
        <meta name="description" content="Demo login page using Inertia + Actix." />
      </Head>

      <main className="w-full h-full flex flex-col items-center justify-center px-6">
        <div className="w-full max-w-lg rounded-2xl bg-white/10 p-8 flex flex-col gap-6">
          <header className="text-center">
            <h1 className="text-4xl font-black mb-2">Authentication</h1>
            <p className="text-white/80">
              This page demonstrates logging in and out using actix-identity with Inertia.
            </p>
          </header>

          {user ? (
            <section className="space-y-4">
              <div className="rounded-xl bg-emerald-600/20 border border-emerald-400/30 p-4">
                <p className="text-lg">
                  You are logged in as
                  <span className="font-semibold"> {user.id}</span>.
                </p>
              </div>

              <div className="flex gap-3 flex-wrap">
                <Link
                  href="/"
                  className="px-5 py-3 rounded-lg bg-white/10 hover:bg-white/15 transition font-medium"
                >
                  Go to Home
                </Link>

                <Link
                  href="/logout"
                  method="post"
                  as="button"
                  className="
                    px-5 py-3 rounded-lg
                    bg-red-600 hover:bg-red-700 active:bg-red-800
                    transition font-semibold
                  "
                >
                  Log out
                </Link>
              </div>
            </section>
          ) : (
            <section className="space-y-4">
              <div className="rounded-xl bg-purple-600/20 border border-purple-400/30 p-4">
                <p className="text-lg">You are currently not logged in.</p>
              </div>

              <div className="flex gap-3 flex-wrap">
                <Link
                  href="/login"
                  method="post"
                  as="button"
                  className="
                    px-5 py-3 rounded-lg
                    bg-purple-700 hover:bg-purple-800 active:bg-purple-900
                    transition font-semibold
                  "
                >
                  Log in
                </Link>

                <Link
                  href="/"
                  className="px-5 py-3 rounded-lg bg-white/10 hover:bg-white/15 transition font-medium"
                >
                  Back to Home
                </Link>
              </div>
            </section>
          )}

          <footer className="pt-2 text-center">
            <Link
              href="/contact"
              className="text-purple-200 underline hover:text-purple-100 transition"
            >
              Contact
            </Link>
          </footer>
        </div>
      </main>
    </>
  );
}
