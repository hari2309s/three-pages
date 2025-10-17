# Render Deployment Guide - Three Pages API

This guide helps you deploy the Three Pages Rust API to Render and troubleshoot common deployment issues.

## 🚨 Quick Fix for Database Connection Error

If you're seeing this error:
```
Error: Database error: error communicating with database: Network is unreachable (os error 101)
```

**Root Cause**: Database service not properly configured or environment variables incorrect.

**Solution**: Follow the complete setup below.

## 📋 Prerequisites

- [Render account](https://render.com)
- PostgreSQL database (we'll create this)
- Required API keys:
  - Hugging Face API token
  - Google Books API key (optional)

## 🗄️ Database Setup

### Step 1: Create PostgreSQL Database

1. Go to [Render Dashboard](https://dashboard.render.com)
2. Click "New +" → "PostgreSQL"
3. Configure:
   - **Name**: `three-pages-db`
   - **Database**: `three_pages`
   - **User**: `three_pages_user`
   - **Plan**: Free (or paid as needed)
   - **Region**: Same as your web service
4. Click "Create Database"
5. **Important**: Wait for database to be fully provisioned before proceeding

### Step 2: Get Database Connection String

After creation, go to database dashboard and copy:
- **Internal Database URL** (for Render services)
- **External Database URL** (for local development)

Format: `postgres://username:password@host:port/database`

## 🚀 API Service Setup

### Step 1: Create Web Service

1. Go to [Render Dashboard](https://dashboard.render.com)
2. Click "New +" → "Web Service"
3. Connect your GitHub repository
4. Configure:
   - **Name**: `three-pages-api`
   - **Runtime**: `Rust`
   - **Plan**: Free (or paid as needed)
   - **Region**: Same as your database

### Step 2: Build Configuration

Set these build settings:

| Setting | Value |
|---------|-------|
| **Root Directory** | `apps/api` |
| **Build Command** | `cargo build --release` |
| **Start Command** | `./target/release/three-pages-api` |

### Step 3: Environment Variables

Add these environment variables in the Render dashboard:

| Variable | Value | Notes |
|----------|-------|-------|
| `DATABASE_URL` | [From database dashboard] | Use internal URL |
| `DATABASE_POOL_SIZE` | `5` | Connection pool size |
| `HF_TOKEN` | `[Your HF token]` | Hugging Face API key |
| `GOOGLE_BOOKS_API_KEY` | `[Your API key]` | Optional |
| `PORT` | `10000` | Server port |
| `ENVIRONMENT` | `production` | Environment mode |
| `RUST_LOG` | `info,three_pages_api=debug` | Logging level |
| `HF_API_BASE_URL` | `https://api-inference.huggingface.co` | HF API endpoint |
| `GUTENBERG_API_BASE_URL` | `https://gutendex.com` | Gutenberg API |
| `CACHE_TTL_SECONDS` | `3600` | Cache duration |
| `CACHE_MAX_CAPACITY` | `1000` | Cache size |
| `ALLOWED_ORIGINS` | `https://your-frontend.vercel.app` | CORS origins |

### Step 4: Health Check

Set **Health Check Path** to: `/api/health`

## 📄 Configuration Files

Your `render.yaml` should look like this:

```yaml
services:
  # PostgreSQL Database
  - type: pserv
    name: three-pages-db
    plan: free
    databaseName: three_pages
    user: three_pages_user

  # API Service
  - type: web
    name: three-pages-api
    runtime: rust
    plan: free
    rootDir: apps/api
    buildCommand: cargo build --release
    startCommand: ./target/release/three-pages-api
    healthCheckPath: /api/health
    envVars:
      - key: DATABASE_URL
        fromDatabase:
          name: three-pages-db
          property: connectionString
      - key: HF_TOKEN
        sync: false
      - key: GOOGLE_BOOKS_API_KEY
        sync: false
      - key: ALLOWED_ORIGINS
        sync: false
      - key: PORT
        value: 10000
      - key: ENVIRONMENT
        value: production
      - key: DATABASE_POOL_SIZE
        value: 5
      - key: RUST_LOG
        value: info,three_pages_api=debug
      - key: HF_API_BASE_URL
        value: https://api-inference.huggingface.co
      - key: GUTENBERG_API_BASE_URL
        value: https://gutendex.com
      - key: CACHE_TTL_SECONDS
        value: 3600
      - key: CACHE_MAX_CAPACITY
        value: 1000
```

## 🔧 Troubleshooting

### Database Connection Issues

**Error**: `Network is unreachable (os error 101)`

**Diagnosis**:
```bash
# Run the database health check
./check_db.sh
```

**Solutions**:
1. **Database not ready**: Wait 5-10 minutes after database creation
2. **Wrong connection string**: Verify DATABASE_URL in environment variables
3. **Database service dependency**: Ensure database is created before API service
4. **Region mismatch**: Database and API should be in same region

### Build Failures

**Error**: `cargo build failed`

**Solutions**:
1. **Dependencies**: Check Cargo.toml for correct versions
2. **Root directory**: Ensure `rootDir: apps/api` in render.yaml
3. **Rust version**: Render uses stable Rust, ensure compatibility

### Migration Failures

**Error**: `Migration failed`

**Solutions**:
1. **Migration files**: Ensure `migrations/` folder is accessible
2. **Database permissions**: User must have CREATE TABLE permissions
3. **Schema conflicts**: Drop and recreate database if needed

### Runtime Errors

**Error**: `Required environment variable missing`

**Solutions**:
1. Check all required environment variables are set
2. Use Render dashboard to verify values
3. Check variable names match exactly

### Health Check Failures

**Error**: `Health check failed`

**Solutions**:
1. **Health endpoint**: Verify `/api/health` returns 200
2. **Service startup**: Check logs for startup errors
3. **Port binding**: Ensure app listens on PORT environment variable

## 🔍 Debugging Commands

### Check Database Connection
```bash
# Local testing
export DATABASE_URL="your_database_url_here"
./check_db.sh
```

### View Logs
```bash
# In Render dashboard
# Go to your service → Logs tab
# Look for startup errors and database connection messages
```

### Test Health Endpoint
```bash
# After deployment
curl https://your-api.onrender.com/api/health
```

### Manual Migration
```bash
# If migrations fail, run manually
psql "$DATABASE_URL" < migrations/001_initial_schema.sql
```

## 📊 Performance Optimization

### Database Connection Pooling
- Set `DATABASE_POOL_SIZE` based on your plan
- Free tier: 5 connections
- Paid tier: Up to 25 connections

### Caching Configuration
- Adjust `CACHE_TTL_SECONDS` for your use case
- Monitor memory usage with `CACHE_MAX_CAPACITY`

### Logging
- Production: `RUST_LOG=info`
- Debug: `RUST_LOG=debug,three_pages_api=trace`

## 🔄 Deployment Workflow

1. **Database First**: Always create database before API service
2. **Environment Variables**: Set all required variables
3. **Test Locally**: Use same DATABASE_URL locally first
4. **Deploy Service**: Create web service after database is ready
5. **Monitor Logs**: Check deployment logs for errors
6. **Health Check**: Verify `/api/health` endpoint works

## 🆘 Common Issues & Solutions

### Issue: "Database does not exist"
**Solution**: Ensure database name matches exactly in connection string

### Issue: "Permission denied"
**Solution**: Check database user has correct permissions

### Issue: "Connection timeout"
**Solution**: Database and API should be in same Render region

### Issue: "Migration already applied"
**Solution**: This is normal on redeployment, can be ignored

### Issue: "Port already in use"
**Solution**: Use `PORT` environment variable, don't hardcode port

## 📈 Monitoring & Maintenance

### Key Metrics to Monitor
- Database connection pool usage
- API response times
- Error rates
- Memory usage

### Log Analysis
Look for these patterns:
- `Database connection established` ✓
- `Database migrations completed` ✓
- `Server starting on` ✓
- `error communicating with database` ❌

### Health Checks
- Endpoint: `https://your-api.onrender.com/api/health`
- Should return: `200 OK` with JSON status
- Includes database, cache, and external API status

## 🎯 Success Checklist

- [ ] Database service created and running
- [ ] All environment variables set
- [ ] API service deployed successfully
- [ ] Health check endpoint returns 200
- [ ] Database migrations completed
- [ ] API endpoints responding correctly
- [ ] CORS configured for frontend domain
- [ ] Logs show no critical errors

## 📞 Getting Help

1. **Check logs** in Render dashboard first
2. **Run health check** script locally
3. **Verify environment** variables
4. **Test database** connection independently
5. **Check GitHub issues** for similar problems

Your Three Pages API should now be successfully deployed on Render! 🎉