# 🔨 Maintenance Scripts

This directory contains scripts for building, deploying, and maintaining the Radix-Leptos project.

## 📁 Scripts Overview

### Build Scripts

#### `build.sh`
**Purpose**: Standard build process for development
**Features**:
- Compiles all crates
- Runs basic validation
- Generates development artifacts

**Usage**:
```bash
./scripts/maintenance/build.sh
```

#### `build-production.sh`
**Purpose**: Production build with optimizations
**Features**:
- Optimized compilation
- Minified assets
- Production-ready artifacts
- Performance optimizations

**Usage**:
```bash
./scripts/maintenance/build-production.sh
```

### Deployment Scripts

#### `deploy-production.sh`
**Purpose**: Production deployment automation
**Features**:
- Automated deployment process
- Environment validation
- Rollback capabilities
- Health checks

**Usage**:
```bash
./scripts/maintenance/deploy-production.sh
```

### Test Scripts

#### `run-tests.sh`
**Purpose**: Comprehensive test execution
**Features**:
- Unit tests
- Integration tests
- E2E tests
- Performance tests
- Test reporting

**Usage**:
```bash
./scripts/maintenance/run-tests.sh
```

## 🎯 Usage Guidelines

### Development Workflow

1. **Before development**:
   ```bash
   ./scripts/maintenance/build.sh
   ```

2. **During development**:
   ```bash
   ./scripts/maintenance/run-tests.sh
   ```

3. **Before release**:
   ```bash
   ./scripts/maintenance/build-production.sh
   ```

4. **For deployment**:
   ```bash
   ./scripts/maintenance/deploy-production.sh
   ```

### Script Safety

- All scripts include error handling
- Scripts validate prerequisites
- Scripts provide detailed output
- Scripts can be run multiple times safely

## 🔧 Configuration

Scripts can be configured by modifying environment variables or script parameters:

- **Build targets**: Specify which crates to build
- **Test suites**: Configure which tests to run
- **Deployment targets**: Set deployment environments
- **Output directories**: Customize artifact locations

## 📊 Monitoring

Scripts provide comprehensive monitoring and reporting:

- **Build status**: Success/failure reporting
- **Test results**: Detailed test output
- **Deployment status**: Deployment progress tracking
- **Performance metrics**: Build and test performance

## 🚀 Best Practices

1. **Run scripts from project root**
2. **Check prerequisites before execution**
3. **Review output for any warnings**
4. **Use appropriate script for your needs**
5. **Keep scripts up to date with project changes**

---

*These maintenance scripts ensure consistent and reliable build, test, and deployment processes for the Radix-Leptos project.*
