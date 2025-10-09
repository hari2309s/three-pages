import { motion } from "framer-motion";

export const Header = () => {
  return (
    <header className="border-b bg-header text-header">
      <div className="container mx-auto px-4 py-4">
        <div className="flex items-center gap-2">
          <motion.img
            src="/assets/books.png"
            alt="Three Pages Logo"
            className="h-11 w-11"
          />
          <div>
            <h1 className="text-2xl font-bold text-header">Three Pages</h1>
            <p className="text-sm text-header/80">
              AI-powered book summaries in multiple languages
            </p>
          </div>
        </div>
      </div>
    </header>
  );
};
