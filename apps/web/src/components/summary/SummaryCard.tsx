import { useState } from "react";
import { Copy, Download, Check } from "lucide-react";
import ReactMarkdown from "react-markdown";
import { motion, AnimatePresence } from "framer-motion";
import {
  Button,
  CardContent,
  CardHeader,
  CardTitle,
  AnimatedCard,
  AnimatedContainer,
  AnimatedText,
} from "@three-pages/ui";
import { buttonVariants } from "@three-pages/ui/animations";
import { copyToClipboard, downloadText, formatNumber } from "@/lib/utils";

import type { SummaryResponse } from "@/types";

interface SummaryCardProps {
  summary: SummaryResponse;
}

export const SummaryCard = ({ summary }: SummaryCardProps) => {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await copyToClipboard(summary.summary_text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleDownload = () => {
    const filename = `${summary.book_info.title.replace(/[^a-z0-9]/gi, "_")}_summary.txt`;
    downloadText(summary.summary_text, filename);
  };

  return (
    <AnimatedCard hover={false} tap={false}>
      <CardHeader>
        <AnimatedContainer
          variant="container"
          staggerChildren={true}
          className="flex items-center justify-between"
        >
          <AnimatedText as="h3">
            <CardTitle>Summary</CardTitle>
          </AnimatedText>
          <AnimatedContainer
            variant="container"
            staggerChildren={true}
            className="flex gap-2"
          >
            <motion.div
              variants={buttonVariants}
              initial="idle"
              whileHover="hover"
              whileTap="tap"
            >
              <Button
                variant="outline"
                size="icon"
                onClick={handleCopy}
                title="Copy to clipboard"
              >
                <AnimatePresence mode="wait">
                  {copied ? (
                    <motion.div
                      key="check"
                      initial={{ scale: 0, rotate: -90 }}
                      animate={{ scale: 1, rotate: 0 }}
                      exit={{ scale: 0, rotate: 90 }}
                      transition={{ duration: 0.2 }}
                    >
                      <Check className="h-4 w-4" />
                    </motion.div>
                  ) : (
                    <motion.div
                      key="copy"
                      initial={{ scale: 0 }}
                      animate={{ scale: 1 }}
                      exit={{ scale: 0 }}
                      transition={{ duration: 0.2 }}
                    >
                      <Copy className="h-4 w-4" />
                    </motion.div>
                  )}
                </AnimatePresence>
              </Button>
            </motion.div>
            <motion.div
              variants={buttonVariants}
              initial="idle"
              whileHover="hover"
              whileTap="tap"
            >
              <Button
                variant="outline"
                size="icon"
                onClick={handleDownload}
                title="Download summary"
              >
                <Download className="h-4 w-4" />
              </Button>
            </motion.div>
          </AnimatedContainer>
        </AnimatedContainer>
      </CardHeader>
      <CardContent className="space-y-4">
        <AnimatedContainer
          variant="container"
          staggerChildren={true}
          className="space-y-4"
        >
          <AnimatedContainer
            variant="fade"
            className="flex items-center justify-between text-sm text-muted-foreground"
          >
            <AnimatedText as="span">
              {summary.book_info.title} by {summary.book_info.author}
            </AnimatedText>
            <AnimatedText as="span" delay={0.1}>
              {formatNumber(summary.word_count)} words
            </AnimatedText>
          </AnimatedContainer>

          <AnimatedContainer
            variant="fade"
            className="prose prose-sm max-w-none dark:prose-invert"
          >
            <ReactMarkdown>{summary.summary_text}</ReactMarkdown>
          </AnimatedContainer>
        </AnimatedContainer>
      </CardContent>
    </AnimatedCard>
  );
};
