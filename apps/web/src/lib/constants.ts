export const SUPPORTED_LANGUAGES = [
  { code: "en", name: "English", flag: "🇬🇧" },
  { code: "es", name: "Spanish", flag: "🇪🇸" },
  { code: "fr", name: "French", flag: "🇫🇷" },
  { code: "de", name: "German", flag: "🇩🇪" },
  { code: "it", name: "Italian", flag: "🇮🇹" },
];

export const SUMMARY_STYLES = [
  { value: "concise", label: "Concise", description: "Brief and to the point" },
  {
    value: "detailed",
    label: "Detailed",
    description: "Comprehensive coverage",
  },
  { value: "academic", label: "Academic", description: "Formal and scholarly" },
  { value: "simple", label: "Simple", description: "Easy to understand" },
];

export const API_CONFIG = {
  BASE_URL: import.meta.env.VITE_API_URL || "http://localhost:10000",
  TIMEOUT: parseInt(import.meta.env.VITE_API_TIMEOUT || "30000"),
};

export const QUERY_KEYS = {
  HEALTH: ["health"],
  SEARCH: ["search"],
  BOOK: ["book"],
  SUMMARY: ["summary"],
  AUDIO: ["audio"],
} as const;
