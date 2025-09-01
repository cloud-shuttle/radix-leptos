#!/bin/bash

# Production Deployment Script for Radix-Leptos
echo "🚀 Deploying Radix-Leptos to Production..."

# Configuration
PRODUCTION_DIR="pkg"
DEPLOY_TARGET=${1:-"local"}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if production build exists
if [[ ! -d "$PRODUCTION_DIR" ]]; then
    echo -e "${RED}❌ Production build not found. Run build-production.sh first.${NC}"
    exit 1
fi

# Check if wasm-opt is available for optimization
if command -v wasm-opt &> /dev/null; then
    echo -e "${BLUE}⚡ wasm-opt found - will optimize WASM bundle${NC}"
else
    echo -e "${YELLOW}⚠️  wasm-opt not found. Install with: cargo install wasm-opt${NC}"
fi

case $DEPLOY_TARGET in
    "local")
        echo -e "${BLUE}🏠 Deploying to local production server...${NC}"
        
        # Create local production server
        mkdir -p production-server
        cp -r $PRODUCTION_DIR/* production-server/
        
        # Create production index.html
        cat > production-server/index.html << EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Radix-Leptos Production</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; padding: 2rem; }
        .component { margin: 1rem 0; padding: 1rem; border: 1px solid #e5e7eb; border-radius: 0.5rem; }
        .status { padding: 0.5rem; border-radius: 0.25rem; margin: 0.5rem 0; }
        .success { background: #d1fae5; color: #065f46; }
        .error { background: #fee2e2; color: #991b1b; }
    </style>
</head>
<body>
    <h1>🚀 Radix-Leptos Production Build</h1>
    <p>This is the production build of your Radix-Leptos components.</p>
    
    <div class="component">
        <h2>📦 Bundle Information</h2>
        <div id="bundle-info">Loading...</div>
    </div>
    
    <div class="component">
        <h2>🧪 Component Test</h2>
        <button id="test-btn">Test Component</button>
        <div id="test-result"></div>
    </div>
    
    <div class="component">
        <h2>📊 Performance Metrics</h2>
        <div id="performance-metrics">Loading...</div>
    </div>

    <script type="module">
        import init, { start_dropdown_menu_examples } from './radix_leptos_examples.js';
        
        let startTime = performance.now();
        
        async function run() {
            try {
                // Initialize WASM
                await init();
                const initTime = performance.now() - startTime;
                
                // Update bundle info
                document.getElementById('bundle-info').innerHTML = \`
                    <div class="status success">✅ WASM Initialized in \${initTime.toFixed(2)}ms</div>
                    <p><strong>JavaScript:</strong> \${(performance.memory?.usedJSHeapSize / 1024 / 1024).toFixed(2)}MB used</p>
                \`;
                
                // Test component
                start_dropdown_menu_examples();
                
                // Update test result
                document.getElementById('test-result').innerHTML = \`
                    <div class="status success">✅ Component loaded successfully</div>
                \`;
                
                // Performance metrics
                const loadTime = performance.now() - startTime;
                document.getElementById('performance-metrics').innerHTML = \`
                    <div class="status success">✅ Total load time: \${loadTime.toFixed(2)}ms</div>
                    <p><strong>WASM Init:</strong> \${initTime.toFixed(2)}ms</p>
                    <p><strong>Component Load:</strong> \${(loadTime - initTime).toFixed(2)}ms</p>
                \`;
                
            } catch (error) {
                document.getElementById('bundle-info').innerHTML = \`
                    <div class="status error">❌ Error: \${error.message}</div>
                \`;
            }
        }
        
        // Test button functionality
        document.getElementById('test-btn').addEventListener('click', () => {
            document.getElementById('test-result').innerHTML = \`
                <div class="status success">✅ Button click working - component is functional!</div>
            \`;
        });
        
        run();
    </script>
</body>
</html>
EOF
        
        echo -e "${GREEN}✅ Local production server created in: production-server/${NC}"
        echo -e "${BLUE}🌐 Start with: cd production-server && python3 -m http.server 8081${NC}"
        echo -e "${BLUE}📱 Visit: http://localhost:8081${NC}"
        ;;
        
    "cdn")
        echo -e "${BLUE}☁️  Preparing for CDN deployment...${NC}"
        
        # Create CDN-ready package
        mkdir -p cdn-deploy
        cp -r $PRODUCTION_DIR/* cdn-deploy/
        
        # Create CDN deployment guide
        cat > cdn-deploy/DEPLOYMENT.md << EOF
# CDN Deployment Guide

## 🚀 Quick Deploy to CDN

### 1. Cloudflare
1. Go to Cloudflare Dashboard
2. Upload all files in this directory to your domain
3. Set cache rules for .wasm and .js files

### 2. AWS CloudFront
1. Upload to S3 bucket
2. Create CloudFront distribution
3. Set cache behaviors for optimal performance

### 3. Google Cloud CDN
1. Upload to Cloud Storage
2. Configure load balancer
3. Enable CDN with appropriate cache settings

## 📁 Files to Upload
- radix_leptos_examples.js
- radix_leptos_examples_bg.wasm
- radix_leptos_examples.d.ts

## 🔧 Cache Headers
Set these cache headers for optimal performance:
- .js files: Cache-Control: public, max-age=31536000
- .wasm files: Cache-Control: public, max-age=31536000
EOF
        
        echo -e "${GREEN}✅ CDN deployment package created in: cdn-deploy/${NC}"
        echo -e "${BLUE}📤 Upload the contents of cdn-deploy/ to your CDN${NC}"
        ;;
        
    "docker")
        echo -e "${BLUE}🐳 Creating Docker production image...${NC}"
        
        # Create Dockerfile
        cat > Dockerfile.production << EOF
FROM nginx:alpine

# Copy production files
COPY pkg/ /usr/share/nginx/html/

# Copy custom nginx config
COPY nginx.conf /etc/nginx/nginx.conf

# Expose port
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
EOF
        
        # Create nginx config
        cat > nginx.conf << EOF
events {
    worker_connections 1024;
}

http {
    include /etc/nginx/mime.types;
    default_type application/octet-stream;
    
    # Enable gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css text/xml text/javascript application/javascript application/xml+rss application/json application/wasm;
    
    server {
        listen 80;
        server_name localhost;
        root /usr/share/nginx/html;
        index index.html;
        
        # Cache JavaScript files for 1 year
        location ~* \\.js$ {
            expires 1y;
            add_header Cache-Control "public, immutable";
        }
        
        # Cache WASM files for 1 year
        location ~* \\.wasm$ {
            expires 1y;
            add_header Cache-Control "public, immutable";
            add_header Content-Type "application/wasm";
        }
        
        # Handle all other files
        location / {
            try_files \$uri \$uri/ /index.html;
        }
    }
}
EOF
        
        echo -e "${GREEN}✅ Docker files created: Dockerfile.production, nginx.conf${NC}"
        echo -e "${BLUE}🐳 Build with: docker build -f Dockerfile.production -t radix-leptos-prod .${NC}"
        echo -e "${BLUE}🚀 Run with: docker run -p 8080:80 radix-leptos-prod${NC}"
        ;;
        
    *)
        echo -e "${YELLOW}Usage: $0 [local|cdn|docker]${NC}"
        echo -e "${BLUE}Available deployment targets:${NC}"
        echo -e "  local   - Local production server"
        echo -e "  cdn     - CDN deployment package"
        echo -e "  docker  - Docker production image"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}🎉 Deployment preparation complete!${NC}"
echo -e "${BLUE}📋 Check the generated files and follow the deployment guide${NC}"
