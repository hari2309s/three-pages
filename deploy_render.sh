#!/bin/bash

# Three Pages API - Render Deployment Script
# This script helps deploy the Three Pages Rust API to Render

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
API_DIR="$SCRIPT_DIR/apps/api"
RENDER_YAML="$SCRIPT_DIR/render.yaml"

echo -e "${BLUE}🚀 Three Pages API - Render Deployment Helper${NC}"
echo "=============================================="
echo ""
echo "This script will guide you through deploying your API to Render."
echo ""

# Function to print section headers
print_section() {
    echo ""
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${PURPLE}$1${NC}"
    echo -e "${PURPLE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Function to check if a command exists
check_command() {
    if command -v "$1" >/dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} $1 is installed"
        return 0
    else
        echo -e "${RED}✗${NC} $1 is not installed"
        return 1
    fi
}

# Function to validate environment variables
validate_env_vars() {
    local missing_vars=()

    if [ -z "$HF_TOKEN" ] && [ -z "$APP_HUGGINGFACE_API_KEY" ]; then
        missing_vars+=("HF_TOKEN or APP_HUGGINGFACE_API_KEY")
    fi

    if [ -z "$DATABASE_URL" ] && [ -z "$APP_SUPABASE_URL" ]; then
        missing_vars+=("DATABASE_URL or APP_SUPABASE_URL")
    fi

    if [ ${#missing_vars[@]} -eq 0 ]; then
        echo -e "${GREEN}✓${NC} All required environment variables are set"
        return 0
    else
        echo -e "${RED}✗${NC} Missing environment variables:"
        for var in "${missing_vars[@]}"; do
            echo -e "${RED}  - $var${NC}"
        done
        return 1
    fi
}

# Function to test API build
test_build() {
    echo "Testing Rust build process..."
    if [ -d "$API_DIR" ]; then
        cd "$API_DIR"
        if cargo check --release >/dev/null 2>&1; then
            echo -e "${GREEN}✓${NC} Rust project builds successfully"
            cd "$SCRIPT_DIR"
            return 0
        else
            echo -e "${RED}✗${NC} Rust build failed"
            cd "$SCRIPT_DIR"
            return 1
        fi
    else
        echo -e "${RED}✗${NC} API directory not found at $API_DIR"
        return 1
    fi
}

# Function to check render.yaml configuration
validate_render_config() {
    if [ ! -f "$RENDER_YAML" ]; then
        echo -e "${RED}✗${NC} render.yaml not found"
        return 1
    fi

    echo -e "${GREEN}✓${NC} render.yaml exists"

    # Check for required sections
    if grep -q "type: pserv" "$RENDER_YAML"; then
        echo -e "${GREEN}✓${NC} Database service configured"
    else
        echo -e "${YELLOW}⚠${NC} Database service not found in render.yaml"
    fi

    if grep -q "type: web" "$RENDER_YAML"; then
        echo -e "${GREEN}✓${NC} Web service configured"
    else
        echo -e "${RED}✗${NC} Web service not found in render.yaml"
        return 1
    fi

    return 0
}

# Function to display deployment instructions
show_deployment_steps() {
    print_section "📋 DEPLOYMENT STEPS"

    echo "Follow these steps to deploy to Render:"
    echo ""
    echo -e "${CYAN}1. Create Database Service:${NC}"
    echo "   • Go to https://dashboard.render.com"
    echo "   • Click 'New +' → 'PostgreSQL'"
    echo "   • Name: three-pages-db"
    echo "   • Database: three_pages"
    echo "   • User: three_pages_user"
    echo "   • Plan: Free"
    echo "   • Click 'Create Database'"
    echo "   • ⚠️  WAIT for database to be fully provisioned!"
    echo ""

    echo -e "${CYAN}2. Get Database Connection String:${NC}"
    echo "   • Go to your database dashboard"
    echo "   • Copy the 'Internal Database URL'"
    echo "   • Format: postgres://user:pass@host:port/database"
    echo ""

    echo -e "${CYAN}3. Create Web Service:${NC}"
    echo "   • Go to https://dashboard.render.com"
    echo "   • Click 'New +' → 'Web Service'"
    echo "   • Connect your GitHub repository"
    echo "   • Name: three-pages-api"
    echo "   • Runtime: Rust"
    echo "   • Root Directory: apps/api"
    echo "   • Build Command: cargo build --release"
    echo "   • Start Command: ./target/release/three-pages-api"
    echo ""

    echo -e "${CYAN}4. Set Environment Variables:${NC}"
    echo "   Required variables:"
    echo "   • DATABASE_URL (from step 2)"
    echo "   • HF_TOKEN (your Hugging Face API token)"
    echo "   • GOOGLE_BOOKS_API_KEY (optional)"
    echo "   • ALLOWED_ORIGINS (your frontend URL)"
    echo "   • PORT=10000"
    echo "   • ENVIRONMENT=production"
    echo "   • DATABASE_POOL_SIZE=5"
    echo ""

    echo -e "${CYAN}5. Configure Health Check:${NC}"
    echo "   • Health Check Path: /api/health"
    echo ""

    echo -e "${CYAN}6. Deploy:${NC}"
    echo "   • Click 'Create Web Service'"
    echo "   • Monitor deployment logs"
    echo "   • Wait for successful deployment"
    echo ""
}

# Function to run deployment checks
run_checks() {
    print_section "🔍 PRE-DEPLOYMENT CHECKS"

    local errors=0

    echo "Checking prerequisites..."

    # Check for required tools
    check_command "cargo" || ((errors++))
    check_command "git" || echo -e "${YELLOW}⚠${NC} Git recommended for deployment"

    # Check project structure
    if [ -f "$API_DIR/Cargo.toml" ]; then
        echo -e "${GREEN}✓${NC} Cargo.toml found"
    else
        echo -e "${RED}✗${NC} Cargo.toml not found at $API_DIR"
        ((errors++))
    fi

    if [ -d "$API_DIR/src" ]; then
        echo -e "${GREEN}✓${NC} Source directory exists"
    else
        echo -e "${RED}✗${NC} Source directory not found"
        ((errors++))
    fi

    if [ -d "$SCRIPT_DIR/migrations" ] || [ -d "$API_DIR/migrations" ]; then
        echo -e "${GREEN}✓${NC} Database migrations found"
    else
        echo -e "${YELLOW}⚠${NC} Database migrations not found"
    fi

    # Validate configuration
    echo ""
    echo "Checking configuration..."
    validate_render_config || ((errors++))

    # Test build
    echo ""
    test_build || ((errors++))

    # Check environment variables if available
    echo ""
    echo "Checking environment variables..."
    if validate_env_vars; then
        echo "Environment looks good for deployment!"
    else
        echo -e "${YELLOW}⚠${NC} Some environment variables are missing"
        echo "You'll need to set these in the Render dashboard"
    fi

    return $errors
}

# Function to test deployed service
test_deployment() {
    print_section "🧪 TESTING DEPLOYED SERVICE"

    read -p "Enter your deployed API URL (e.g., https://your-api.onrender.com): " API_URL

    if [ -z "$API_URL" ]; then
        echo "No URL provided, skipping deployment test"
        return 0
    fi

    echo ""
    echo "Testing deployed service at: $API_URL"

    # Test health endpoint
    echo "Testing health endpoint..."
    if curl -f -s "$API_URL/api/health" >/dev/null; then
        echo -e "${GREEN}✓${NC} Health endpoint is responding"

        # Show health status
        echo ""
        echo "Health status:"
        curl -s "$API_URL/api/health" | python3 -m json.tool 2>/dev/null || curl -s "$API_URL/api/health"

    else
        echo -e "${RED}✗${NC} Health endpoint is not responding"
        echo "This could mean:"
        echo "  • Service is still starting up (wait a few minutes)"
        echo "  • Database connection issues"
        echo "  • Configuration problems"
        echo "  • Check the Render dashboard logs"
        return 1
    fi

    echo ""
    echo "Testing API endpoint..."
    if curl -f -s "$API_URL/api/books/search?q=test&limit=1" >/dev/null; then
        echo -e "${GREEN}✓${NC} API endpoints are working"
    else
        echo -e "${YELLOW}⚠${NC} API endpoints may not be fully functional"
        echo "Check the logs for more details"
    fi
}

# Function to show troubleshooting help
show_troubleshooting() {
    print_section "🔧 TROUBLESHOOTING"

    echo "Common deployment issues and solutions:"
    echo ""

    echo -e "${CYAN}Database Connection Error:${NC}"
    echo "  Error: 'Network is unreachable (os error 101)'"
    echo "  Solutions:"
    echo "    • Wait 5-10 minutes after database creation"
    echo "    • Verify DATABASE_URL is correctly set"
    echo "    • Ensure database and API are in same region"
    echo "    • Check database service is running"
    echo ""

    echo -e "${CYAN}Build Failures:${NC}"
    echo "  Error: 'cargo build failed'"
    echo "  Solutions:"
    echo "    • Ensure rootDir is set to 'apps/api'"
    echo "    • Check Cargo.toml dependencies"
    echo "    • Verify Rust version compatibility"
    echo ""

    echo -e "${CYAN}Migration Failures:${NC}"
    echo "  Error: 'Migration failed'"
    echo "  Solutions:"
    echo "    • Check database permissions"
    echo "    • Verify migration files exist"
    echo "    • Try recreating database"
    echo ""

    echo -e "${CYAN}Environment Variable Issues:${NC}"
    echo "  Error: 'Required environment variable missing'"
    echo "  Solutions:"
    echo "    • Double-check all variables are set in Render dashboard"
    echo "    • Verify variable names match exactly"
    echo "    • Check for typos in variable values"
    echo ""

    echo -e "${CYAN}Health Check Failures:${NC}"
    echo "  Error: 'Health check failed'"
    echo "  Solutions:"
    echo "    • Set health check path to '/api/health'"
    echo "    • Ensure service binds to PORT environment variable"
    echo "    • Check service startup logs"
    echo ""

    echo "For more help:"
    echo "  • Check Render dashboard logs"
    echo "  • Run './check_db.sh' to test database"
    echo "  • See RENDER_DEPLOYMENT.md for detailed guide"
}

# Function to show completion message
show_completion() {
    print_section "🎉 DEPLOYMENT COMPLETE"

    echo "Your Three Pages API should now be deployed to Render!"
    echo ""
    echo "Next steps:"
    echo "  1. Test your API endpoints"
    echo "  2. Update your frontend VITE_API_URL"
    echo "  3. Set up monitoring and alerts"
    echo "  4. Configure custom domain (optional)"
    echo ""
    echo "Important URLs:"
    echo "  • Render Dashboard: https://dashboard.render.com"
    echo "  • API Health Check: https://your-api.onrender.com/api/health"
    echo ""
    echo "Useful commands:"
    echo "  • Test database: ./check_db.sh"
    echo "  • View logs: Check Render dashboard"
    echo "  • Local development: cd apps/api && cargo run"
    echo ""
    echo -e "${GREEN}Happy deploying! 🚀${NC}"
}

# Main menu function
show_menu() {
    echo ""
    echo "What would you like to do?"
    echo ""
    echo "1) Run pre-deployment checks"
    echo "2) Show deployment instructions"
    echo "3) Test deployed service"
    echo "4) Show troubleshooting guide"
    echo "5) Exit"
    echo ""
    read -p "Enter your choice (1-5): " choice

    case $choice in
        1)
            if run_checks; then
                echo ""
                echo -e "${GREEN}✓ All checks passed! Ready for deployment.${NC}"
            else
                echo ""
                echo -e "${RED}✗ Some checks failed. Please fix issues before deploying.${NC}"
            fi
            show_menu
            ;;
        2)
            show_deployment_steps
            show_menu
            ;;
        3)
            test_deployment
            show_menu
            ;;
        4)
            show_troubleshooting
            show_menu
            ;;
        5)
            echo ""
            echo "Good luck with your deployment!"
            exit 0
            ;;
        *)
            echo "Invalid choice. Please enter 1-5."
            show_menu
            ;;
    esac
}

# Main execution
main() {
    # Check if we're in the right directory
    if [ ! -f "$SCRIPT_DIR/package.json" ] || [ ! -d "$API_DIR" ]; then
        echo -e "${RED}Error: This script must be run from the three-pages project root directory.${NC}"
        echo "Current directory: $SCRIPT_DIR"
        echo "Expected structure:"
        echo "  three-pages/"
        echo "  ├── apps/api/"
        echo "  ├── package.json"
        echo "  └── render.yaml"
        exit 1
    fi

    echo "Project directory: $SCRIPT_DIR"
    echo "API directory: $API_DIR"

    # Show main menu
    show_menu
}

# Check if script is being sourced or executed
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
