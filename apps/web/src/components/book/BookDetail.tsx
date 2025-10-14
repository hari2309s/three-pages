import { Book as BookIcon, ExternalLink } from "lucide-react";
import {
  Button,
  CardContent,
  CardHeader,
  CardTitle,
  AnimatedCard,
  AnimatedContainer,
  AnimatedText,
} from "@three-pages/ui";
import type { BookDetail as BookDetailType } from "@/types";

interface BookDetailProps {
  book: BookDetailType;
}

export const BookDetail = ({ book }: BookDetailProps) => {
  return (
    <AnimatedCard hover={false} tap={false}>
      <CardHeader>
        <AnimatedText as="h2">
          <CardTitle>Book Details</CardTitle>
        </AnimatedText>
      </CardHeader>
      <CardContent>
        <AnimatedContainer
          variant="container"
          staggerChildren={true}
          className="flex flex-col gap-6 md:flex-row"
        >
          <AnimatedContainer variant="scale" className="flex-shrink-0">
            {book.cover_url ? (
              <img
                src={book.cover_url}
                alt={book.title}
                className="h-64 w-48 rounded-lg object-cover shadow-md"
              />
            ) : (
              <div className="flex h-64 w-48 items-center justify-center rounded-lg bg-muted">
                <BookIcon className="h-24 w-24 text-muted-foreground" />
              </div>
            )}
          </AnimatedContainer>

          <AnimatedContainer
            variant="container"
            staggerChildren={true}
            className="flex-1 space-y-4"
          >
            <AnimatedContainer variant="fade">
              <AnimatedText as="h2" className="text-2xl font-bold">
                {book.title}
              </AnimatedText>
              <AnimatedText
                as="p"
                delay={0.1}
                className="mt-1 text-lg text-muted-foreground"
              >
                {book.authors.join(", ") || "Unknown Author"}
              </AnimatedText>
            </AnimatedContainer>

            <AnimatedContainer
              variant="container"
              staggerChildren={true}
              className="grid grid-cols-2 gap-4 text-sm"
            >
              {book.publisher && (
                <AnimatedContainer variant="fade">
                  <AnimatedText as="p" className="font-medium">
                    Publisher
                  </AnimatedText>
                  <AnimatedText
                    as="p"
                    delay={0.05}
                    className="text-muted-foreground"
                  >
                    {book.publisher}
                  </AnimatedText>
                </AnimatedContainer>
              )}
              {book.published_date && (
                <AnimatedContainer variant="fade">
                  <AnimatedText as="p" className="font-medium">
                    Published
                  </AnimatedText>
                  <AnimatedText
                    as="p"
                    delay={0.05}
                    className="text-muted-foreground"
                  >
                    {book.published_date}
                  </AnimatedText>
                </AnimatedContainer>
              )}
              {book.page_count && (
                <AnimatedContainer variant="fade">
                  <AnimatedText as="p" className="font-medium">
                    Pages
                  </AnimatedText>
                  <AnimatedText
                    as="p"
                    delay={0.05}
                    className="text-muted-foreground"
                  >
                    {book.page_count}
                  </AnimatedText>
                </AnimatedContainer>
              )}
              {book.isbn && (
                <AnimatedContainer variant="fade">
                  <AnimatedText as="p" className="font-medium">
                    ISBN
                  </AnimatedText>
                  <AnimatedText
                    as="p"
                    delay={0.05}
                    className="text-muted-foreground"
                  >
                    {book.isbn}
                  </AnimatedText>
                </AnimatedContainer>
              )}
              {book.language && (
                <AnimatedContainer variant="fade">
                  <AnimatedText as="p" className="font-medium">
                    Language
                  </AnimatedText>
                  <AnimatedText
                    as="p"
                    delay={0.05}
                    className="text-muted-foreground uppercase"
                  >
                    {book.language}
                  </AnimatedText>
                </AnimatedContainer>
              )}
            </AnimatedContainer>

            {book.description && (
              <AnimatedContainer variant="fade">
                <AnimatedText as="p" className="font-medium">
                  Description
                </AnimatedText>
                <AnimatedText
                  as="p"
                  delay={0.1}
                  className="mt-2 text-sm text-muted-foreground"
                >
                  {book.description}
                </AnimatedText>
              </AnimatedContainer>
            )}

            {book.preview_link && (
              <AnimatedContainer variant="scale">
                <a
                  href={book.preview_link}
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  <Button
                    variant="outline"
                    className="inline-flex items-center gap-2"
                  >
                    Preview
                    <ExternalLink className="h-4 w-4" />
                  </Button>
                </a>
              </AnimatedContainer>
            )}

            {!book.content_url && (
              <AnimatedContainer variant="fade">
                <AnimatedText as="p" className="text-sm text-muted-foreground">
                  Summary generation is only available for books from Project
                  Gutenberg
                </AnimatedText>
              </AnimatedContainer>
            )}
          </AnimatedContainer>
        </AnimatedContainer>
      </CardContent>
    </AnimatedCard>
  );
};
