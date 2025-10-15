import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      "@/components": path.resolve(__dirname, "./src/components"),
      "@/hooks": path.resolve(__dirname, "./src/hooks"),
      "@/services": path.resolve(__dirname, "./src/services"),
      "@/types": path.resolve(__dirname, "./src/types"),
      "@/lib": path.resolve(__dirname, "./src/lib"),
    },
  },
  build: {
    outDir: "dist",
    chunkSizeWarningLimit: 1000,
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ["react", "react-dom"],
          ui: [
            "@radix-ui/react-dialog",
            "@radix-ui/react-progress",
            "@radix-ui/react-select",
            "@radix-ui/react-slot",
            "@radix-ui/react-tabs",
          ],
          motion: ["framer-motion"],
          query: ["@tanstack/react-query"],
          markdown: ["react-markdown"],
          audio: ["howler"],
          utils: [
            "axios",
            "clsx",
            "class-variance-authority",
            "tailwind-merge",
            "zustand",
          ],
        },
      },
    },
    target: "es2015",
    minify: "terser",
    sourcemap: false,
  },
  server: {
    port: 5173,
    proxy: {
      "/api": {
        target: process.env.VITE_API_URL || "http://localhost:10000",
        changeOrigin: true,
      },
    },
  },
  preview: {
    port: 4173,
  },
});
