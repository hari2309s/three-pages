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
            className="h-11 w-11 cursor-pointer hover:opacity-90 transition-all duration-200 rounded-lg hover:shadow-lg hover:bg-white/10 p-1"
            onClick={handleReload}
            onKeyDown={(e) => e.key === "Enter" && handleReload()}
            whileHover={{ scale: 1.08, rotate: 2 }}
            whileTap={{ scale: 0.92, rotate: -2 }}
            role="button"
            tabIndex={0}
            aria-label="Reload Three Pages application"
            title="Click to reload the app"
          />
          <motion.div
            className="cursor-pointer hover:opacity-95 transition-all duration-200 rounded-lg hover:shadow-md hover:bg-white/5 p-2 -m-2"
            onClick={handleReload}
            onKeyDown={(e) => e.key === "Enter" && handleReload()}
            whileHover={{ scale: 1.03, x: 2 }}
            whileTap={{ scale: 0.97, x: -1 }}
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
