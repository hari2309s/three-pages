import { BookOpen } from "lucide-react";

export const Header = () => {
  return (
    <header className="border-b">
      <div className="container mx-auto px-4 py-4">
        <div className="flex items-center gap-2">
          <BookOpen className="h-8 w-8 text-primary" />
          <div>
            <h1 className="text-2xl font-bold">Book Summarizer</h1>
            <p className="text-sm text-muted-foreground">
              AI-powered book summaries in multiple languages
            </p>
          </div>
        </div>
      </div>
    </header>
  );
};
