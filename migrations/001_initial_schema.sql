-- Create summaries table
CREATE TABLE IF NOT EXISTS summaries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    book_id VARCHAR(100) NOT NULL,
    book_title VARCHAR(500) NOT NULL,
    book_author VARCHAR(200) NOT NULL,
    isbn VARCHAR(20),
    language VARCHAR(10) NOT NULL,
    summary_text TEXT NOT NULL,
    word_count INTEGER NOT NULL,
    style VARCHAR(20) NOT NULL DEFAULT 'concise',
    source_hash VARCHAR(64) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create index on book_id, language, and style for faster lookups
CREATE INDEX IF NOT EXISTS idx_summaries_book_lang_style
    ON summaries(book_id, language, style);

-- Create index on source_hash for deduplication
CREATE INDEX IF NOT EXISTS idx_summaries_source_hash
    ON summaries(source_hash);

-- Create audio_files table
CREATE TABLE IF NOT EXISTS audio_files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    summary_id UUID NOT NULL REFERENCES summaries(id) ON DELETE CASCADE,
    language VARCHAR(10) NOT NULL,
    voice_type VARCHAR(50) NOT NULL DEFAULT 'default',
    file_url TEXT NOT NULL,
    duration_ms INTEGER,
    file_size_kb INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create index on summary_id and language for faster lookups
CREATE INDEX IF NOT EXISTS idx_audio_summary_lang
    ON audio_files(summary_id, language);

-- Create book_cache table for caching book metadata
CREATE TABLE IF NOT EXISTS book_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    external_id VARCHAR(100) NOT NULL UNIQUE,
    provider VARCHAR(20) NOT NULL,
    metadata JSONB NOT NULL,
    content_url TEXT,
    cached_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Create index on external_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_book_cache_external_id
    ON book_cache(external_id);

-- Create index on expires_at for cleanup operations
CREATE INDEX IF NOT EXISTS idx_book_cache_expires_at
    ON book_cache(expires_at);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
CREATE TRIGGER update_summaries_updated_at
    BEFORE UPDATE ON summaries
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
