import axios, { AxiosError } from "axios";
import { API_CONFIG } from "@/lib/constants";
import type { ApiError } from "@/types";

const api = axios.create({
  baseURL: API_CONFIG.BASE_URL,
  timeout: API_CONFIG.TIMEOUT,
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
