# Build the Rust WebAssembly module
FROM rust:latest AS wasm-builder
WORKDIR /app
COPY pihex pihex
RUN cargo install wasm-pack && \
    cd pihex && wasm-pack build --target web

# Build TailwindCSS
FROM node:18 AS tailwind-builder
WORKDIR /app
COPY package.json package-lock.json ./
COPY index.html .
RUN mkdir styles
COPY ./styles/input.css ./styles
RUN npm install tailwindcss @tailwindcss/cli
RUN npm run build

# Serve with Nginx
FROM nginx:alpine
COPY --from=tailwind-builder /app/styles/output.css /usr/share/nginx/html/styles/output.css
RUN mkdir -p /usr/share/nginx/html/pihex
COPY --from=wasm-builder /app/pihex/pkg /usr/share/nginx/html/pihex/pkg
COPY index.html /usr/share/nginx/html/index.html
COPY images /usr/share/nginx/html/images
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
