export type BookSource = "google" | "openlibrary" | "gutenberg";

export interface Book {
  id: string;
  title: string;
  authors: string[];
  description?: string;
  isbn?: string;
  publisher?: string;
  published_date?: string;
  page_count?: number;
  language?: string;
  cover_url?: string;
  preview_link?: string;
  source: BookSource;
}

export interface BookDetail extends Book {
  content_url?: string;
  gutenberg_id?: number;
}
