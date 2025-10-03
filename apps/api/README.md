# Three Pages API

A high-performance Rust backend API for generating multilingual book summaries with text-to-speech capabilities.

## Features

- 🔍 **Natural Language Search** - Search books using conversational queries
- 📚 **Multiple Book Sources** - Integrates Google Books, Open Library, and Project Gutenberg
- 📝 **AI-Powered Summaries** - Generate 3-page summaries using Hugging Face LLMs
- 🌍 **Multilingual Support** - 15+ languages for summaries and audio
- 🔊 **Text-to-Speech** - Convert summaries to audio in multiple languages
- ⚡ **High Performance** - Async Rust with caching and connection pooling
- 💾 **Smart Caching** - In-memory and database caching for optimal performance

## Tech Stack

- **Framework**: Axum 0.7
- **Runtime**: Tokio
- **Database**: PostgreSQL (via SQLx)
- **Cache**: Moka (in-memory)
- **AI**: Hugging Face Inference API
- **Deployment**: Render

## Prerequisites

- Rust 1.70+
- PostgreSQL database (Supabase recommended)
- Hugging Face API token

## Quick Start

### 1. Clone the repository

```bash
git clone <repository-url>
cd backend
```

### 2. Set up environment variables

```bash
cp .env.example .env
# Edit .env with your credentials
```

### 3. Set up the database

```bash
# The migrations will run automatically on startup
# Or run manually with:
sqlx database create
sqlx migrate run
```

### 4. Build and run

```bash
# Development
cargo run

# Production
cargo build --release
./target/release/three-pages-api
```

The API will start on `http://localhost:10000`

## API Endpoints

### Health Check

```
GET /api/health
```

### Search Books

```
POST /api/search
Content-Type: application/json

{
  "query": "thriller about artificial intelligence",
  "limit": 10
}
```

### Get Book Details

```
GET /api/books/:id
```

### Generate Summary

```
POST /api/books/:id/summary
Content-Type: application/json

{
  "language": "en",
  "style": "concise"
}
```

### Get Audio

```
GET /api/summary/:id/audio?language=en&voice_type=default
```

## Environment Variables

| Variable               | Description                            | Required | Default                              |
| ---------------------- | -------------------------------------- | -------- | ------------------------------------ |
| `PORT`                 | Server port                            | No       | 10000                                |
| `ENVIRONMENT`          | Environment (development/production)   | No       | development                          |
| `DATABASE_URL`         | PostgreSQL connection string           | Yes      | -                                    |
| `DATABASE_POOL_SIZE`   | Database connection pool size          | No       | 5                                    |
| `HF_TOKEN`             | Hugging Face API token                 | Yes      | -                                    |
| `HF_API_BASE_URL`      | Hugging Face API base URL              | No       | https://api-inference.huggingface.co |
| `GOOGLE_BOOKS_API_KEY` | Google Books API key (optional)        | No       | -                                    |
| `GUTENBERG_API_BASE`   | Project Gutenberg API base URL         | No       | https://gutendex.com                 |
| `CACHE_TTL_SECONDS`    | Cache TTL in seconds                   | No       | 3600                                 |
| `CACHE_MAX_CAPACITY`   | Maximum cache entries                  | No       | 1000                                 |
| `ALLOWED_ORIGINS`      | CORS allowed origins (comma-separated) | No       | localhost:5173,localhost:3000        |

## Project Structure

```
backend/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── api/                    # API layer
│   │   ├── handlers/          # Request handlers
│   │   └── routes.rs          # Route definitions
│   ├── config/                # Configuration management
│   ├── middleware/            # Custom middleware
│   ├── models/                # Data models
│   ├── services/              # Business logic
│   │   ├── books/            # Book search services
│   │   ├── cache/            # Caching service
│   │   ├── huggingface/      # AI services
│   │   └── storage/          # Database operations
│   └── utils/                 # Utility functions
├── migrations/                # Database migrations
├── Cargo.toml                # Dependencies
└── README.md
```

## Development

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Database Migrations

```bash
# Create a new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## Deployment

### Render

1. Push your code to GitHub
2. Connect your repository to Render
3. Render will use the `render.yaml` configuration
4. Set environment variables in Render dashboard
5. Deploy!

### Manual Deployment

```bash
# Build for production
cargo build --release

# Run the binary
./target/release/book-summarizer-api
```

## Performance Optimization

- **Connection Pooling**: Configured for optimal database performance
- **In-Memory Caching**: Frequently accessed data cached with Moka
- **Async Operations**: Non-blocking I/O throughout the application
- **Request Deduplication**: Identical searches served from cache
- **Compression**: Gzip compression for API responses

## Monitoring

- Health check endpoint: `/api/health`
- Structured logging with tracing
- Request/response logging in development mode

## Troubleshooting

### Database Connection Issues

```bash
# Check connection string format
# postgresql://user:password@host:port/database

# Verify database is accessible
psql $DATABASE_URL
```

### Hugging Face API Issues

- Verify your HF_TOKEN is valid
- Check rate limits on your account
- Some models may have queue times

### Memory Issues

- Adjust CACHE_MAX_CAPACITY if needed
- Monitor cache hit rates
- Consider reducing DATABASE_POOL_SIZE on constrained systems

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

[Your chosen license]

## Support

For issues and questions:

- Open an issue on GitHub
- Check existing documentation
- Review logs for error details
