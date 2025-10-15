import { motion } from "framer-motion";

export const Header = () => {
  const handleReload = () => {
    window.location.reload();
  };

  return (
    <header className="border-b bg-header text-header mb-2 border-[1px] rounded-md">
      <div className="container mx-auto px-4 py-4">
        <div className="flex items-center gap-2">
          <motion.img
            src="/books.png"
            alt="Three Pages Logo"
            className="h-11 w-11 cursor-pointer hover:opacity-80 transition-opacity"
            onClick={handleReload}
            onKeyDown={(e) => e.key === "Enter" && handleReload()}
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            role="button"
            tabIndex={0}
            aria-label="Reload Three Pages application"
            title="Click to reload the app"
          />
          <motion.div
            className="cursor-pointer hover:opacity-80 transition-opacity"
            onClick={handleReload}
            onKeyDown={(e) => e.key === "Enter" && handleReload()}
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
            role="button"
            tabIndex={0}
            aria-label="Reload Three Pages application"
            title="Click to reload the app"
          >
            <h1 className="text-2xl font-bold text-header">Three Pages</h1>
            <p className="text-sm text-header/80">AI-powered book summaries</p>
          </motion.div>
        </div>
      </div>
    </header>
  );
};
