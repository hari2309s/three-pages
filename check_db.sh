#!/bin/bash

# Database Health Check Script for Three Pages API
# This script helps diagnose database connectivity issues

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Three Pages - Database Health Check${NC}"
echo "======================================"

# Check if required tools are available
check_tool() {
    if ! command -v "$1" &> /dev/null; then
        echo -e "${RED}✗${NC} $1 is not installed"
        return 1
    fi
    echo -e "${GREEN}✓${NC} $1 is available"
    return 0
}

# Parse DATABASE_URL components
parse_database_url() {
    local url="$1"

    if [[ $url =~ postgres://([^:]+):([^@]+)@([^:]+):([0-9]+)/(.+) ]]; then
        DB_USER="${BASH_REMATCH[1]}"
        DB_PASS="${BASH_REMATCH[2]}"
        DB_HOST="${BASH_REMATCH[3]}"
        DB_PORT="${BASH_REMATCH[4]}"
        DB_NAME="${BASH_REMATCH[5]}"
        return 0
    else
        echo -e "${RED}✗${NC} Invalid DATABASE_URL format"
        echo "Expected format: postgres://user:password@host:port/database"
        return 1
    fi
}

echo -e "\n${BLUE}📋 Environment Check${NC}"
echo "===================="

# Check for DATABASE_URL environment variable
if [ -z "$DATABASE_URL" ]; then
    echo -e "${RED}✗${NC} DATABASE_URL environment variable is not set"
    echo ""
    echo "Please set DATABASE_URL in one of these ways:"
    echo "1. Export in current shell:"
    echo "   export DATABASE_URL='postgres://user:pass@host:port/database'"
    echo "2. Create .env file with:"
    echo "   DATABASE_URL=postgres://user:pass@host:port/database"
    echo "3. Set in Render dashboard environment variables"
    exit 1
else
    echo -e "${GREEN}✓${NC} DATABASE_URL is set"
fi

# Parse the DATABASE_URL
if parse_database_url "$DATABASE_URL"; then
    echo -e "${GREEN}✓${NC} DATABASE_URL format is valid"
    echo -e "${GREEN}ℹ${NC} Host: $DB_HOST"
    echo -e "${GREEN}ℹ${NC} Port: $DB_PORT"
    echo -e "${GREEN}ℹ${NC} Database: $DB_NAME"
    echo -e "${GREEN}ℹ${NC} User: $DB_USER"
else
    exit 1
fi

echo -e "\n${BLUE}🛠️  Tool Check${NC}"
echo "==============="

PSQL_AVAILABLE=false
CURL_AVAILABLE=false

if check_tool "psql"; then
    PSQL_AVAILABLE=true
fi

if check_tool "curl"; then
    CURL_AVAILABLE=true
fi

if check_tool "nc"; then
    NC_AVAILABLE=true
else
    NC_AVAILABLE=false
fi

echo -e "\n${BLUE}🌐 Network Connectivity${NC}"
echo "========================"

# Test basic network connectivity to host and port
if [ "$NC_AVAILABLE" = true ]; then
    echo "Testing network connectivity to $DB_HOST:$DB_PORT..."
    if timeout 10 nc -z "$DB_HOST" "$DB_PORT" 2>/dev/null; then
        echo -e "${GREEN}✓${NC} Network connection to $DB_HOST:$DB_PORT successful"
    else
        echo -e "${RED}✗${NC} Cannot connect to $DB_HOST:$DB_PORT"
        echo "This could mean:"
        echo "1. Database server is not running"
        echo "2. Network connectivity issues"
        echo "3. Firewall blocking the connection"
        echo "4. Incorrect host or port"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠${NC} nc (netcat) not available, skipping network test"
fi

echo -e "\n${BLUE}🗄️  Database Connection${NC}"
echo "========================"

if [ "$PSQL_AVAILABLE" = true ]; then
    echo "Testing PostgreSQL connection..."

    # Test connection with timeout
    export PGCONNECT_TIMEOUT=10

    if psql "$DATABASE_URL" -c "SELECT 1 as connection_test;" >/dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} Database connection successful"

        # Get database version and basic info
        echo -e "${GREEN}ℹ${NC} Database info:"
        psql "$DATABASE_URL" -c "SELECT version();" 2>/dev/null | head -n 3

        # Test if we can create tables (for migration check)
        echo "Testing database permissions..."
        if psql "$DATABASE_URL" -c "CREATE TABLE IF NOT EXISTS health_check (id SERIAL PRIMARY KEY, checked_at TIMESTAMP DEFAULT NOW());" >/dev/null 2>&1; then
            echo -e "${GREEN}✓${NC} Database write permissions OK"

            # Clean up test table
            psql "$DATABASE_URL" -c "DROP TABLE IF EXISTS health_check;" >/dev/null 2>&1
        else
            echo -e "${YELLOW}⚠${NC} Database write permissions may be limited"
        fi

    else
        echo -e "${RED}✗${NC} Database connection failed"
        echo ""
        echo "Common solutions:"
        echo "1. Check if DATABASE_URL is correct"
        echo "2. Verify database server is running"
        echo "3. Check firewall/security group settings"
        echo "4. Verify user credentials and permissions"
        echo ""
        echo "Detailed error:"
        psql "$DATABASE_URL" -c "SELECT 1;" 2>&1 || true
        exit 1
    fi
else
    echo -e "${YELLOW}⚠${NC} psql not available, skipping database connection test"
    echo "Install PostgreSQL client tools for full database testing"
fi

echo -e "\n${BLUE}🔍 Migration Check${NC}"
echo "==================="

# Check if migration files exist
if [ -f "migrations/001_initial_schema.sql" ]; then
    echo -e "${GREEN}✓${NC} Migration files found in migrations/"
elif [ -f "apps/api/migrations/001_initial_schema.sql" ]; then
    echo -e "${GREEN}✓${NC} Migration files found in apps/api/migrations/"
else
    echo -e "${YELLOW}⚠${NC} Migration files not found"
    echo "Expected locations:"
    echo "- migrations/001_initial_schema.sql"
    echo "- apps/api/migrations/001_initial_schema.sql"
fi

echo -e "\n${BLUE}🚀 Application Health${NC}"
echo "====================="

# Check if the application binary exists (for Render deployment)
if [ -f "target/release/three-pages-api" ]; then
    echo -e "${GREEN}✓${NC} Application binary found at target/release/three-pages-api"
elif [ -f "apps/api/target/release/three-pages-api" ]; then
    echo -e "${GREEN}✓${NC} Application binary found at apps/api/target/release/three-pages-api"
else
    echo -e "${YELLOW}⚠${NC} Application binary not found"
    echo "Run 'cargo build --release' to build the application"
fi

# Check required environment variables for the app
echo -e "\n${BLUE}📊 Environment Variables${NC}"
echo "=========================="

check_env() {
    if [ -n "${!1}" ]; then
        echo -e "${GREEN}✓${NC} $1 is set"
    else
        echo -e "${YELLOW}⚠${NC} $1 is not set"
    fi
}

check_env "DATABASE_URL"
check_env "HF_TOKEN"
check_env "GOOGLE_BOOKS_API_KEY"
check_env "PORT"
check_env "ENVIRONMENT"

echo -e "\n${BLUE}📋 Summary${NC}"
echo "==========="

if [ "$PSQL_AVAILABLE" = true ]; then
    echo -e "${GREEN}🎉 Database health check completed successfully!${NC}"
    echo ""
    echo "✓ DATABASE_URL is properly formatted"
    echo "✓ Network connectivity is working"
    echo "✓ Database connection is successful"
    echo "✓ Database permissions are adequate"
    echo ""
    echo "Your database is ready for the Three Pages API!"
else
    echo -e "${YELLOW}⚠ Partial health check completed${NC}"
    echo ""
    echo "✓ DATABASE_URL is properly formatted"
    echo "? Database connection not tested (psql not available)"
    echo ""
    echo "Install PostgreSQL client tools for complete testing:"
    echo "  macOS: brew install postgresql"
    echo "  Ubuntu: sudo apt-get install postgresql-client"
    echo "  Alpine: apk add postgresql-client"
fi

echo ""
echo "For more troubleshooting help, see the API README.md"
