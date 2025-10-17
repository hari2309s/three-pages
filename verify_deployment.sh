#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Deployment Verification Script for Three Pages
echo -e "${BLUE}🔍 Three Pages - Vercel Deployment Verification${NC}"
echo "=================================================="

# Function to check if file exists
check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✓${NC} $1 exists"
        return 0
    else
        echo -e "${RED}✗${NC} $1 missing"
        return 1
    fi
}

# Function to check if command exists
check_command() {
    if command -v "$1" >/dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} $1 is installed"
        return 0
    else
        echo -e "${RED}✗${NC} $1 is not installed"
        return 1
    fi
}

# Initialize error count
ERRORS=0

echo -e "\n${BLUE}📋 Checking Prerequisites${NC}"
echo "=============================="

# Check for required commands
check_command "node" || ((ERRORS++))
check_command "pnpm" || ((ERRORS++))
check_command "vercel" || echo -e "${YELLOW}⚠${NC} vercel CLI not installed (optional for manual deployment)"

# Check Node.js version
NODE_VERSION=$(node --version | sed 's/v//')
REQUIRED_NODE="18.0.0"
if [ "$(printf '%s\n' "$REQUIRED_NODE" "$NODE_VERSION" | sort -V | head -n1)" = "$REQUIRED_NODE" ]; then
    echo -e "${GREEN}✓${NC} Node.js version $NODE_VERSION >= $REQUIRED_NODE"
else
    echo -e "${RED}✗${NC} Node.js version $NODE_VERSION < $REQUIRED_NODE"
    ((ERRORS++))
fi

echo -e "\n${BLUE}📁 Checking Configuration Files${NC}"
echo "=================================="

# Check Vercel configuration files
check_file "vercel.json" || ((ERRORS++))
check_file "apps/web/vercel.json" || echo -e "${YELLOW}⚠${NC} apps/web/vercel.json not found (optional - using project-level config)"
check_file ".vercelignore" || ((ERRORS++))

# Check package.json files
check_file "package.json" || ((ERRORS++))
check_file "apps/web/package.json" || ((ERRORS++))

# Check workspace files
check_file "pnpm-workspace.yaml" || ((ERRORS++))
check_file "pnpm-lock.yaml" || ((ERRORS++))

# Check environment files
check_file "apps/web/.env.local.example" || ((ERRORS++))
check_file "apps/web/.env.local" || echo -e "${YELLOW}⚠${NC} apps/web/.env.local not found (optional for development)"

# Check build configuration
check_file "apps/web/vite.config.js" || ((ERRORS++))
check_file "apps/web/tsconfig.json" || ((ERRORS++))

echo -e "\n${BLUE}🔧 Checking Build Configuration${NC}"
echo "===================================="

# Check package.json scripts
if grep -q '"build":' package.json; then
    echo -e "${GREEN}✓${NC} Root build script configured"
else
    echo -e "${RED}✗${NC} Root build script missing"
    ((ERRORS++))
fi

if grep -q '"vercel-build":' package.json; then
    echo -e "${GREEN}✓${NC} Vercel build script configured"
else
    echo -e "${RED}✗${NC} Vercel build script missing"
    ((ERRORS++))
fi

# Check web app scripts
if grep -q '"build":' apps/web/package.json; then
    echo -e "${GREEN}✓${NC} Web app build script configured"
else
    echo -e "${RED}✗${NC} Web app build script missing"
    ((ERRORS++))
fi

echo -e "\n${BLUE}🌐 Checking Environment Configuration${NC}"
echo "========================================"

# Check if environment variables are defined in vercel.json
if grep -q "VITE_API_URL" vercel.json; then
    echo -e "${GREEN}✓${NC} VITE_API_URL configured in vercel.json"
else
    echo -e "${RED}✗${NC} VITE_API_URL missing in vercel.json"
    ((ERRORS++))
fi

if grep -q "VITE_API_TIMEOUT" vercel.json; then
    echo -e "${GREEN}✓${NC} VITE_API_TIMEOUT configured in vercel.json"
else
    echo -e "${RED}✗${NC} VITE_API_TIMEOUT missing in vercel.json"
    ((ERRORS++))
fi

# Check if environment variables are used in code
if grep -q "VITE_API_URL" apps/web/src/lib/constants.ts 2>/dev/null; then
    echo -e "${GREEN}✓${NC} VITE_API_URL used in application code"
else
    echo -e "${YELLOW}⚠${NC} VITE_API_URL usage not found in constants.ts"
fi

echo -e "\n${BLUE}🔨 Testing Build Process${NC}"
echo "=========================="

# Install dependencies
echo "Installing dependencies..."
if pnpm install --silent; then
    echo -e "${GREEN}✓${NC} Dependencies installed successfully"
else
    echo -e "${RED}✗${NC} Failed to install dependencies"
    ((ERRORS++))
fi

# Test build
echo "Testing build process..."
if cd apps/web && pnpm run build >/dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} Build completed successfully"
    cd ../..
else
    echo -e "${RED}✗${NC} Build failed"
    cd ../.. 2>/dev/null || true
    ((ERRORS++))
fi

# Check build output
if [ -f "apps/web/dist/index.html" ]; then
    echo -e "${GREEN}✓${NC} Build output (index.html) exists"
else
    echo -e "${RED}✗${NC} Build output (index.html) missing"
    ((ERRORS++))
fi

if [ -d "apps/web/dist/assets" ]; then
    echo -e "${GREEN}✓${NC} Assets directory exists"
    ASSET_COUNT=$(find apps/web/dist/assets -type f | wc -l | tr -d ' ')
    echo -e "${GREEN}ℹ${NC} Found $ASSET_COUNT asset files"
else
    echo -e "${RED}✗${NC} Assets directory missing"
    ((ERRORS++))
fi

echo -e "\n${BLUE}📊 Build Analysis${NC}"
echo "=================="

# Check bundle sizes
if [ -f "apps/web/dist/index.html" ]; then
    DIST_SIZE=$(du -sh apps/web/dist 2>/dev/null | cut -f1)
    echo -e "${GREEN}ℹ${NC} Total dist size: $DIST_SIZE"

    # Check for large JavaScript files
    LARGE_JS=$(find apps/web/dist/assets -name "*.js" -size +500k 2>/dev/null | wc -l | tr -d ' ')
    if [ "$LARGE_JS" -gt 0 ]; then
        echo -e "${YELLOW}⚠${NC} Found $LARGE_JS JavaScript files larger than 500KB"
        echo -e "${YELLOW}  ${NC} Consider code splitting for better performance"
    else
        echo -e "${GREEN}✓${NC} All JavaScript chunks are under 500KB"
    fi
fi

echo -e "\n${BLUE}🚀 Deployment Readiness${NC}"
echo "========================="

# Check if docs app is completely removed
if [ -d "apps/docs" ]; then
    echo -e "${RED}✗${NC} docs app still exists (should be removed)"
    ((ERRORS++))
else
    echo -e "${GREEN}✓${NC} docs app successfully removed"
fi

# Check for any remaining docs references
DOCS_REFS=$(grep -r "apps/docs" . 2>/dev/null | wc -l | tr -d ' ')
if [ "$DOCS_REFS" -gt 0 ]; then
    echo -e "${YELLOW}⚠${NC} Found $DOCS_REFS references to apps/docs"
    echo "  Run: grep -r 'apps/docs' . to find and clean up"
else
    echo -e "${GREEN}✓${NC} No lingering docs app references"
fi

# Check if API deployment config exists but is separate
if [ -f "render.yaml" ] && ! grep -q "apps/web" render.yaml; then
    echo -e "${GREEN}✓${NC} API deployment config separate from web deployment"
else
    echo -e "${YELLOW}⚠${NC} Check render.yaml configuration"
fi

echo -e "\n${BLUE}📋 Summary${NC}"
echo "==========="

if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}🎉 All checks passed! Your app is ready for Vercel deployment.${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Run 'vercel --prod' to deploy"
    echo "2. Or push to GitHub and configure auto-deployment"
    echo "3. Set up environment variables in Vercel dashboard"
    echo "4. See VERCEL_DEPLOYMENT.md for detailed instructions"
    exit 0
else
    echo -e "${RED}❌ Found $ERRORS issues that need to be resolved before deployment.${NC}"
    echo ""
    echo "Please fix the issues marked with ✗ above and run this script again."
    exit 1
fi
