{
  description = "Radix-Leptos Primitives - A comprehensive UI component library built with Leptos and Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Use specific Rust version for stability
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
        
        # Node.js and pnpm
        nodejs = pkgs.nodejs_20;
        pnpm = pkgs.nodePackages.pnpm;
        
        # Development tools
        devTools = with pkgs; [
          # Rust tools
          rust
          cargo-watch
          wasm-pack
          
          # Node.js tools
          nodejs
          pnpm
          
          # System tools
          python3
          git
          
          # Optional: Additional development tools
          # clang
          # pkg-config
          # openssl
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = devTools;
          
          shellHook = ''
            echo "🚀 Welcome to Radix-Leptos Development Environment!"
            echo ""
            echo "Available tools:"
            echo "  • Rust: $(rustc --version)"
            echo "  • Cargo: $(cargo --version)"
            echo "  • wasm-pack: $(wasm-pack --version)"
            echo "  • Node.js: $(node --version)"
            echo "  • pnpm: $(pnpm --version)"
            echo ""
            echo "Quick start:"
            echo "  • Build WASM: cd examples && wasm-pack build"
            echo "  • Run tests: pnpm test"
            echo "  • Start server: python3 -m http.server 8080"
            echo ""
            
            # Set up environment variables
            export RUST_BACKTRACE=1
            export RUST_LOG=info
            
            # Ensure we're using the right toolchain
            export PATH="${pkgs.lib.makeBinPath devTools}:$PATH"
          '';
          
          # Environment variables for the shell
          RUST_BACKTRACE = "1";
          RUST_LOG = "info";
        };
        
        # Development commands
        apps = {
          build = {
            type = "app";
            program = toString (pkgs.writeShellScript "build" ''
              echo "🔨 Building Radix-Leptos..."
              cd examples
              wasm-pack build --target web
              echo "✅ Build complete!"
            '');
          };
          
          test = {
            type = "app";
            program = toString (pkgs.writeShellScript "test" ''
              echo "🧪 Running tests..."
              cd examples
              pnpm test
            '');
          };
          
          serve = {
            type = "app";
            program = toString (pkgs.writeShellScript "serve" ''
              echo "🌐 Starting development server..."
              cd examples
              python3 -m http.server 8080
            '');
          };
          
          dev = {
            type = "app";
            program = toString (pkgs.writeShellScript "dev" ''
              echo "🚀 Starting development environment..."
              cd examples
              
              # Build in background
              wasm-pack build --target web &
              BUILD_PID=$!
              
              # Start server
              python3 -m http.server 8080 &
              SERVER_PID=$!
              
              echo "✅ Development environment started!"
              echo "  • Server: http://localhost:8080"
              echo "  • WASM building in background..."
              echo ""
              echo "Press Ctrl+C to stop all services"
              
              # Wait for interrupt
              trap "kill $BUILD_PID $SERVER_PID 2>/dev/null; exit" INT
              wait
            '');
          };
        };
        
        # Packages
        packages = {
          # Build the WASM examples
          examples = pkgs.stdenv.mkDerivation {
            name = "radix-leptos-examples";
            src = ./examples;
            buildInputs = [ rust pkgs.wasm-pack ];
            
            buildPhase = ''
              cd examples
              wasm-pack build --target web
            '';
            
            installPhase = ''
              mkdir -p $out
              cp -r pkg $out/
              cp -r *.html $out/
            '';
          };
          
          # Default package
          default = self.packages.${system}.examples;
        };
      }
    );
}
