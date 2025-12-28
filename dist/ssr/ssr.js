import { jsxs, Fragment, jsx } from "react/jsx-runtime";
import { Head, Link, usePage, router, createInertiaApp } from "@inertiajs/react";
import { useState, useEffect } from "react";
import ReactDOMServer from "react-dom/server";
import createServer from "@inertiajs/react/server";
function Contact({ user }) {
  return /* @__PURE__ */ jsxs(Fragment, { children: [
    /* @__PURE__ */ jsxs(Head, { children: [
      /* @__PURE__ */ jsx("title", { children: "Contact" }),
      /* @__PURE__ */ jsx(
        "meta",
        {
          name: "description",
          content: "A simple Contact page mirroring the actix_ssr example."
        }
      )
    ] }),
    /* @__PURE__ */ jsx("main", { className: "w-full h-full flex flex-col justify-center items-center px-6", children: /* @__PURE__ */ jsxs("section", { className: "w-full max-w-2xl rounded-2xl bg-white/10 p-8 shadow-lg space-y-6", children: [
      /* @__PURE__ */ jsxs("header", { className: "text-center", children: [
        /* @__PURE__ */ jsx("h1", { className: "text-5xl font-black tracking-tight mb-3", children: "Contact" }),
        /* @__PURE__ */ jsx("p", { className: "text-white/80", children: "This page mirrors the actix_ssr example and shows server-provided user details passed as Inertia props." })
      ] }),
      /* @__PURE__ */ jsxs(
        "div",
        {
          className: "\n              grid gap-4 sm:grid-cols-2 bg-black/20 rounded-xl p-6\n              border border-white/10\n            ",
          children: [
            /* @__PURE__ */ jsxs("div", { className: "space-y-1", children: [
              /* @__PURE__ */ jsx("div", { className: "text-sm uppercase tracking-wide text-white/60", children: "Name" }),
              /* @__PURE__ */ jsx("div", { className: "text-xl font-semibold break-words", children: user?.name })
            ] }),
            /* @__PURE__ */ jsxs("div", { className: "space-y-1", children: [
              /* @__PURE__ */ jsx("div", { className: "text-sm uppercase tracking-wide text-white/60", children: "Email" }),
              /* @__PURE__ */ jsx("div", { className: "text-xl font-semibold break-words", children: user?.email })
            ] })
          ]
        }
      ),
      /* @__PURE__ */ jsxs("footer", { className: "flex flex-wrap gap-3 justify-center pt-2", children: [
        /* @__PURE__ */ jsx(
          Link,
          {
            href: "/",
            className: "\n                px-5 py-3 rounded-lg\n                bg-white/10 hover:bg-white/15 active:bg-white/20\n                transition font-medium\n              ",
            children: "← Back to Home"
          }
        ),
        /* @__PURE__ */ jsx(
          Link,
          {
            href: "/login",
            className: "\n                px-5 py-3 rounded-lg\n                bg-purple-700 hover:bg-purple-800 active:bg-purple-900\n                transition font-semibold\n              ",
            children: "Login"
          }
        ),
        /* @__PURE__ */ jsx(
          Link,
          {
            href: "/logout",
            method: "post",
            as: "button",
            className: "\n                px-5 py-3 rounded-lg\n                bg-red-600 hover:bg-red-700 active:bg-red-800\n                transition font-semibold\n              ",
            children: "Logout"
          }
        )
      ] })
    ] }) })
  ] });
}
const __vite_glob_0_0 = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  default: Contact
}, Symbol.toStringTag, { value: "Module" }));
function Index({ message, version, auth }) {
  const user = auth?.user ?? null;
  const [count, setCount] = useState(0);
  const increment = () => setCount((prev) => ++prev);
  return /* @__PURE__ */ jsxs(Fragment, { children: [
    /* @__PURE__ */ jsxs(Head, { children: [
      /* @__PURE__ */ jsx("title", { children: "Hello, from inertia-rust!" }),
      /* @__PURE__ */ jsx("meta", { name: "description", content: "Just a mocked head... Ha!" })
    ] }),
    /* @__PURE__ */ jsxs("main", { className: "w-full h-full flex flex-col items-center justify-center gap-8 px-6", children: [
      /* @__PURE__ */ jsx("section", { className: "w-full max-w-3xl rounded-2xl bg-white/10 p-6", children: /* @__PURE__ */ jsxs("div", { className: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4", children: [
        /* @__PURE__ */ jsxs("div", { children: [
          /* @__PURE__ */ jsx("h2", { className: "text-2xl font-black", children: "Auth state" }),
          user ? /* @__PURE__ */ jsxs("p", { className: "text-white/80 mt-1", children: [
            "Logged in as ",
            /* @__PURE__ */ jsx("span", { className: "font-semibold", children: user.id })
          ] }) : /* @__PURE__ */ jsx("p", { className: "text-white/80 mt-1", children: "You are not logged in." })
        ] }),
        /* @__PURE__ */ jsxs("div", { className: "flex flex-wrap gap-3", children: [
          !user ? /* @__PURE__ */ jsx(Fragment, { children: /* @__PURE__ */ jsx(
            Link,
            {
              href: "/login",
              method: "post",
              as: "button",
              className: "\n                      px-5 py-2 rounded-lg\n                      bg-purple-700 hover:bg-purple-800 active:bg-purple-900\n                      transition font-semibold\n                    ",
              children: "Log in"
            }
          ) }) : /* @__PURE__ */ jsx(Fragment, { children: /* @__PURE__ */ jsx(
            Link,
            {
              href: "/logout",
              method: "post",
              as: "button",
              className: "\n                      px-5 py-2 rounded-lg\n                      bg-red-600 hover:bg-red-700 active:bg-red-800\n                      transition font-semibold\n                    ",
              children: "Log out"
            }
          ) }),
          /* @__PURE__ */ jsx(
            Link,
            {
              href: "/contact",
              className: "px-5 py-2 rounded-lg bg-white/10 hover:bg-white/15 transition font-medium",
              children: "Contact"
            }
          )
        ] })
      ] }) }),
      /* @__PURE__ */ jsxs("section", { className: "w-full max-w-3xl text-center", children: [
        /* @__PURE__ */ jsxs("h1", { className: "text-6xl font-black mb-3", children: [
          "Yeah!",
          /* @__PURE__ */ jsx("br", {}),
          "Inertia-Rust v",
          version
        ] }),
        /* @__PURE__ */ jsx("p", { className: "text-xl font-medium mb-8", children: message }),
        /* @__PURE__ */ jsxs("div", { className: "flex flex-col items-center gap-4 w-fit mx-auto", children: [
          /* @__PURE__ */ jsx("div", { className: "rounded-2xl bg-white/10 h-20 w-full grid place-items-center px-10", children: /* @__PURE__ */ jsx("span", { className: "font-black text-4xl", children: count }) }),
          /* @__PURE__ */ jsx(
            "button",
            {
              className: "\n                p-7 py-4 rounded-xl bg-purple-700 hover:bg-purple-800 active:bg-purple-900\n                transition-all duration-100 ring-0 ring-purple-600/25 focus:ring-8 outline-none\n                select-none font-medium text-xl cursor-default\n              ",
              onClick: increment,
              children: "Taste this state!"
            }
          )
        ] })
      ] })
    ] })
  ] });
}
const __vite_glob_0_1 = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  default: Index
}, Symbol.toStringTag, { value: "Module" }));
function Login() {
  const { auth } = usePage().props;
  const user = auth?.user ?? null;
  return /* @__PURE__ */ jsxs(Fragment, { children: [
    /* @__PURE__ */ jsxs(Head, { children: [
      /* @__PURE__ */ jsx("title", { children: "Login" }),
      /* @__PURE__ */ jsx("meta", { name: "description", content: "Demo login page using Inertia + Actix." })
    ] }),
    /* @__PURE__ */ jsx("main", { className: "w-full h-full flex flex-col items-center justify-center px-6", children: /* @__PURE__ */ jsxs("div", { className: "w-full max-w-lg rounded-2xl bg-white/10 p-8 flex flex-col gap-6", children: [
      /* @__PURE__ */ jsxs("header", { className: "text-center", children: [
        /* @__PURE__ */ jsx("h1", { className: "text-4xl font-black mb-2", children: "Authentication" }),
        /* @__PURE__ */ jsx("p", { className: "text-white/80", children: "This page demonstrates logging in and out using actix-identity with Inertia." })
      ] }),
      user ? /* @__PURE__ */ jsxs("section", { className: "space-y-4", children: [
        /* @__PURE__ */ jsx("div", { className: "rounded-xl bg-emerald-600/20 border border-emerald-400/30 p-4", children: /* @__PURE__ */ jsxs("p", { className: "text-lg", children: [
          "You are logged in as",
          /* @__PURE__ */ jsxs("span", { className: "font-semibold", children: [
            " ",
            user.id
          ] }),
          "."
        ] }) }),
        /* @__PURE__ */ jsxs("div", { className: "flex gap-3 flex-wrap", children: [
          /* @__PURE__ */ jsx(
            Link,
            {
              href: "/",
              className: "px-5 py-3 rounded-lg bg-white/10 hover:bg-white/15 transition font-medium",
              children: "Go to Home"
            }
          ),
          /* @__PURE__ */ jsx(
            Link,
            {
              href: "/logout",
              method: "post",
              as: "button",
              className: "\n                    px-5 py-3 rounded-lg\n                    bg-red-600 hover:bg-red-700 active:bg-red-800\n                    transition font-semibold\n                  ",
              children: "Log out"
            }
          )
        ] })
      ] }) : /* @__PURE__ */ jsxs("section", { className: "space-y-4", children: [
        /* @__PURE__ */ jsx("div", { className: "rounded-xl bg-purple-600/20 border border-purple-400/30 p-4", children: /* @__PURE__ */ jsx("p", { className: "text-lg", children: "You are currently not logged in." }) }),
        /* @__PURE__ */ jsxs("div", { className: "flex gap-3 flex-wrap", children: [
          /* @__PURE__ */ jsx(
            Link,
            {
              href: "/login",
              method: "post",
              as: "button",
              className: "\n                    px-5 py-3 rounded-lg\n                    bg-purple-700 hover:bg-purple-800 active:bg-purple-900\n                    transition font-semibold\n                  ",
              children: "Log in"
            }
          ),
          /* @__PURE__ */ jsx(
            Link,
            {
              href: "/",
              className: "px-5 py-3 rounded-lg bg-white/10 hover:bg-white/15 transition font-medium",
              children: "Back to Home"
            }
          )
        ] })
      ] }),
      /* @__PURE__ */ jsx("footer", { className: "pt-2 text-center", children: /* @__PURE__ */ jsx(
        Link,
        {
          href: "/contact",
          className: "text-purple-200 underline hover:text-purple-100 transition",
          children: "Contact"
        }
      ) })
    ] }) })
  ] });
}
const __vite_glob_0_2 = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  default: Login
}, Symbol.toStringTag, { value: "Module" }));
function Logout() {
  useEffect(() => {
    router.post("/logout", {}, { preserveScroll: true });
  }, []);
  return /* @__PURE__ */ jsxs(Fragment, { children: [
    /* @__PURE__ */ jsxs(Head, { children: [
      /* @__PURE__ */ jsx("title", { children: "Logging out..." }),
      /* @__PURE__ */ jsx("meta", { name: "robots", content: "noindex" })
    ] }),
    /* @__PURE__ */ jsx("main", { className: "w-full h-full flex items-center justify-center px-6", children: /* @__PURE__ */ jsxs("form", { method: "post", action: "/logout", className: "w-full max-w-md text-center", children: [
      /* @__PURE__ */ jsx("p", { className: "text-xl font-medium mb-4", children: "Logging you out..." }),
      /* @__PURE__ */ jsxs("p", { className: "text-white/70", children: [
        "If you are not redirected automatically,",
        /* @__PURE__ */ jsx(
          "button",
          {
            type: "submit",
            className: "underline text-purple-200 hover:text-purple-100 ml-1",
            children: "click here"
          }
        ),
        "."
      ] }),
      /* @__PURE__ */ jsx("noscript", { children: /* @__PURE__ */ jsx("div", { className: "mt-6", children: /* @__PURE__ */ jsx(
        "button",
        {
          type: "submit",
          className: "px-5 py-3 rounded-lg bg-red-600 hover:bg-red-700 active:bg-red-800 font-semibold",
          children: "Log out"
        }
      ) }) })
    ] }) })
  ] });
}
const __vite_glob_0_3 = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  default: Logout
}, Symbol.toStringTag, { value: "Module" }));
const portArgIdx = process.argv.indexOf("--port");
const port = portArgIdx >= 0 ? Number(process.argv[portArgIdx + 1]) : 5174;
const appName = "Inertia Test";
const titleResolver = (title) => title ? `${appName} - ${title}` : title;
createServer((page) => {
  return createInertiaApp({
    page,
    title: titleResolver,
    render: ReactDOMServer.renderToString,
    resolve: async (component) => {
      const pages = /* @__PURE__ */ Object.assign({ "./pages/Contact.tsx": __vite_glob_0_0, "./pages/Index.tsx": __vite_glob_0_1, "./pages/Login.tsx": __vite_glob_0_2, "./pages/Logout.tsx": __vite_glob_0_3 });
      return pages[`./pages/${component}.tsx`];
    },
    setup: ({ App, props }) => {
      return /* @__PURE__ */ jsx(App, { ...props });
    }
  });
}, port);
export {
  appName,
  titleResolver
};
