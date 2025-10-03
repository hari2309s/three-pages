import { useMutation } from "@tanstack/react-query";
import { bookService } from "@/services/bookService";
import type { SearchRequest, SearchResponse } from "@/types";

export const useBookSearch = () => {
  return useMutation<SearchResponse, Error, SearchRequest>({
    mutationFn: (request) => bookService.search(request),
  });
};
