export interface SummaryRequest {
  language: string;
  style?: string;
}

export interface BookInfo {
  title: string;
  author: string;
  isbn?: string;
}

export interface SummaryResponse {
  id: string;
  summary_text: string;
  language: string;
  word_count: number;
  book_info: BookInfo;
  created_at: string;
}
