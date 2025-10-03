export interface AudioRequest {
  language: string;
  voice_type?: string;
}

export interface AudioResponse {
  id: string;
  summary_id: string;
  language: string;
  voice_type: string;
  duration_ms?: number;
  file_size_kb?: number;
  audio_url: string;
  created_at: string;
}
