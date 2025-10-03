import { Book as BookIcon } from "lucide-react";
import { CardContent, CardFooter, CardHeader } from "../shared/Card";
import { Button } from "../shared/Button";
import { truncateText } from "@/lib/utils";
import { AnimatedCard, AnimatedContainer, AnimatedText } from "../animated";
import type { Book } from "@/types";

interface BookCardProps {
  book: Book;
  onSelect: (book: Book) => void;
}

export const BookCard = ({ book, onSelect }: BookCardProps) => {
  return (
    <AnimatedCard
      hover={true}
      tap={true}
      className="flex flex-col overflow-hidden"
    >
      <CardHeader className="pb-4">
        <AnimatedContainer
          variant="container"
          staggerChildren={true}
          className="flex gap-4"
        >
          <AnimatedContainer variant="scale">
            {book.cover_url ? (
              <img
                src={book.cover_url}
                alt={book.title}
                className="h-32 w-24 rounded object-cover"
                onError={(e) => {
                  e.currentTarget.style.display = "none";
                }}
              />
            ) : (
              <div className="flex h-32 w-24 items-center justify-center rounded bg-muted">
                <BookIcon className="h-12 w-12 text-muted-foreground" />
              </div>
            )}
          </AnimatedContainer>
          <AnimatedContainer variant="fade" className="flex-1">
            <AnimatedText
              as="h3"
              className="text-lg line-clamp-2 font-semibold"
            >
              {book.title}
            </AnimatedText>
            <AnimatedText
              as="p"
              delay={0.1}
              className="mt-1 text-sm text-muted-foreground"
            >
              {book.authors.join(", ") || "Unknown Author"}
            </AnimatedText>
            {book.published_date && (
              <AnimatedText
                as="p"
                delay={0.2}
                className="mt-1 text-xs text-muted-foreground"
              >
                Published: {book.published_date}
              </AnimatedText>
            )}
          </AnimatedContainer>
        </AnimatedContainer>
      </CardHeader>
      <CardContent className="flex-1">
        {book.description && (
          <AnimatedText
            as="p"
            className="text-sm text-muted-foreground line-clamp-3"
          >
            {truncateText(book.description, 150)}
          </AnimatedText>
        )}
      </CardContent>
      <CardFooter>
        <AnimatedContainer variant="scale">
          <Button onClick={() => onSelect(book)} className="w-full">
            View Details
          </Button>
        </AnimatedContainer>
      </CardFooter>
    </AnimatedCard>
  );
};
