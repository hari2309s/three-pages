# Database Connection Troubleshooting Guide

## Current Issue
Your Three Pages API is failing to connect to the PostgreSQL database on Render with the error:
```
Error: Database connection failed: Database error: error communicating with database: Network is unreachable (os error 101)
```

## What This Error Means
- **Network is unreachable (101)**: The API service cannot establish a network connection to the database
- This is typically a networking/infrastructure issue, not a code problem
- The database service may not be ready, or the services aren't properly linked

## Quick Diagnostics

### 1. Check Service Status
In your Render dashboard:
- ✅ **Database Service**: Should show "Live" status
- ✅ **API Service**: Should show "Deploy failed" or "Exited" 
- ⚠️ **Database Provisioning**: May take 5-10 minutes for new databases

### 2. Verify Environment Variables
In your API service settings → Environment:
- `DATABASE_URL` should be automatically populated from database
- Format should be: `postgres://user:password@host:port/database`
- Should NOT be empty or show "***"

### 3. Check Service Regions
Both services MUST be in the same region:
- Database region: Check in database dashboard
- API region: Check in web service dashboard  
- ❌ Different regions = connection will fail

## Step-by-Step Troubleshooting

### Step 1: Manual Database Service Creation
If using render.yaml isn't working, create manually:

1. **Create Database First**:
   - Go to Render Dashboard → "New +" → "PostgreSQL"
   - Name: `three-pages-db`
   - Database: `three_pages` 
   - User: `three_pages_user`
   - Plan: Free
   - Region: Oregon (recommended)
   - **Wait for "Live" status before proceeding**

2. **Get Connection Details**:
   - Go to database dashboard
   - Copy "Internal Database URL" (starts with postgres://)
   - This is what your API will use

### Step 2: Update API Service
1. **Update Environment Variables**:
   - Go to API service → Settings → Environment
   - Set `DATABASE_URL` to the Internal Database URL from Step 1
   - Ensure other required variables are set:
     ```
     DATABASE_URL=postgres://user:pass@host:port/database
     HF_TOKEN=your_hugging_face_token
     GOOGLE_BOOKS_API_KEY=your_google_books_key
     ALLOWED_ORIGINS=https://your-frontend.vercel.app
     PORT=10000
     ENVIRONMENT=production
     ```

2. **Trigger Redeploy**:
   - Manual Deploy → "Deploy Latest Commit"
   - Or push a new commit to trigger automatic deployment

### Step 3: Verify Connection
After deployment:
1. **Check Logs**:
   - Look for: `Database connection established successfully`
   - Or errors with more specific details

2. **Test Health Endpoint**:
   ```bash
   curl https://your-api.onrender.com/api/health
   ```

## Common Solutions

### Solution 1: Service Dependencies
Update your `render.yaml`:
```yaml
services:
  - type: pserv
    name: three-pages-db
    plan: free
    databaseName: three_pages
    user: three_pages_user
    region: oregon

  - type: web
    name: three-pages-api
    runtime: rust
    plan: free
    region: oregon  # Same region as database
    rootDir: apps/api
    buildCommand: cargo build --release
    startCommand: ./target/release/three-pages-api
    dependsOn:      # Wait for database
      - three-pages-db
    envVars:
      - key: DATABASE_URL
        fromDatabase:
          name: three-pages-db
          property: connectionString
```

### Solution 2: Manual Service Setup
Instead of render.yaml, create services manually:
1. Create database service first
2. Wait for it to be fully provisioned
3. Create web service with database URL

### Solution 3: Connection Pool Settings
Reduce connection pool size for free tier:
```yaml
- key: DATABASE_POOL_SIZE
  value: 3  # Reduced from 5
```

## Advanced Troubleshooting

### Check Database Connectivity
If you have `psql` installed locally:
```bash
# Test connection from your machine
psql "your_database_url_here"
```

### Service Logs Analysis
Look for these patterns in API service logs:
- ✅ `Database connection established successfully`
- ✅ `Database migrations completed successfully` 
- ❌ `Network is unreachable`
- ❌ `Connection refused`
- ❌ `timeout`

### Database Service Logs
Check database service logs for:
- Connection attempts from API service
- Authentication failures
- Resource limits

## Alternative Approaches

### Option 1: External Database
Use a managed PostgreSQL service:
- **Supabase**: Free tier with good integration
- **Neon**: Serverless PostgreSQL
- **Railway**: Simple setup with good Render integration

### Option 2: Render Blueprint
Create a render blueprint file instead of render.yaml:
```yaml
# render-blueprint.yaml
services:
- type: pserv
  name: three-pages-db
  plan: starter
  
- type: web
  name: three-pages-api
  env: rust
  plan: starter
  buildCommand: cargo build --release
  startCommand: ./target/release/three-pages-api
  envVars:
  - key: DATABASE_URL
    fromDatabase:
      name: three-pages-db
      property: connectionString
```

## Checklist Before Deployment

- [ ] Database service created and shows "Live" status
- [ ] Both services in same region
- [ ] DATABASE_URL environment variable populated
- [ ] All required environment variables set
- [ ] API service has `dependsOn: [three-pages-db]`
- [ ] Connection pool size appropriate for plan (≤3 for free)
- [ ] No typos in service names or database references

## Getting Help

### Check These Resources:
1. **Render Dashboard Logs**: Most detailed error information
2. **Render Status Page**: Check for service outages
3. **Render Documentation**: PostgreSQL setup guides
4. **GitHub Issues**: Search for similar database connection problems

### Information to Provide When Asking for Help:
- Exact error message from logs
- Service regions for both database and API
- Whether services were created manually or via render.yaml
- DATABASE_URL format (sanitized - don't share actual credentials)
- Timing: How long after database creation did you deploy API?

---

**Next Steps**: Start with manual service creation if render.yaml approach isn't working. The database must be fully provisioned before the API can connect to it.