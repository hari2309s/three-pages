import api from "@/services/api";
import type { SearchRequest, SearchResponse, BookDetail } from "@/types";

export const bookService = {
  search: async (request: SearchRequest): Promise<SearchResponse> => {
    const { data } = await api.post<SearchResponse>("/api/search", request);
    return data;
  },

  getById: async (id: string): Promise<BookDetail> => {
    // Encode the ID to handle special characters like colons
    const encodedId = encodeURIComponent(id);
    const { data } = await api.get<BookDetail>(`/api/books/${encodedId}`);
    return data;
  },

  checkHealth: async (): Promise<{ status: string }> => {
    const { data } = await api.get("/api/health");
    return data;
  },
};
