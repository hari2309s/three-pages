import {
  Book as BookIcon,
  Globe,
  Library,
  Sparkles,
  FileText,
} from "lucide-react";
import {
  Button,
  CardContent,
  CardFooter,
  CardHeader,
  AnimatedCard,
  AnimatedContainer,
  AnimatedText,
} from "@three-pages/ui";
import { truncateText } from "@/lib/utils";
import type { Book } from "@/types";

interface BookCardProps {
  book: Book;
  onSelect: (book: Book) => void;
}

export const BookCard = ({ book, onSelect }: BookCardProps) => {
  const getSourceIcon = (source: string) => {
    switch (source) {
      case "gutenberg":
        return {
          icon: Library,
          label: "Project Gutenberg",
          color: "text-green-600",
        };
      case "openlibrary":
        return { icon: Globe, label: "Open Library", color: "text-blue-600" };
      default:
        return {
          icon: BookIcon,
          label: "Google Books",
          color: "text-gray-600",
        };
    }
  };

  const sourceInfo = getSourceIcon(book.source);
  const getSummarizationStatus = (source: string) => {
    switch (source) {
      case "gutenberg":
        return {
          available: true,
          label: "Summarizable",
          bgColor: "bg-green-100",
          textColor: "text-green-800",
          icon: Sparkles,
        };
      case "openlibrary":
        return {
          available: true,
          label: "May be summarizable",
          bgColor: "bg-yellow-100",
          textColor: "text-yellow-800",
          icon: FileText,
        };
      default:
        return {
          available: false,
          label: "",
          bgColor: "",
          textColor: "",
          icon: null,
        };
    }
  };

  const summaryStatus = getSummarizationStatus(book.source);

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
            <div className="flex items-start justify-between gap-2 mb-2">
              <div className="flex items-center gap-2">
                <sourceInfo.icon className={`h-4 w-4 ${sourceInfo.color}`} />
                <span className={`text-xs ${sourceInfo.color} font-medium`}>
                  {sourceInfo.label}
                </span>
              </div>
              {summaryStatus.available && (
                <div
                  className={`flex items-center gap-1 ${summaryStatus.bgColor} ${summaryStatus.textColor} px-2 py-1 rounded-full text-xs`}
                >
                  {summaryStatus.icon && (
                    <summaryStatus.icon className="h-3 w-3" />
                  )}
                  <span>{summaryStatus.label}</span>
                </div>
              )}
            </div>
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
