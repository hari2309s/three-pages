import api from "@/services/api";
import type { SummaryRequest, SummaryResponse } from "@/types";

export const summaryService = {
  generate: async (
    bookId: string,
    request: SummaryRequest,
  ): Promise<SummaryResponse> => {
    const { data } = await api.post<SummaryResponse>(
      `/api/books/${bookId}/summary`,
      request,
    );
    return data;
  },
};
