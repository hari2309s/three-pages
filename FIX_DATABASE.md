# 🚨 IMMEDIATE DATABASE CONNECTION FIX

Your API is failing with: `Network is unreachable (os error 101)`

This means the database service isn't properly linked. Here's the **immediate fix**:

## 🔧 Quick Fix (Choose One Approach)

### Option A: Manual Service Creation (RECOMMENDED)

1. **Delete current services** (if they exist):
   - Go to Render Dashboard
   - Delete `three-pages-api` web service
   - Delete `three-pages-db` database service

2. **Create Database First**:
   - New + → PostgreSQL
   - Name: `three-pages-db`  
   - Database: `three_pages`
   - User: `three_pages_user`
   - Plan: Free
   - Region: **Oregon**
   - Click "Create Database"
   - ⏳ **WAIT** until status shows "Live" (5-10 minutes)

3. **Get Database URL**:
   - Go to database dashboard
   - Copy "Internal Database URL"
   - Should look like: `postgres://user:pass@host.render.com:5432/db`

4. **Create Web Service**:
   - New + → Web Service  
   - Connect your GitHub repo
   - Name: `three-pages-api`
   - Runtime: Rust
   - Plan: Free
   - Region: **Oregon** (same as database)
   - Root Directory: `apps/api`
   - Build Command: `cargo build --release`
   - Start Command: `./target/release/three-pages-api`

5. **Set Environment Variables**:
   ```
   DATABASE_URL = [paste the Internal Database URL from step 3]
   HF_TOKEN = [your Hugging Face token]
   GOOGLE_BOOKS_API_KEY = [your Google Books key]
   ALLOWED_ORIGINS = https://three-pages-web.vercel.app
   PORT = 10000
   ENVIRONMENT = production
   DATABASE_POOL_SIZE = 3
   RUST_LOG = info,three_pages_api=debug
   CACHE_TTL_SECONDS = 3600
   CACHE_MAX_CAPACITY = 1000
   ```

6. **Deploy**:
   - Click "Create Web Service"
   - Monitor logs for: `Database connection established successfully`

### Option B: Fix Current render.yaml

If you prefer Infrastructure as Code:

1. **Update render.yaml** regions:
   ```yaml
   services:
     - type: pserv
       name: three-pages-db
       plan: free
       region: oregon          # Add this
       databaseName: three_pages
       user: three_pages_user
   
     - type: web  
       name: three-pages-api
       runtime: rust
       plan: free
       region: oregon          # Add this - SAME as database
       rootDir: apps/api
       # ... rest of config
   ```

2. **Delete and recreate** services for region change to take effect

## 🔍 Verify Fix

After deployment, check:

1. **Logs show**:
   ```
   Database connection established successfully
   Database migrations completed successfully
   Server starting on 0.0.0.0:10000
   ```

2. **Health endpoint works**:
   ```bash
   curl https://three-pages-api.onrender.com/api/health
   ```

## ⚠️ Common Mistakes

- ❌ **Different regions**: Database in Oregon, API in Ohio = connection fails
- ❌ **Database not ready**: Creating API before database is "Live"
- ❌ **Wrong URL**: Using External URL instead of Internal URL
- ❌ **Typo in service name**: `fromDatabase.name` must match exactly

## 🆘 Still Not Working?

1. **Check database status**: Must show "Live", not "Building"
2. **Verify same region**: Both services in same region
3. **Check environment variables**: DATABASE_URL should not be empty
4. **Wait longer**: Sometimes takes 15+ minutes for full provisioning

## 🎯 Expected Success

When working, you'll see:
```
Database connection established successfully on attempt 1
Database migrations completed successfully  
Server starting on 0.0.0.0:10000
```

Your API will be available at: `https://three-pages-api.onrender.com`

---

**Recommendation**: Use Option A (manual creation) first to get it working, then migrate to render.yaml later if needed.