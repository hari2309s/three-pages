# Three Pages 📚

> **Any book. Three pages. Perfectly summarized.**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/React-19+-blue.svg)](https://reactjs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5+-blue.svg)](https://www.typescriptlang.org/)
[![Performance](https://img.shields.io/badge/Search%20Speed-1--2s-brightgreen)](#performance)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

**Three Pages** is an AI-powered application that transforms how you discover and consume books. Search across millions of titles from multiple sources, generate intelligent summaries with AI, and listen to them with natural text-to-speech—all optimized for speed, reliability, and user experience.

## ✨ Key Features

### 🔍 **Intelligent Book Discovery**
- **Multi-Source Search**: Aggregates results from Google Books, Open Library, and Project Gutenberg
- **Smart Deduplication**: Advanced algorithms eliminate duplicate results across sources
- **Source Prioritization**: Automatically prioritizes free full-text books (Gutenberg) over commercial sources
- **Lightning Fast**: Concurrent API calls deliver results in 1-2 seconds (3x performance improvement)

### 🤖 **AI-Powered Summaries**
- **Advanced Language Models**: Leverages Hugging Face's state-of-the-art summarization models
- **Multiple Styles**: Choose from concise, detailed, academic, or simple summary formats
- **English Support**: Currently optimized for English-language content
- **Timeout Protection**: Robust error handling prevents hanging requests
- **Smart Content Extraction**: Intelligent fallback strategies for optimal summarization

### 🔊 **Natural Text-to-Speech**
- **High-Quality Audio**: AI-generated speech using Microsoft SpeechT5 and Facebook MMS models
- **Intelligent Fallbacks**: Multiple TTS models with graceful fallback to synthetic audio when services are unavailable
- **Enhanced User Experience**: Realistic duration estimation, progress feedback, and detailed error messages
- **Streaming Playbook**: Optimized audio delivery with base64 encoding and Howler.js integration

### ⚡ **Production-Ready Performance**
- **Advanced Caching**: Non-blocking cache operations with statistics and management
- **Health Monitoring**: Comprehensive system health checks for all services
- **Concurrent Processing**: Optimized async operations throughout the stack
- **Error Recovery**: Graceful degradation when external services fail

### 🦀 **Backend (Rust) - Production Optimized**

**Core Technologies:**
- **Axum 0.7**: High-performance async web framework
- **tokio**: Async runtime with concurrent processing
- **SQLx**: Type-safe PostgreSQL integration
- **Moka**: High-performance in-memory caching

**Recent Performance Optimizations:**
- ✅ **Concurrent API Calls**: 3x faster search (1-2s vs 3-5s)
- ✅ **Smart Deduplication**: Eliminates duplicate results with source prioritization
- ✅ **Timeout Protection**: All external calls protected with reasonable timeouts
- ✅ **Enhanced Error Handling**: Descriptive error messages with proper HTTP status codes
- ✅ **Health Monitoring**: Comprehensive system health checks
- ✅ **Cache Optimization**: Non-blocking operations with statistics

**Key Services:**
- **BookAggregatorService**: Multi-source search with intelligent deduplication
- **SummarizerService**: AI-powered text summarization with fallback strategies
- **TTSService**: Text-to-speech audio generation
- **CacheService**: Advanced caching with statistics and management
- **DatabaseService**: Type-safe database operations with connection pooling

### ⚛️ **Frontend (React) - Modern & Responsive**

**Core Technologies:**
- **React 19**: Latest features with concurrent rendering
- **TypeScript 5**: Full type safety across the application
- **Vite**: Lightning-fast development and optimized builds
- **TailwindCSS**: Utility-first styling with custom design system

**User Experience:**
- **Responsive Design**: Optimized for mobile, tablet, and desktop
- **Smooth Animations**: Framer Motion for delightful interactions
- **Real-time Feedback**: Loading states, error handling, and progress indicators
- **Audio Playbook**: Integrated audio player with Howler.js
- **Component Library**: Radix UI primitives with custom styling

## 🚀 Quick Start

### Prerequisites

- **Node.js** 18+ and **pnpm** 9+
- **Rust** 1.70+ with Cargo
- **PostgreSQL** 14+
- **Hugging Face API Key** (required for AI features)

### 1. Clone and Setup

```bash
git clone https://github.com/yourusername/three-pages.git
cd three-pages

# Install all dependencies
pnpm install
```

### 2. Environment Configuration

#### Backend Configuration (`apps/api/.env`)

```env
# Database
APP_SUPABASE_URL=postgresql://user:password@localhost:5432/three_pages
DATABASE_POOL_SIZE=10

# AI Services (Required)
APP_HUGGINGFACE_API_KEY=your_hugging_face_api_key
APP_HUGGINGFACE_API_BASE_URL=https://api-inference.huggingface.co

# Book APIs
GOOGLE_BOOKS_API_KEY=your_google_books_api_key  # Optional but recommended
GUTENBERG_API_BASE_URL=https://gutendx.com

# Server Configuration
PORT=10000
ENVIRONMENT=development
ALLOWED_ORIGINS=http://localhost:5173,http://localhost:3000

# Performance Optimization
CACHE_TTL_SECONDS=3600
CACHE_MAX_CAPACITY=10000
```

#### Frontend Configuration (`apps/web/.env`)

```env
VITE_API_BASE_URL=http://localhost:10000/api
```

### 3. Database Setup

```bash
# Create database
createdb three_pages

# Navigate to API directory and run migrations
cd apps/api
sqlx migrate run
```

### 4. Development

```bash
# Start everything (recommended)
pnpm dev

# Or run services individually
pnpm dev:api    # Rust backend only
pnpm dev:web    # React frontend only
```

**Access Points:**
- 🌐 **Frontend**: http://localhost:5173
- 🔗 **API**: http://localhost:10000/api
- 📊 **Health Check**: http://localhost:10000/api/health/detailed

## 🎯 Usage Examples

### Basic Book Search

```bash
# Search with intelligent deduplication
curl -X POST "http://localhost:10000/api/search" \
  -H "Content-Type: application/json" \
  -d '{"query": "pride and prejudice", "limit": 10}'

# Response includes deduplicated results with source prioritization
{
  "total_results": 8,
  "results": [
    {
      "id": "gutenberg:1342",
      "title": "Pride and Prejudice",
      "authors": ["Jane Austen"],
      "source": "Gutenberg",
      "has_full_text": true
    }
  ],
  "query_understood": {
    "original_query": "pride and prejudice",
    "search_query": "pride and prejudice jane austen"
  }
}
```

### Generate AI Summary

```bash
# Generate summary with timeout protection
curl -X POST "http://localhost:10000/api/books/gutenberg:1342/summary" \
  -H "Content-Type: application/json" \
  -d '{
    "language": "en",
    "style": "concise",
    "max_pages": 3
  }'

# Optimized response with caching
{
  "id": "uuid",
  "summary_text": "Pride and Prejudice follows Elizabeth Bennet...",
  "language": "en",
  "word_count": 245,
  "book_info": {
    "title": "Pride and Prejudice",
    "author": "Jane Austen"
  },
  "created_at": "2024-01-15T10:30:00Z"
}
```

### Generate Audio

```bash
# Generate text-to-speech audio (with enhanced error handling)
curl "http://localhost:10000/api/summary/{summary_id}/audio?language=en&voice_type=default"

# Returns base64-encoded audio data with metadata
{
  "id": "uuid",
  "summary_id": "summary_uuid",
  "language": "en",
  "voice_type": "default",
  "audio_url": "data:audio/wav;base64,UklGRnoGAABXQVZFZm10IBAAAA...",
  "duration_ms": 45000,
  "file_size_kb": 720,
  "created_at": "2024-01-15T10:30:00Z"
}
```

**Audio Generation Features:**
- **Smart Fallbacks**: If HuggingFace TTS fails, generates synthetic audio with realistic duration
- **Progress Feedback**: Frontend shows generation progress (30-60 seconds typical)
- **Error Recovery**: Detailed error messages help users understand and resolve issues
- **Audio Validation**: Comprehensive checks ensure audio quality before delivery

### System Health Check

```bash
# Comprehensive health monitoring
curl "http://localhost:10000/api/health/detailed"

{
  "status": "healthy",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "services": {
    "database": {
      "status": "healthy",
      "response_time_ms": 15
    },
    "cache": {
      "status": "healthy",
      "entry_count": 1250,
      "hit_rate": 0.0
    },
    "external_apis": {
      "google_books": {"status": "healthy"},
      "hugging_face": {"status": "healthy"}
    }
  }
}
```

## 🛠️ Development

### Available Scripts

```bash
# Development
pnpm dev              # Start all services
pnpm dev:web          # Frontend only
pnpm dev:api          # Backend only

# Building
pnpm build            # Build all
pnpm build:web        # Build frontend
pnpm build:api        # Build backend (release mode)

## 🚀 Deployment

### Frontend (Vercel)

The web application is configured for deployment on Vercel:

```bash
# Quick deployment via Vercel CLI
vercel --prod

# Or use the build script
pnpm run build        # Build web app
pnpm run vercel-build # Vercel-specific build
```

**Configuration:**
- **Framework:** Vite
- **Build Command:** `cd apps/web && pnpm install && pnpm run build`
- **Output Directory:** `apps/web/dist`
- **Environment Variables:**
  - `VITE_API_URL`: Your API backend URL
  - `VITE_API_TIMEOUT`: API timeout (120000ms)
  - `VITE_MAX_SUMMARY_LENGTH`: Max summary length (1000)

**Files:**
- `vercel.json` - Project-level Vercel configuration
- `apps/web/vercel.json` - App-specific configuration
- `.vercelignore` - Excludes API code and dev files
- `VERCEL_DEPLOYMENT.md` - Detailed deployment guide

### Backend (Render)

The API is deployed on Render using the `render.yaml` configuration:

```bash
# API runs on: https://book-summarizer-api.onrender.com
# Health check: https://book-summarizer-api.onrender.com/api/health
```

**Environment Variables Required:**
- `HF_TOKEN` - Hugging Face API token
- `GOOGLE_BOOKS_API_KEY` - Google Books API key  
- `DATABASE_URL` - PostgreSQL connection string
- `ALLOWED_ORIGINS` - CORS allowed origins

## 🧪 Testing & Quality
pnpm test:api         # Run Rust tests
pnpm lint:web         # Lint frontend code
pnpm format           # Format all code
./performance_test.sh # Run performance tests

## 🔧 Utilities
pnpm clean            # Clean build artifacts
pnpm install:web      # Install frontend deps only
```

## 📄 License

**MIT License** - see [LICENSE](LICENSE) file for details.

---

**Built with ❤️ by developers who love both books and great code.**
