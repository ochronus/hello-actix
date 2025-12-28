# syntax=docker/dockerfile:1

# 1) Rust dependency planner (for caching)
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 2) Rust builder (uses cached deps from planner)
FROM chef AS rust-builder
WORKDIR /app
COPY --from=planner /app/recipe.json /app/recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build the application binary
COPY . .
RUN cargo build --release --bin hello-actix

# 3) Frontend builder (builds Vite client assets + SSR bundle)
FROM node:24-trixie-slim AS frontend-builder
WORKDIR /app

# Install dependencies first (cache)
COPY package.json package-lock.json ./
RUN npm ci

# Copy only what's needed for build
COPY vite.config.js tsconfig.json ./
COPY www ./www
# If you have static files under public/, copy them before build so vite can include them if needed
COPY public ./public

# Build client bundle into public/bundle and SSR entry into dist/ssr/ssr.js
RUN npm run build

# 4) Runtime image - includes Node (for SSR server) + Rust binary
FROM node:24-trixie-slim AS runtime
WORKDIR /app

# Install runtime dependencies: OpenSSL 3 (libssl3) and CA certificates
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy the compiled Rust binary
COPY --from=rust-builder /app/target/release/hello-actix /usr/local/bin/hello-actix

# Copy built frontend assets (client + SSR)
COPY --from=frontend-builder /app/public /app/public
COPY --from=frontend-builder /app/dist/ssr /app/dist/ssr

# Copy the Handlebars root template used by ViteHBSTemplateResolver
# (Resolver expects it at "www/root.hbs")
COPY www/root.hbs /app/www/root.hbs

# Environment (Fly will provide PORT; we keep APP_MODE explicitly prod)
ENV APP_MODE=prod
ENV NODE_ENV=production

# Expose Fly's default port; app will read PORT env to bind
EXPOSE 8080

# Run the server (it will spawn the SSR Node server and serve assets from /app/public)
ENTRYPOINT ["/usr/local/bin/hello-actix"]
