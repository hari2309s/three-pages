import { useQuery } from "@tanstack/react-query";
import { bookService } from "@/services/bookService";
import { QUERY_KEYS } from "@/lib/constants";
import type { BookDetail } from "@/types";

export const useBookDetail = (bookId: string | undefined) => {
  return useQuery<BookDetail, Error>({
    queryKey: [...QUERY_KEYS.BOOK, bookId],
    queryFn: () => bookService.getById(bookId!),
    enabled: !!bookId,
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
};
