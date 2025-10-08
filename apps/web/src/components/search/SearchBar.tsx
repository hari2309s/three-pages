import { useState, FormEvent } from "react";
import { Search } from "lucide-react";
import { motion } from "framer-motion";
import { Button, AnimatedContainer } from "@three-pages/ui";

interface SearchBarProps {
  onSearch: (query: string) => void;
  isLoading?: boolean;
}

export const SearchBar = ({ onSearch, isLoading }: SearchBarProps) => {
  const [query, setQuery] = useState("");

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    if (query.trim()) {
      onSearch(query.trim());
    }
  };

  return (
    <AnimatedContainer variant="scale" className="w-full max-w-3xl mx-auto">
      <form onSubmit={handleSubmit}>
        <AnimatedContainer
          variant="container"
          staggerChildren={true}
          className="flex gap-2"
        >
          <AnimatedContainer variant="fade" className="relative flex-1">
            <Search className="absolute left-3 top-1/2 h-5 w-5 -translate-y-1/2 text-muted-foreground" />
            <motion.input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search for books... (e.g., 'thriller about AI' or 'books by Stephen King')"
              className="w-full rounded-lg border border-input bg-background px-10 py-3 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
              disabled={isLoading}
              whileFocus={{
                scale: 1.02,
                transition: { duration: 0.2 },
              }}
              whileTap={{
                scale: 0.98,
                transition: { duration: 0.1 },
              }}
            />
          </AnimatedContainer>
          <Button type="submit" disabled={!query.trim() || isLoading} size="lg">
            Search
          </Button>
        </AnimatedContainer>
      </form>
    </AnimatedContainer>
  );
};
