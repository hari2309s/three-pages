import axios, { AxiosError } from "axios";
import { API_CONFIG } from "@/lib/constants";
import type { ApiError } from "@/types";

console.log("API Configuration:", {
  baseURL: API_CONFIG.BASE_URL,
  timeout: API_CONFIG.TIMEOUT,
  env_timeout: import.meta.env.VITE_API_TIMEOUT,
});

const api = axios.create({
  baseURL: API_CONFIG.BASE_URL,
  timeout: 120000, // 2 minutes explicit timeout for summarization
  headers: {
    "Content-Type": "application/json",
  },
});

api.interceptors.response.use(
  (response) => response,
  (error: AxiosError<{ error?: string }>) => {
    const apiError: ApiError = {
      message:
        error.response?.data?.error || error.message || "An error occurred",
      status: error.response?.status,
    };
    return Promise.reject(apiError);
  },
);

export default api;
