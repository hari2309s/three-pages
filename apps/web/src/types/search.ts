import { Book } from "./book";

export interface SearchRequest {
  query: string;
  limit?: number;
}

export interface ExtractedTerms {
  genre?: string;
  theme?: string;
  keywords: string[];
  author?: string;
  title?: string;
}

export interface QueryIntent {
  original_query: string;
  extracted_terms: ExtractedTerms;
  search_query: string;
}

export interface SearchResponse {
  results: Book[];
  total_results: number;
  query_understood: QueryIntent;
}
