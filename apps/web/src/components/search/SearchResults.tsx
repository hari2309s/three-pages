import { BookCard } from "./BookCard";
import { LoadingSpinner } from "../shared/LoadingSpinner";
import { ErrorMessage } from "../shared/ErrorMessage";
import {
  AnimatedContainer,
  AnimatedList,
  AnimatedListItem,
  AnimatedText,
} from "../animated";
import type { Book, SearchResponse } from "@/types";

interface SearchResultsProps {
  results?: SearchResponse;
  isLoading: boolean;
  error: Error | null;
  onSelectBook: (book: Book) => void;
}

export const SearchResults = ({
  results,
  isLoading,
  error,
  onSelectBook,
}: SearchResultsProps) => {
  if (isLoading) {
    return (
      <AnimatedContainer
        variant="fade"
        className="flex flex-col items-center justify-center py-12"
      >
        <LoadingSpinner size="lg" />
        <AnimatedText as="p" delay={0.2} className="mt-4 text-muted-foreground">
          Searching for books...
        </AnimatedText>
      </AnimatedContainer>
    );
  }

  if (error) {
    return (
      <AnimatedContainer variant="scale">
        <ErrorMessage message={error.message} className="mx-auto max-w-2xl" />
      </AnimatedContainer>
    );
  }

  if (!results) {
    return (
      <AnimatedContainer
        variant="container"
        staggerChildren={true}
        className="flex flex-col items-center justify-center py-12 text-center"
      >
        <AnimatedText as="p" className="text-lg text-muted-foreground">
          Start by searching for a book above
        </AnimatedText>
        <AnimatedText
          as="p"
          delay={0.1}
          className="mt-2 text-sm text-muted-foreground"
        >
          Try: "science fiction about AI" or "books by J.K. Rowling"
        </AnimatedText>
      </AnimatedContainer>
    );
  }

  if (results.results.length === 0) {
    return (
      <AnimatedContainer
        variant="container"
        staggerChildren={true}
        className="flex flex-col items-center justify-center py-12 text-center"
      >
        <AnimatedText as="p" className="text-lg text-muted-foreground">
          No books found
        </AnimatedText>
        <AnimatedText
          as="p"
          delay={0.1}
          className="mt-2 text-sm text-muted-foreground"
        >
          Try a different search query
        </AnimatedText>
      </AnimatedContainer>
    );
  }

  return (
    <AnimatedContainer
      variant="container"
      staggerChildren={true}
      className="space-y-6"
    >
      {results.query_understood && (
        <AnimatedContainer variant="fade" className="rounded-lg bg-muted p-4">
          <AnimatedText as="p" className="text-sm">
            <span className="font-medium">Search interpreted as:</span>{" "}
            {results.query_understood.search_query}
          </AnimatedText>
          {results.query_understood.extracted_terms.genre && (
            <AnimatedText
              as="p"
              delay={0.1}
              className="mt-1 text-sm text-muted-foreground"
            >
              Genre: {results.query_understood.extracted_terms.genre}
            </AnimatedText>
          )}
        </AnimatedContainer>
      )}

      <AnimatedList
        staggerDelay={0.08}
        className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3"
      >
        {results.results.map((book, index) => (
          <AnimatedListItem key={book.id} index={index}>
            <BookCard book={book} onSelect={onSelectBook} />
          </AnimatedListItem>
        ))}
      </AnimatedList>

      <AnimatedText
        as="p"
        className="text-center text-sm text-muted-foreground"
      >
        Found {results.total_results} books
      </AnimatedText>
    </AnimatedContainer>
  );
};
