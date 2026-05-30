# Deployment Guide

This guide covers deploying and managing the Connect Four application on Cloudflare Workers, D1 Database, and custom domains.

## 🚀 Quick Deployment

### Recommended: Standard Deployment

The most reliable way to deploy is using the automated script:

```bash
npm run deploy
```

This script handles the entire pipeline:

1. Builds WebAssembly assets & bindings
2. Generates Service Worker
3. Builds the Next.js application (OpenNext)
4. Applies pending D1 database migrations
5. Deploys to Cloudflare Workers

### Alternative: Quick Deploy

If you want to skip the custom script checks and just build/deploy:

```bash
npm run deploy:quick
```

_Note: This runs `build:cf` (which includes WASM build) and `wrangler deploy`._

### Manual / Debug Deployment

If you need to step through the process manually:

```bash
# 1. Build everything (WASM + Frontend)
npm run build:cf

# 2. Run D1 migrations
npm run db:migrate

# 3. Deploy to Cloudflare
wrangler deploy
```

## 📋 Prerequisites

### Required Tools

1. **Node.js 20+** and npm
2. **Rust** and Cargo
3. **wasm-pack**: `cargo install wasm-pack --version 0.13.1 --locked` (matches the version pinned in CI)
4. **Wrangler CLI**: `npm install -g wrangler`

### Cloudflare Account Setup

1. **Create Cloudflare Account**: [Sign up here](https://dash.cloudflare.com/sign-up)
2. **Get Account ID**: Found in Cloudflare dashboard
3. **Create D1 Database**:
   ```bash
   wrangler d1 create connect-four-db
   ```
4. **Add Custom Domain**: Configure DNS for your domain

## ⚙️ Configuration

### wrangler.toml

Your current configuration includes:

```toml
name = "connect-four-main"
main = ".open-next/worker.js"
compatibility_date = "2025-06-14"
compatibility_flags = ["nodejs_compat", "global_fetch_strictly_public"]

[assets]
directory = ".open-next/assets"
binding = "ASSETS"

[vars]
ENVIRONMENT = "production"

[observability]
enabled = true
head_sampling_rate = 1

[[routes]]
pattern = "connect-4.tre.systems/*"
zone_name = "tre.systems"

[[d1_databases]]
binding = "DB"
database_name = "connect-four-db"
database_id = "15bf99a9-0f64-41a0-a895-361c1cf15757"
preview_database_id = "connect-four-db-preview"
migrations_dir = "migrations"
```

### Environment Variables

Add any environment variables to `wrangler.toml`:

```toml
[vars]
ENVIRONMENT = "production"
API_KEY = "your-api-key"
```

For secrets, use:

```bash
wrangler secret put SECRET_NAME
```

## 🗄️ Database Management

### D1 Database Commands

```bash
# View database info
wrangler d1 info connect-four-db

# Run migrations
npm run db:migrate

# Execute SQL commands
wrangler d1 execute connect-four-db --command "SELECT * FROM games LIMIT 10;"

# Open database shell
npm run db:shell

# Backup database
wrangler d1 export connect-four-db --output backup.sql

# Restore database
wrangler d1 execute connect-four-db --file backup.sql
```

### Local Development Database

```bash
# Reset local database
npm run db:setup

# Run local migrations
npm run migrate:local
```

## 📊 Monitoring and Debugging

### View Logs

```bash
# Real-time logs
npm run logs

# JSON format
wrangler tail --format json

# Filter errors
wrangler tail | grep "ERROR"
```

### Performance Monitoring

```bash
# Check deployment status
wrangler status

# View analytics
wrangler analytics

# Check worker performance
wrangler tail --format pretty | grep "duration"
```

### Debugging

```bash
# Test locally
wrangler dev

# Test specific routes
wrangler dev --test-scheduled

# Check configuration
wrangler config
```

## 🔄 CI/CD Integration

### GitHub Actions

Deployment is automated by [`.github/workflows/deploy.yml`](../.github/workflows/deploy.yml). It triggers on **push to `main`** (and manual `workflow_dispatch`) and runs:

1. Checkout, set up Node 20 + Rust (`dtolnay/rust-toolchain@stable`) + `wasm-pack` v0.13.1
2. `npm ci --legacy-peer-deps`
3. Install Playwright browsers
4. `npm run build:wasm-assets`
5. `npm run check` — the full gate: lint, type-check, Rust AI matrix test, unit coverage, and Playwright e2e
6. `npm run build:cf` (OpenNext build)
7. `wrangler deploy`

> **Note:** the workflow does **not** apply D1 migrations — that step is intentionally a no-op echo. Migrations are applied out-of-band by the `npm run deploy` script (`scripts/deploy.sh`, which runs `wrangler d1 migrations apply … --remote`) or manually via `npm run db:migrate`. If you add a schema change, apply the migration yourself; pushing to `main` will not run it.

### Environment Secrets

Set up these secrets in your GitHub repository:

- `CLOUDFLARE_API_TOKEN`: Your Cloudflare API token
- `CLOUDFLARE_ACCOUNT_ID`: Your Cloudflare account ID

## 🌐 Custom Domain Setup

### DNS Configuration

1. **Add Domain to Cloudflare**:
   - Go to Cloudflare dashboard
   - Add your domain
   - Update nameservers at your registrar

2. **Configure DNS Records**:

   ```
   Type: CNAME
   Name: connect-4
   Target: connect-four-main.your-subdomain.workers.dev
   Proxy: Enabled (orange cloud)
   ```

3. **SSL/TLS Settings**:
   - Set SSL/TLS mode to "Full (strict)"
   - Enable "Always Use HTTPS"

### Route Configuration

Update `wrangler.toml` with your domain:

```toml
[[routes]]
pattern = "your-domain.com/*"
zone_name = "your-domain.com"
```

## 🔧 Troubleshooting

### Common Issues

| Issue               | Quick Fix                                |
| ------------------- | ---------------------------------------- |
| Build failures      | `npm run nuke && npm run build:cf`       |
| Database connection | `wrangler d1 info connect-four-db`       |
| WASM loading        | `npm run build:wasm-assets`              |
| Deployment failures | Check `wrangler.toml` and authentication |

### Build Issues

```bash
# Clean and rebuild
npm run nuke
npm run build:cf
```

### Database Issues

```bash
# Check database status
wrangler d1 info connect-four-db

# Test connection
wrangler d1 execute connect-four-db --command "SELECT 1;"
```

### WASM Loading Issues

```bash
# Rebuild WASM assets
npm run build:wasm-assets

# Check asset paths
ls -la .open-next/assets/
```

### Deployment Failures

```bash
# Check wrangler version
wrangler --version

# Update wrangler
npm install -g wrangler@latest

# Check authentication
wrangler whoami
```

### Authentication Issues

```bash
wrangler login
wrangler whoami
```

## 📈 Performance Optimization

### Asset Optimization

1. **WASM Files**: Ensure proper caching with appropriate headers
2. **Cache Headers**: Use appropriate cache headers for static assets
3. **Image Optimization**: Optimize image assets for web delivery

### Database Optimization

1. **Indexes**: Add indexes for frequently queried columns
2. **Connection Pooling**: Use connection pooling where appropriate
3. **Query Performance**: Monitor and optimize slow queries

### Worker Optimization

1. **Bundle Size**: Minimize worker bundle size
2. **Compatibility Flags**: Use appropriate compatibility flags
3. **Cold Start Times**: Monitor and optimize cold start performance

## 🔐 Security

### Best Practices

1. **Environment Variables**: Use secrets for sensitive data
2. **CORS Configuration**: Restrict origins appropriately
3. **Rate Limiting**: Implement rate limiting for API endpoints
4. **Input Validation**: Validate all user inputs
5. **HTTPS Only**: Force HTTPS for all requests

### Security Headers

Configure security headers in your application:

```typescript
// Add to your Next.js config
const securityHeaders = [
  {
    key: 'X-Frame-Options',
    value: 'DENY',
  },
  {
    key: 'X-Content-Type-Options',
    value: 'nosniff',
  },
  {
    key: 'Referrer-Policy',
    value: 'origin-when-cross-origin',
  },
];
```

## 📊 Analytics and Monitoring

### Cloudflare Analytics

- **Web Analytics**: Built into Cloudflare dashboard
- **Workers Analytics**: Monitor function execution
- **D1 Analytics**: Database performance metrics

### Custom Monitoring

```bash
# Set up custom metrics
wrangler tail --format json | jq '.metrics'

# Monitor specific endpoints
wrangler tail --format pretty | grep "/api/"
```

## 🆘 Emergency Commands

### Rollback

```bash
wrangler rollback
```

### Emergency Deploy

```bash
npm run deploy:quick
```

### Check the live site

```bash
curl -I https://connect-4.tre.systems/
```

## 📚 Additional Resources

- [Cloudflare Workers Documentation](https://developers.cloudflare.com/workers/)
- [D1 Database Documentation](https://developers.cloudflare.com/d1/)
- [OpenNext Documentation](https://open-next.js.org/)
- [Wrangler CLI Reference](https://developers.cloudflare.com/workers/wrangler/)

## 🆘 Support

If you encounter issues:

1. Check the troubleshooting section above
2. Review Cloudflare Workers logs
3. Check the GitHub Issues for known problems
4. Consult the Cloudflare community forums

---

**Last Updated**: May 2026  
**Status**: Production Ready ✅
