import { useState } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AnimatePresence } from "framer-motion";
import { Layout } from "@/components/layout";
import { SearchBar, SearchResults } from "@/components/search";
import { BookDetail } from "@/components/book";
import { StyleSelector, SummaryCard } from "@/components/summary";
import { AudioPlayer } from "@/components/audio";
import { LoadingSpinner, ErrorMessage, Button } from "@three-pages/ui";
import { useBookSearch, useAudio, useBookDetail, useSummary } from "@/hooks";
import {
  AnimatedContainer,
  AnimatedText,
  PageTransition,
} from "@three-pages/ui";
import type { Book } from "@/types";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

function AppContent() {
  const [selectedBookId, setSelectedBookId] = useState<string>();
  const [summaryStyle, setSummaryStyle] = useState("concise");

  const searchMutation = useBookSearch();
  const { data: bookDetail, isLoading: isLoadingBook } =
    useBookDetail(selectedBookId);
  const summaryMutation = useSummary();
  const audio = useAudio();

  const handleSearch = (query: string) => {
    searchMutation.mutate({ query, limit: 12 });
    setSelectedBookId(undefined);
    summaryMutation.reset();
    audio.reset();
  };

  const handleSelectBook = (book: Book) => {
    setSelectedBookId(book.id);
    summaryMutation.reset();
    audio.reset();
  };

  const handleGenerateSummary = () => {
    if (!selectedBookId) return;

    summaryMutation.mutate({
      bookId: selectedBookId,
      request: {
        language: "en",
        style: summaryStyle,
      },
    });
  };

  const handleGenerateAudio = () => {
    if (!summaryMutation.data?.id) return;

    audio.mutate({
      summaryId: summaryMutation.data.id,
      language: "en",
    });
  };

  const handleBackToSearch = () => {
    setSelectedBookId(undefined);
    summaryMutation.reset();
    audio.reset();
  };

  return (
    <Layout>
      <PageTransition>
        <div className="space-y-8">
          <AnimatedContainer variant="fade" className="text-center space-y-4">
            <AnimatedText as="div">
              <SearchBar
                onSearch={handleSearch}
                isLoading={searchMutation.isPending}
              />
            </AnimatedText>
            {selectedBookId && (
              <AnimatedContainer variant="fade" delay={0.2}>
                <Button variant="outline" onClick={handleBackToSearch}>
                  ← Back to Search Results
                </Button>
              </AnimatedContainer>
            )}
          </AnimatedContainer>

          <AnimatePresence mode="wait">
            {!selectedBookId ? (
              <AnimatedContainer
                key="search-results"
                variant="container"
                staggerChildren={true}
              >
                <SearchResults
                  results={searchMutation.data}
                  isLoading={searchMutation.isPending}
                  error={searchMutation.error}
                  onSelectBook={handleSelectBook}
                />
              </AnimatedContainer>
            ) : (
              <AnimatedContainer
                key="book-detail"
                variant="page"
                className="space-y-6"
              >
                {isLoadingBook ? (
                  <AnimatedContainer
                    variant="fade"
                    className="flex justify-center py-12"
                  >
                    <LoadingSpinner size="lg" />
                  </AnimatedContainer>
                ) : bookDetail ? (
                  <>
                    <AnimatedContainer variant="scale">
                      <BookDetail book={bookDetail} />
                    </AnimatedContainer>

                    {bookDetail.content_url && (
                      <AnimatedContainer
                        variant="container"
                        staggerChildren={true}
                        className="space-y-4"
                      >
                        <AnimatedContainer variant="fade">
                          <div className="flex flex-col gap-4 sm:flex-row sm:items-end">
                            <StyleSelector
                              selectedStyle={summaryStyle}
                              onStyleChange={setSummaryStyle}
                            />
                            <div className="flex-1">
                              <Button
                                onClick={handleGenerateSummary}
                                disabled={summaryMutation.isPending}
                                className="w-full"
                              >
                                {summaryMutation.isPending ? (
                                  <div className="flex items-center justify-center gap-2">
                                    <LoadingSpinner size="sm" />
                                    <span>Generating Summary...</span>
                                  </div>
                                ) : (
                                  "Generate Summary"
                                )}
                              </Button>
                            </div>
                          </div>
                        </AnimatedContainer>

                        {summaryMutation.isError && (
                          <AnimatedContainer variant="fade">
                            <ErrorMessage
                              message={summaryMutation.error.message}
                            />
                          </AnimatedContainer>
                        )}

                        <AnimatePresence>
                          {summaryMutation.data && (
                            <AnimatedContainer
                              key="summary-section"
                              variant="container"
                              staggerChildren={true}
                            >
                              <AnimatedContainer variant="scale">
                                <SummaryCard summary={summaryMutation.data} />
                              </AnimatedContainer>

                              <AnimatedContainer
                                variant="fade"
                                className="space-y-4"
                              >
                                <Button
                                  onClick={handleGenerateAudio}
                                  disabled={audio.isPending}
                                  variant="outline"
                                  className="w-full"
                                >
                                  {audio.isPending
                                    ? "Generating Audio..."
                                    : "Generate Audio"}
                                </Button>

                                {audio.isError && (
                                  <AnimatedContainer variant="fade">
                                    <ErrorMessage
                                      message={audio.error.message}
                                    />
                                  </AnimatedContainer>
                                )}

                                <AnimatePresence>
                                  {audio.data && (
                                    <AnimatedContainer
                                      key="audio-player"
                                      variant="scale"
                                    >
                                      <AudioPlayer
                                        isPlaying={audio.isPlaying}
                                        currentTime={audio.currentTime}
                                        duration={audio.duration}
                                        onPlay={audio.play}
                                        onPause={audio.pause}
                                        onStop={audio.stop}
                                        onSeek={audio.seek}
                                      />
                                    </AnimatedContainer>
                                  )}
                                </AnimatePresence>
                              </AnimatedContainer>
                            </AnimatedContainer>
                          )}
                        </AnimatePresence>
                      </AnimatedContainer>
                    )}
                  </>
                ) : null}
              </AnimatedContainer>
            )}
          </AnimatePresence>
        </div>
      </PageTransition>
    </Layout>
  );
}

const App = () => {
  return (
    <QueryClientProvider client={queryClient}>
      <AppContent />
    </QueryClientProvider>
  );
};

export default App;
