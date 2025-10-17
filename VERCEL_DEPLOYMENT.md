# Vercel Deployment Guide

This guide walks you through deploying the Three Pages web application to Vercel.

## 📋 Prerequisites

- [Vercel account](https://vercel.com)
- [Vercel CLI](https://vercel.com/cli) installed globally: `npm i -g vercel`
- Your API backend deployed (currently on Render at `https://three-pages-api.onrender.com`)
- Node.js 18+ and pnpm installed

## 🚀 Quick Deployment

### Option 1: Deploy via Vercel CLI

1. **Login to Vercel:**
   ```bash
   vercel login
   ```

2. **Navigate to project root:**
   ```bash
   cd three-pages
   ```

3. **Deploy:**
   ```bash
   vercel --prod
   ```

### Option 2: Deploy via GitHub Integration

1. **Push your code to GitHub**
2. **Go to [Vercel Dashboard](https://vercel.com/dashboard)**
3. **Click "Add New Project"**
4. **Import your repository**
5. **Configure the project settings** (see configuration below)

## ⚙️ Configuration

### Project Settings

When setting up the project in Vercel dashboard:

- **Framework Preset:** `Vite`
- **Root Directory:** `./` (project root)
- **Build Command:** `cd apps/web && pnpm install && pnpm run build`
- **Output Directory:** `apps/web/dist`
- **Install Command:** `pnpm install`

### Environment Variables

Add these environment variables in your Vercel project settings:

| Variable | Value | Description |
|----------|-------|-------------|
| `VITE_API_URL` | `https://three-pages-api.onrender.com` | Your API backend URL |
| `VITE_API_TIMEOUT` | `120000` | API timeout in milliseconds |
| `VITE_MAX_SUMMARY_LENGTH` | `1000` | Maximum summary length |

**To add environment variables:**
1. Go to your project dashboard on Vercel
2. Click on "Settings" tab
3. Click on "Environment Variables"
4. Add each variable for Production, Preview, and Development environments

## 📁 File Structure

The deployment uses these key files:

```
three-pages/
├── vercel.json                 # Project-level Vercel config
├── .vercelignore              # Files to exclude from deployment
├── apps/
│   └── web/
│       ├── vercel.json        # App-specific config
│       ├── .env.local.example # Environment template
│       └── dist/              # Build output (generated)
└── packages/                  # Shared packages
```

## 🔧 Configuration Files

### `vercel.json`
- Configures the monorepo deployment
- Sets build commands and output directory
- Defines environment variables
- Sets up routing and caching

### `.vercelignore`
- Excludes API code (deployed separately on Render)
- Excludes development files and caches
- Reduces deployment bundle size

## 🌐 Domain Configuration

### Custom Domain
1. Go to project settings in Vercel dashboard
2. Click "Domains" tab
3. Add your custom domain
4. Follow DNS configuration instructions

### Preview Deployments
- Every push to non-main branches creates a preview deployment
- Preview URLs are automatically generated
- Perfect for testing before production

## 🚨 Troubleshooting

### Build Failures

**Issue:** "Module not found" errors
**Solution:** Ensure all workspace dependencies are properly configured in `package.json`

**Issue:** "No such file or directory" errors
**Solution:** Ensure build command is `pnpm run build` and runs from project root

### API Connection Issues

**Issue:** "Network Error" in production
**Solution:**
1. Verify `VITE_API_URL` environment variable is set correctly
2. Ensure your API backend is running and accessible
3. Check CORS settings on your API backend

### Build Timeouts

**Issue:** Build taking too long
**Solution:**
1. Review `.vercelignore` to exclude unnecessary files
2. Consider optimizing dependencies
3. Check if build cache is working properly

## 📈 Performance Optimization

### Automatic Optimizations
- Static asset caching (1 year for `/assets/`)
- Brotli compression
- Tree shaking via Vite

### Manual Optimizations
1. **Images:** Use Next.js Image component or optimize manually
2. **Bundle Size:** Analyze with `pnpm run build` and check bundle sizes
3. **Code Splitting:** Leverage React.lazy() for large components

## 🔒 Security

### Environment Variables
- Never commit `.env.local` files
- Use Vercel's environment variable dashboard
- `VITE_` prefixed variables are exposed to client-side code

### Headers
Security headers are configured in `vercel.json`:
- Cache-Control for static assets
- CORS handling via API backend

## 📊 Monitoring

### Vercel Analytics
1. Enable in project settings
2. Monitor Core Web Vitals
3. Track performance metrics

### Error Tracking
Consider integrating:
- Sentry for error tracking
- Vercel Analytics for performance
- Custom logging for debugging

## 🔄 CI/CD Pipeline

The deployment automatically:
1. **Installs dependencies** using pnpm
2. **Builds the web app** using Vite
3. **Deploys static files** to Vercel's CDN
4. **Runs on every push** to connected Git repository

## 💡 Best Practices

1. **Use preview deployments** for testing
2. **Monitor build times** and optimize as needed
3. **Keep environment variables secure**
4. **Use custom domains** for production
5. **Enable analytics** for performance insights
6. **Test mobile performance** regularly

## 🆘 Support

- [Vercel Documentation](https://vercel.com/docs)
- [Vite Deployment Guide](https://vitejs.dev/guide/static-deploy.html)
- [pnpm Monorepo Guide](https://pnpm.io/workspaces)

## 🎯 Next Steps After Deployment

1. **Test the deployed application** thoroughly
2. **Set up custom domain** if needed
3. **Configure analytics** and monitoring
4. **Set up alerts** for downtime or errors
5. **Document the deployment process** for your team

---

Your Three Pages application should now be successfully deployed to Vercel! 🎉
