import api from "./api";
import type { SearchRequest, SearchResponse, BookDetail } from "@/types";

export const bookService = {
  search: async (request: SearchRequest): Promise<SearchResponse> => {
    const { data } = await api.post<SearchResponse>("/api/search", request);
    return data;
  },

  getById: async (id: string): Promise<BookDetail> => {
    const { data } = await api.get<BookDetail>(`/api/books/${id}`);
    return data;
  },

  checkHealth: async (): Promise<{ status: string }> => {
    const { data } = await api.get("/api/health");
    return data;
  },
};
