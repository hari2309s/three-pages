# Deployment Guide - API Optimizations

This guide covers deploying the optimized Three Pages API with all the performance improvements and new features.

## Prerequisites

- Rust 1.70+ with Cargo
- PostgreSQL 14+
- Environment variables configured
- Redis/Cache service (optional but recommended for production)

## Environment Variables

Ensure all required environment variables are set:

```bash
# Required
APP_SUPABASE_URL=postgresql://user:password@host:port/database
APP_HUGGINGFACE_API_KEY=your_hugging_face_token

# Optional but recommended
GOOGLE_BOOKS_API_KEY=your_google_books_key
PORT=10000
ENVIRONMENT=production

# Cache Configuration
CACHE_TTL_SECONDS=3600
CACHE_MAX_CAPACITY=10000

# Database Pool
DATABASE_POOL_SIZE=10

# API Base URLs
APP_HUGGINGFACE_API_BASE_URL=https://api-inference.huggingface.co
GUTENBERG_API_BASE_URL=https://gutendex.com

# CORS
ALLOWED_ORIGINS=https://yourapp.com,https://www.yourapp.com
```

## Deployment Steps

### 1. Pre-Deployment Checks

```bash
# Navigate to API directory
cd apps/api

# Run tests
cargo test

# Check for compilation errors
cargo check --release

# Verify all dependencies
cargo audit
```

### 2. Database Setup

The optimized API doesn't require new migrations, but verify your existing schema:

```bash
# Check current migrations
sqlx migrate info --database-url $APP_SUPABASE_URL

# Run any pending migrations
sqlx migrate run --database-url $APP_SUPABASE_URL
```

### 3. Build for Production

```bash
# Build optimized binary
cargo build --release

# The binary will be at: target/release/three-pages-api
```

### 4. Deploy the Service

#### Option A: Direct Deployment

```bash
# Copy binary to production server
scp target/release/three-pages-api user@server:/usr/local/bin/

# Start the service
./three-pages-api
```

#### Option B: Docker Deployment

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/three-pages-api /usr/local/bin/
EXPOSE 10000
CMD ["three-pages-api"]
```

```bash
# Build and run
docker build -t three-pages-api .
docker run -p 10000:10000 --env-file .env three-pages-api
```

## Testing the Deployment

### 1. Basic Health Checks

```bash
# Simple health check (for load balancers)
curl http://localhost:10000/api/health

# Expected response:
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime_seconds": 123
}

# Detailed health check (for monitoring)
curl http://localhost:10000/api/health/detailed

# Expected response includes:
{
  "status": "healthy",
  "services": {
    "database": { "status": "healthy", "response_time_ms": 15 },
    "cache": { "status": "healthy", "entry_count": 42 },
    "external_apis": {
      "google_books": { "status": "healthy" },
      "hugging_face": { "status": "healthy" }
    }
  }
}
```

### 2. Search Functionality

```bash
# Test book search with deduplication
curl -X POST http://localhost:10000/api/search \
  -H "Content-Type: application/json" \
  -d '{"query": "pride and prejudice", "limit": 10}'

# Verify:
# - Results are deduplicated
# - Gutenberg sources appear first
# - Response time under 2 seconds
```

### 3. Summary Generation

```bash
# Test summary generation with timeout protection
curl -X POST http://localhost:10000/api/books/gutenberg:1342/summary \
  -H "Content-Type: application/json" \
  -d '{
    "language": "en",
    "style": "concise",
    "max_pages": 3
  }'

# Verify:
# - Request completes within timeout
# - Error messages are descriptive
# - Cache is populated
```

### 4. Cache Operations

```bash
# Clear cache and verify response
curl -X DELETE http://localhost:10000/api/cache/clear

# Expected response:
{
  "status": "success",
  "message": "Cache cleared successfully. X entries were invalidated.",
  "entries_cleared": X,
  "cache_stats": {...}
}
```

## Performance Testing

### 1. Load Testing Search Endpoints

```bash
# Install hey (HTTP load tester)
go install github.com/rakyll/hey@latest

# Test concurrent searches
hey -n 100 -c 10 -m POST -H "Content-Type: application/json" \
  -d '{"query":"shakespeare","limit":5}' \
  http://localhost:10000/api/search

# Expected:
# - All requests complete successfully
# - Average response time < 2s
# - No timeout errors
```

### 2. Summary Generation Load Test

```bash
# Test concurrent summary requests
for i in {1..5}; do
  curl -X POST http://localhost:10000/api/books/gutenberg:$((1340+i))/summary \
    -H "Content-Type: application/json" \
    -d '{"language":"en","style":"concise"}' &
done
wait

# Verify all requests complete within timeout
```

### 3. Cache Performance

```bash
# First request (cache miss)
time curl -X POST http://localhost:10000/api/search \
  -H "Content-Type: application/json" \
  -d '{"query":"test","limit":5}'

# Second request (cache hit) - should be much faster
time curl -X POST http://localhost:10000/api/search \
  -H "Content-Type: application/json" \
  -d '{"query":"test","limit":5}'
```

## Monitoring Setup

### 1. Health Check Monitoring

Set up monitoring to regularly check the detailed health endpoint:

```bash
#!/bin/bash
# health_monitor.sh
HEALTH_ENDPOINT="http://localhost:10000/api/health/detailed"

while true; do
  RESPONSE=$(curl -s $HEALTH_ENDPOINT)
  STATUS=$(echo $RESPONSE | jq -r '.status')
  
  if [ "$STATUS" != "healthy" ]; then
    echo "ALERT: API health check failed - Status: $STATUS"
    # Send alert to monitoring system
  fi
  
  sleep 60
done
```

### 2. Log Monitoring

The API now provides structured logging. Set up log aggregation:

```bash
# Example log patterns to monitor:
# - "Cache clear requested" (manual intervention)
# - "search failed" (search errors)
# - "Summary generation failed" (AI service issues)
# - "Database connection" (DB issues)
```

### 3. Metrics Collection

Key metrics to track:

- Request response times
- Cache hit/miss ratios
- External API response times
- Database connection pool usage
- Error rates by endpoint

## Rollback Procedure

If issues arise, follow this rollback procedure:

### 1. Immediate Rollback

```bash
# Stop new service
sudo systemctl stop three-pages-api

# Start previous version
sudo systemctl start three-pages-api-backup

# Verify rollback
curl http://localhost:10000/api/health
```

### 2. Database Rollback

The optimizations don't change the database schema, so no database rollback is needed.

### 3. Cache Rollback

```bash
# Clear cache to prevent issues with new data structures
curl -X DELETE http://localhost:10000/api/cache/clear
```

## Common Issues and Solutions

### 1. Timeout Errors

**Symptoms**: Requests timing out, especially summary generation

**Solutions**:
- Check external API connectivity (HuggingFace, Google Books)
- Verify database connection pool settings
- Monitor resource usage (CPU, memory)

### 2. Cache Issues

**Symptoms**: High memory usage, slow responses

**Solutions**:
```bash
# Clear cache
curl -X DELETE http://localhost:10000/api/cache/clear

# Check cache stats
curl http://localhost:10000/api/health/detailed | jq '.services.cache'

# Adjust cache settings
export CACHE_MAX_CAPACITY=5000
export CACHE_TTL_SECONDS=1800
```

### 3. Database Connection Issues

**Symptoms**: "Database connection timeout" in health checks

**Solutions**:
```bash
# Check database connectivity
psql $APP_SUPABASE_URL -c "SELECT 1;"

# Adjust pool size
export DATABASE_POOL_SIZE=5
```

### 4. External API Rate Limits

**Symptoms**: "External service unavailable" errors

**Solutions**:
- Verify API keys are valid
- Check rate limits on external services
- Monitor API quotas

## Production Optimization

### 1. Resource Allocation

Recommended minimum resources:
- **CPU**: 2 cores
- **Memory**: 2GB RAM
- **Storage**: 20GB (for logs and temporary files)
- **Network**: Stable internet for external APIs

### 2. Environment Tuning

```bash
# Production environment variables
export ENVIRONMENT=production
export RUST_LOG=info
export CACHE_MAX_CAPACITY=50000
export DATABASE_POOL_SIZE=20
```

### 3. Monitoring Alerts

Set up alerts for:
- Health check failures
- Response time > 5 seconds
- Error rate > 5%
- Cache hit rate < 50%
- Database connection pool exhaustion

## Security Considerations

1. **API Keys**: Ensure all API keys are properly secured
2. **Database**: Use connection pooling with proper limits
3. **Rate Limiting**: Consider adding API rate limiting
4. **CORS**: Configure appropriate allowed origins
5. **Logging**: Don't log sensitive data (API keys, user data)

## Verification Checklist

Before declaring deployment successful:

- [ ] All health checks passing
- [ ] Search returns deduplicated results
- [ ] Summary generation completes within timeout
- [ ] Cache operations work correctly
- [ ] Error messages are descriptive
- [ ] Performance meets expectations
- [ ] Monitoring is active
- [ ] Logs are being collected
- [ ] Rollback procedure tested

## Support

For issues with the optimized API:

1. Check the detailed health endpoint first
2. Review application logs for specific error messages
3. Verify all environment variables are correctly set
4. Test individual components (database, external APIs)
5. Check resource usage and scaling requirements

The optimizations are designed to be backward compatible, so existing clients should continue to work without changes.