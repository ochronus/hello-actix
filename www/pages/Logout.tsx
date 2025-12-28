import { Head, router } from "@inertiajs/react";
import { useEffect } from "react";

export default function Logout() {
  useEffect(() => {
    // Immediately post to /logout on mount; the server responds with a redirect.
    router.post("/logout", {}, { preserveScroll: true });
  }, []);

  return (
    <>
      <Head>
        <title>Logging out...</title>
        <meta name="robots" content="noindex" />
      </Head>

      <main className="w-full h-full flex items-center justify-center px-6">
        <form method="post" action="/logout" className="w-full max-w-md text-center">
          <p className="text-xl font-medium mb-4">Logging you out...</p>
          <p className="text-white/70">
            If you are not redirected automatically,
            <button
              type="submit"
              className="underline text-purple-200 hover:text-purple-100 ml-1"
            >
              click here
            </button>
            .
          </p>

          <noscript>
            <div className="mt-6">
              <button
                type="submit"
                className="px-5 py-3 rounded-lg bg-red-600 hover:bg-red-700 active:bg-red-800 font-semibold"
              >
                Log out
              </button>
            </div>
          </noscript>
        </form>
      </main>
    </>
  );
}
