import api from "@/services/api";
import type { SummaryRequest, SummaryResponse } from "@/types";

const MAX_RETRIES = 2;
const RETRY_DELAY = 1000;

const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export const summaryService = {
  generate: async (
    bookId: string,
    request: SummaryRequest,
  ): Promise<SummaryResponse> => {
    console.log(
      `Starting summary generation for book ${bookId} in ${request.language}`,
    );

    let lastError: any;

    for (let attempt = 1; attempt <= MAX_RETRIES; attempt++) {
      try {
        console.log(`Summary attempt ${attempt}/${MAX_RETRIES}`);

        const { data } = await api.post<SummaryResponse>(
          `/api/books/${bookId}/summary`,
          request,
          {
            timeout: 180000, // 3 minutes for individual request
          },
        );

        console.log(`Summary generated successfully on attempt ${attempt}`);
        return data;
      } catch (error: any) {
        console.warn(`Summary attempt ${attempt} failed:`, error.message);
        lastError = error;

        // Don't retry on client errors (4xx) except timeout
        if (
          error.response?.status >= 400 &&
          error.response?.status < 500 &&
          !error.code?.includes("timeout")
        ) {
          console.log("Client error detected, not retrying");
          break;
        }

        // If this isn't the last attempt, wait before retrying
        if (attempt < MAX_RETRIES) {
          const delayTime = RETRY_DELAY * attempt;
          console.log(`Waiting ${delayTime}ms before retry...`);
          await delay(delayTime);
        }
      }
    }

    // If we get here, all attempts failed
    const errorMessage =
      lastError?.response?.data?.message ||
      lastError?.message ||
      "Summary generation failed after multiple attempts";

    console.error("All summary attempts failed:", errorMessage);
    throw new Error(errorMessage);
  },
};
