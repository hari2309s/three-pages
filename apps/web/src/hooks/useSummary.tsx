import { useMutation } from "@tanstack/react-query";
import { summaryService } from "@/services/summaryService";
import type { SummaryRequest, SummaryResponse } from "@/types";

interface SummaryMutationArgs {
  bookId: string;
  request: SummaryRequest;
}

export const useSummary = () => {
  return useMutation<SummaryResponse, Error, SummaryMutationArgs>({
    mutationFn: ({ bookId, request }) =>
      summaryService.generate(bookId, request),
  });
};
