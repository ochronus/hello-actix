import { Head, Link } from "@inertiajs/react";

type Props = {
  user: {
    name: string;
    email: string;
  };
};

export default function Contact({ user }: Props) {
  return (
    <>
      <Head>
        <title>Contact</title>
        <meta
          name="description"
          content="A simple Contact page mirroring the actix_ssr example."
        />
      </Head>

      <main className="w-full h-full flex flex-col justify-center items-center px-6">
        <section className="w-full max-w-2xl rounded-2xl bg-white/10 p-8 shadow-lg space-y-6">
          <header className="text-center">
            <h1 className="text-5xl font-black tracking-tight mb-3">Contact</h1>
            <p className="text-white/80">
              This page mirrors the actix_ssr example and shows server-provided
              user details passed as Inertia props.
            </p>
          </header>

          <div
            className="
              grid gap-4 sm:grid-cols-2 bg-black/20 rounded-xl p-6
              border border-white/10
            "
          >
            <div className="space-y-1">
              <div className="text-sm uppercase tracking-wide text-white/60">
                Name
              </div>
              <div className="text-xl font-semibold break-words">{user?.name}</div>
            </div>

            <div className="space-y-1">
              <div className="text-sm uppercase tracking-wide text-white/60">
                Email
              </div>
              <div className="text-xl font-semibold break-words">{user?.email}</div>
            </div>
          </div>

          <footer className="flex flex-wrap gap-3 justify-center pt-2">
            <Link
              href="/"
              className="
                px-5 py-3 rounded-lg
                bg-white/10 hover:bg-white/15 active:bg-white/20
                transition font-medium
              "
            >
              ← Back to Home
            </Link>

            <Link
              href="/login"
              className="
                px-5 py-3 rounded-lg
                bg-purple-700 hover:bg-purple-800 active:bg-purple-900
                transition font-semibold
              "
            >
              Login
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
              Logout
            </Link>
          </footer>
        </section>
      </main>
    </>
  );
}
