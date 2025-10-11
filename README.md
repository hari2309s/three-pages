# Three Pages

> Any book. Three pages. Any language.

**Three Pages** is an AI-powered application that generates concise, multilingual book summaries with text-to-speech capabilities. Search for any book, get a comprehensive summary in your preferred language, and listen to it with AI-generated audio.

![Three Pages Demo](https://via.placeholder.com/800x400/270e07/facda3?text=Three+Pages+Demo)

## ✨ Features

- 🔍 **Universal Book Search** - Search across multiple sources (Google Books, Open Library, Project Gutenberg)
- 🤖 **AI-Powered Summaries** - Generate intelligent summaries using state-of-the-art language models
- 🌍 **Multilingual Support** - Get summaries in multiple languages (English, Spanish, French, German, etc.)
- 🔊 **Text-to-Speech** - Convert summaries to natural-sounding audio
- 📚 **Full Text Access** - Read complete books from Project Gutenberg
- 🎨 **Modern UI** - Beautiful, responsive design with smooth animations
- ⚡ **Fast Performance** - Optimized caching and efficient data fetching

## 🏗️ Architecture

Three Pages is built as a modern monorepo with a Rust backend and React frontend:

```
three-pages/
├── apps/
│   ├── api/          # Rust backend (Axum + PostgreSQL)
│   ├── web/          # React frontend (Vite + TypeScript)
│   └── docs/         # Documentation site (Next.js)
├── packages/
│   ├── ui/           # Shared UI components
│   ├── eslint-config/    # Shared ESLint configuration
│   └── typescript-config/  # Shared TypeScript configuration
└── README.md
```

### Backend (Rust)

- **Framework**: Axum 0.7 with tokio async runtime
- **Database**: PostgreSQL with SQLx for type-safe queries
- **Caching**: Moka for in-memory caching
- **AI Integration**: Hugging Face Inference API for summarization and TTS
- **Book Sources**: Google Books API, Open Library API, Project Gutenberg (Gutendex)
- **Deployment**: Render with automatic deployments

### Frontend (React)

- **Framework**: React 18 with TypeScript
- **Build Tool**: Vite for fast development and optimized builds
- **Styling**: TailwindCSS with custom design system
- **State Management**: TanStack Query (React Query) for server state
- **Animations**: Framer Motion for smooth interactions
- **Audio**: Howler.js for audio playback
- **Deployment**: Vercel with edge functions

## 🚀 Quick Start

### Prerequisites

- **Node.js** 18+ and **pnpm** 9+
- **Rust** 1.70+ with Cargo
- **PostgreSQL** 14+
- **API Keys**: Hugging Face API key (required)

### 1. Clone and Install

```bash
git clone https://github.com/yourusername/three-pages.git
cd three-pages
pnpm install
```

### 2. Environment Setup

#### Backend (apps/api/.env)

```env
# Database
DATABASE_URL=postgresql://username:password@localhost/three_pages
DATABASE_POOL_SIZE=5

# AI Services
HF_TOKEN=your_huggingface_api_key
HF_API_BASE_URL=https://api-inference.huggingface.co

# Book APIs
GOOGLE_BOOKS_API_KEY=your_google_books_api_key  # Optional
GUTENBERG_API_BASE=https://gutendex.com

# Server Configuration
PORT=10000
ENVIRONMENT=development
ALLOWED_ORIGINS=http://localhost:5173,http://localhost:3000

# Caching
CACHE_TTL_SECONDS=3600
CACHE_MAX_CAPACITY=1000
```

#### Frontend (apps/web/.env)

```env
VITE_API_BASE_URL=http://localhost:10000/api
```

### 3. Database Setup

```bash
# Create database
createdb three_pages

# Run migrations (from apps/api directory)
cd apps/api
sqlx migrate run
```

### 4. Development

Run both frontend and backend in development mode:

```bash
# Start everything
pnpm dev

# Or run individually
pnpm dev:api    # Backend only
pnpm dev:web    # Frontend only
```

Visit:

- **Frontend**: http://localhost:5173
- **Backend API**: http://localhost:10000/api
- **API Docs**: http://localhost:10000/api/docs

## 📖 Usage

### Search for Books

```
Search examples:
- "lord of the rings tolkien"
- "science fiction about AI"
- "books by jane austen"
- "mystery thriller detective"
```

### Generate Summaries

1. Search for a book
2. Click "View Details"
3. Select language and style preferences
4. Click "Generate Summary"
5. Optionally generate audio with "Generate Audio"

### API Examples

```bash
# Search books
curl "http://localhost:10000/api/search?q=tolkien&limit=10"

# Get book details
curl "http://localhost:10000/api/books/gutenberg:1234"

# Generate summary
curl -X POST "http://localhost:10000/api/summaries" \
  -H "Content-Type: application/json" \
  -d '{"book_id":"gutenberg:1234","language":"en","style":"concise"}'
```

## 🛠️ Development

### Project Structure

```
apps/api/src/
├── api/handlers/     # HTTP request handlers
├── models/          # Data models and types
├── services/        # Business logic
│   ├── books/       # Book search and aggregation
│   └── huggingface/ # AI service integration
├── utils/           # Utilities and helpers
└── main.rs          # Application entry point

apps/web/src/
├── components/      # React components
│   ├── book/        # Book-related components
│   ├── search/      # Search functionality
│   ├── summary/     # Summary features
│   └── layout/      # Layout components
├── hooks/           # Custom React hooks
├── services/        # API client code
├── types/           # TypeScript types
└── App.tsx          # Main application
```

### Available Scripts

```bash
# Development
pnpm dev              # Start all services
pnpm dev:web          # Frontend only
pnpm dev:api          # Backend only

# Building
pnpm build:web        # Build frontend
pnpm build:api        # Build backend
pnpm build            # Build all

# Testing
pnpm test:api         # Run Rust tests
pnpm lint:web         # Lint frontend
pnpm check-types      # TypeScript checks

# Utilities
pnpm clean            # Clean build artifacts
pnpm format           # Format code
```

### Adding New Features

1. **Backend**: Add handlers in `apps/api/src/api/handlers/`
2. **Frontend**: Add components in `apps/web/src/components/`
3. **Shared UI**: Add to `packages/ui/src/`
4. **Types**: Update both Rust models and TypeScript types

## 🚀 Deployment

### Backend (Render)

The API deploys automatically to Render via `render.yaml`:

```yaml
services:
  - type: web
    name: three-pages-api
    env: rust
    buildCommand: cargo build --release
    startCommand: ./target/release/three-pages-api
```

Required environment variables:

- `HF_TOKEN` - Hugging Face API key
- `DATABASE_URL` - PostgreSQL connection string
- `ALLOWED_ORIGINS` - Frontend URL for CORS

### Frontend (Vercel)

Deploy via Vercel CLI or GitHub integration:

```bash
cd apps/web
vercel --prod
```

Set environment variables:

- `VITE_API_BASE_URL` - Backend API URL

## 🤝 Contributing

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Commit** changes: `git commit -m 'Add amazing feature'`
4. **Push** to branch: `git push origin feature/amazing-feature`
5. **Open** a Pull Request

### Development Guidelines

- Follow conventional commit messages
- Add tests for new features
- Update documentation
- Ensure CI passes
- Use `pnpm format` before committing

## 📚 API Documentation

### Book Search

- `GET /api/search?q={query}&limit={limit}` - Search books
- `GET /api/books/{id}` - Get book details

### Summaries

- `POST /api/summaries` - Generate summary
- `GET /api/summaries/{id}` - Get summary

### Audio

- `POST /api/audio` - Generate audio from summary
- `GET /api/audio/{id}/stream` - Stream audio file

Full API documentation available at `/api/docs` when running locally.

## 🔧 Configuration

### Supported Languages

- English (en)
- Spanish (es)
- French (fr)
- German (de)
- Italian (it)
- Portuguese (pt)
- Chinese (zh)
- Japanese (ja)

### Summary Styles

- **Concise** - Brief, key points only
- **Detailed** - Comprehensive analysis
- **Academic** - Scholarly format
- **Creative** - Engaging narrative style

## 🐛 Troubleshooting

### Common Issues

**"Gutenberg books not appearing in search"**

- **Root Causes Fixed**:
  1. **Aggregator Issue**: Gutenberg was not included in the BookAggregatorService search method
  2. **HTTP Redirect Issue**: Gutenberg API returns 301 redirects, but HTTP client wasn't following them
- **Fixes Applied**:
  1. Updated `apps/api/src/services/books/aggregator.rs` to include Gutenberg in search
  2. Configured HTTP client in `apps/api/src/services/books/gutenberg.rs` to follow redirects:
     ```rust
     let client = Client::builder()
         .redirect(reqwest::redirect::Policy::limited(10))
         .timeout(std::time::Duration::from_secs(30))
         .build()
         .unwrap_or_else(|_| Client::new());
     ```
- **Verification Steps**:
  1. Check API connectivity: `curl "https://gutendex.com/books?search=test"`
  2. Ensure `GUTENBERG_API_BASE=https://gutendex.com` is set in environment
  3. Verify the aggregator searches all three sources in search method:
     ```rust
     let gutenberg_result = self.gutenberg.search(query, per_source).await;
     let results = vec![google_result, openlibrary_result, gutenberg_result];
     ```
  4. Check that `per_source` is calculated as `(limit / 3).max(5)` not `(limit / 2).max(5)`
- **Test**: Search for "love", "family", or "shakespeare" - should return Gutenberg results with IDs like `gutenberg:1234`

**"AI services failing"**

- Verify `HF_TOKEN` is valid
- Check Hugging Face API status
- Try reducing concurrent requests

**"Database connection errors"**

- Verify PostgreSQL is running
- Check `DATABASE_URL` format
- Run migrations: `sqlx migrate run`

**"HTTP client timeout errors"**

- Gutenberg service now uses redirect-following client
- Default timeout is 30 seconds
- Check network connectivity to `https://gutendx.com`

### Debug Mode

Enable detailed logging:

```bash
# Backend
RUST_LOG=debug pnpm dev:api

# Frontend
DEBUG=* pnpm dev:web
```

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- **Hugging Face** for AI model inference
- **Project Gutenberg** for free book access
- **Google Books** and **Open Library** for book metadata
- **Rust** and **React** communities for excellent tooling

---

**Built with ❤️ using Rust and React**

For questions or support, please [open an issue](https://github.com/yourusername/three-pages/issues).
