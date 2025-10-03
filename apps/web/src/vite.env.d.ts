/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_API_URL: string;
  readonly VITE_API_TIMEOUT: string;
  readonly VITE_SUPPORTED_LANGUAGES: string;
  readonly VITE_MAX_SUMMARY_LENGTH: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
