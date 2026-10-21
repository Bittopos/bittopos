import { defineConfig } from "vite";

export default defineConfig({
  root: "web",
  server: {
    host: "127.0.0.1",
    port: 1420,
    strictPort: true,
  },
  build: {
    outDir: "../dist",
    emptyOutDir: true,
  },
  define: {
    "import.meta.env.VITE_API_BASE": JSON.stringify("http://127.0.0.1:3000/api/v1"),
  },
});
