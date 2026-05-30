#!/bin/bash

set -e

echo "🚀 Starting Cloudflare deployment for Connect Four..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "package.json" ] || [ ! -f "wrangler.toml" ]; then
    print_error "Must be run from the project root directory"
    exit 1
fi

# Check if wrangler is installed
if ! command -v wrangler &> /dev/null; then
    print_error "wrangler CLI not found. Install with: npm install -g wrangler"
    exit 1
fi

print_status "Building application..."

print_status "Building for Cloudflare..."
npm run build:cf

print_success "Build completed successfully!"

print_status "Running database migrations..."
wrangler d1 migrations apply connect-four-db --remote

print_success "Database migrations completed!"

print_status "Deploying to Cloudflare Workers..."
wrangler deploy

print_success "Deployment completed successfully!"
print_success "Your app is live at: https://connect-4.tre.systems"

print_status "Checking deployment status..."
wrangler tail --format pretty

echo ""
print_success "🎉 Connect Four is now deployed on Cloudflare!"
echo ""
echo "📊 Useful commands:"
echo "  • View logs: wrangler tail"
echo "  • Check status: wrangler status"
echo "  • Database shell: wrangler d1 execute connect-four-db --command 'SELECT * FROM games LIMIT 5;'"
echo "  • Local development: npm run dev"
echo "" 