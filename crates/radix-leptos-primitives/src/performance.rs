//! Performance optimization utilities for Radix-Leptos components
//! 
//! This module provides performance-focused utilities including:
//! - String interning and caching
//! - Component memoization
//! - Memory pool management
//! - Performance monitoring

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use leptos::prelude::*;

/// String interning cache for frequently used strings
#[derive(Debug, Clone)]
pub struct StringCache {
    cache: Arc<Mutex<HashMap<String, String>>>,
    max_size: usize,
}

impl StringCache {
    /// Create a new string cache with the specified maximum size
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_size,
        }
    }

    /// Get or insert a string into the cache
    pub fn get_or_insert(&self, key: &str) -> String {
        let mut cache = self.cache.lock().unwrap();
        
        if let Some(cached) = cache.get(key) {
            return cached.clone();
        }

        // If cache is full, remove oldest entries
        if cache.len() >= self.max_size {
            let keys_to_remove: Vec<String> = cache.keys().take(self.max_size / 2).cloned().collect();
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }

        let value = key.to_string();
        cache.insert(key.to_string(), value.clone());
        value
    }

    /// Clear the cache
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let cache = self.cache.lock().unwrap();
        CacheStats {
            size: cache.len(),
            max_size: self.max_size,
            usage_percentage: (cache.len() as f64 / self.max_size as f64) * 100.0,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub max_size: usize,
    pub usage_percentage: f64,
}

/// Global string cache instance
static GLOBAL_STRING_CACHE: std::sync::OnceLock<StringCache> = std::sync::OnceLock::new();

/// Get the global string cache
pub fn get_global_string_cache() -> &'static StringCache {
    GLOBAL_STRING_CACHE.get_or_init(|| StringCache::new(1000))
}

/// Optimized string merging with caching
pub fn merge_classes_cached(classes: Vec<&str>) -> String {
    let cache = get_global_string_cache();
    let key = classes.join(" ");
    cache.get_or_insert(&key)
}

/// Performance monitor for tracking component render times
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    measurements: Arc<Mutex<Vec<Measurement>>>,
    max_measurements: usize,
}

#[derive(Debug, Clone)]
pub struct Measurement {
    pub name: String,
    pub duration: Duration,
    pub timestamp: Instant,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(max_measurements: usize) -> Self {
        Self {
            measurements: Arc::new(Mutex::new(Vec::new())),
            max_measurements,
        }
    }

    /// Record a measurement
    pub fn record(&self, name: String, duration: Duration) {
        let mut measurements = self.measurements.lock().unwrap();
        
        if measurements.len() >= self.max_measurements {
            measurements.drain(0..self.max_measurements / 2);
        }

        measurements.push(Measurement {
            name,
            duration,
            timestamp: Instant::now(),
        });
    }

    /// Get performance statistics
    pub fn get_stats(&self) -> PerformanceStats {
        let measurements = self.measurements.lock().unwrap();
        
        if measurements.is_empty() {
            return PerformanceStats::default();
        }

        let total_duration: Duration = measurements.iter().map(|m| m.duration).sum();
        let avg_duration = total_duration / measurements.len() as u32;
        
        let mut durations: Vec<Duration> = measurements.iter().map(|m| m.duration).collect();
        durations.sort();
        
        let median_duration = if durations.len() % 2 == 0 {
            let mid = durations.len() / 2;
            (durations[mid - 1] + durations[mid]) / 2
        } else {
            durations[durations.len() / 2]
        };

        PerformanceStats {
            total_measurements: measurements.len(),
            average_duration: avg_duration,
            median_duration,
            min_duration: durations[0],
            max_duration: durations[durations.len() - 1],
        }
    }

    /// Clear all measurements
    pub fn clear(&self) {
        let mut measurements = self.measurements.lock().unwrap();
        measurements.clear();
    }
}

/// Performance statistics
#[derive(Debug, Clone, Default)]
pub struct PerformanceStats {
    pub total_measurements: usize,
    pub average_duration: Duration,
    pub median_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
}

/// Global performance monitor
static GLOBAL_PERFORMANCE_MONITOR: std::sync::OnceLock<PerformanceMonitor> = std::sync::OnceLock::new();

/// Get the global performance monitor
pub fn get_global_performance_monitor() -> &'static PerformanceMonitor {
    GLOBAL_PERFORMANCE_MONITOR.get_or_init(|| PerformanceMonitor::new(1000))
}

/// Macro for measuring component render time
#[macro_export]
macro_rules! measure_render_time {
    ($name:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let duration = start.elapsed();
        $crate::performance::get_global_performance_monitor().record($name.to_string(), duration);
        result
    }};
}

/// Memory pool for efficient allocation of common types
#[derive(Debug)]
pub struct MemoryPool<T> {
    pool: Arc<Mutex<Vec<T>>>,
    max_size: usize,
}

impl<T> MemoryPool<T> {
    /// Create a new memory pool
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: Arc::new(Mutex::new(Vec::new())),
            max_size,
        }
    }

    /// Get an item from the pool or create a new one
    pub fn get<F>(&self, factory: F) -> T
    where
        F: FnOnce() -> T,
    {
        let mut pool = self.pool.lock().unwrap();
        pool.pop().unwrap_or_else(factory)
    }

    /// Return an item to the pool
    pub fn return_item(&self, item: T) {
        let mut pool = self.pool.lock().unwrap();
        if pool.len() < self.max_size {
            pool.push(item);
        }
    }

    /// Clear the pool
    pub fn clear(&self) {
        let mut pool = self.pool.lock().unwrap();
        pool.clear();
    }
}

/// Component memoization utility
pub struct MemoizedComponent<T, F> {
    cache: Arc<Mutex<HashMap<String, T>>>,
    factory: F,
    max_cache_size: usize,
}

impl<T, F> MemoizedComponent<T, F>
where
    T: Clone,
    F: FnMut() -> T,
{
    /// Create a new memoized component
    pub fn new(factory: F, max_cache_size: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            factory,
            max_cache_size,
        }
    }

    /// Get a memoized value
    pub fn get(&mut self, key: &str) -> T {
        let mut cache = self.cache.lock().unwrap();
        
        if let Some(cached) = cache.get(key) {
            return cached.clone();
        }

        // If cache is full, remove oldest entries
        if cache.len() >= self.max_cache_size {
            let keys_to_remove: Vec<String> = cache.keys().take(self.max_cache_size / 2).cloned().collect();
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }

        let value = (self.factory)();
        cache.insert(key.to_string(), value.clone());
        value
    }

    /// Clear the cache
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }
}

/// Performance-optimized CSS class merging
pub fn merge_classes_optimized(classes: &[&str]) -> String {
    if classes.is_empty() {
        return String::new();
    }

    if classes.len() == 1 {
        return classes[0].to_string();
    }

    // Use string cache for frequently used combinations
    let cache = get_global_string_cache();
    let key = classes.join(" ");
    cache.get_or_insert(&key)
}

/// Performance-optimized ID generation with caching
pub fn generate_id_cached(prefix: &str) -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    
    let cache = get_global_string_cache();
    let key = format!("{}-{}", prefix, id);
    cache.get_or_insert(&key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_cache() {
        let cache = StringCache::new(10);
        
        let result1 = cache.get_or_insert("test");
        let result2 = cache.get_or_insert("test");
        
        assert_eq!(result1, result2);
        assert_eq!(cache.stats().size, 1);
    }

    #[test]
    fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new(10);
        
        monitor.record("test".to_string(), Duration::from_millis(100));
        monitor.record("test".to_string(), Duration::from_millis(200));
        
        let stats = monitor.get_stats();
        assert_eq!(stats.total_measurements, 2);
        assert_eq!(stats.average_duration, Duration::from_millis(150));
    }

    #[test]
    fn test_memory_pool() {
        let pool = MemoryPool::new(5);
        
        let item = pool.get(|| "test".to_string());
        assert_eq!(item, "test");
        
        pool.return_item("test".to_string());
    }

    #[test]
    fn test_memoized_component() {
        let mut call_count = 0;
        let mut memoized = MemoizedComponent::new(move || {
            call_count += 1;
            "expensive_result".to_string()
        }, 10);
        
        let result1 = memoized.get("key1");
        let result2 = memoized.get("key1");
        
        assert_eq!(result1, result2);
        assert_eq!(call_count, 1); // Should only call factory once
    }

    #[test]
    fn test_merge_classes_optimized() {
        let classes = vec!["class1", "class2", "class3"];
        let result = merge_classes_optimized(&classes);
        assert_eq!(result, "class1 class2 class3");
        
        let empty: Vec<&str> = Vec::new();
        let result = merge_classes_optimized(&empty);
        assert_eq!(result, "");
    }

    #[test]
    fn test_generate_id_cached() {
        let id1 = generate_id_cached("test");
        let id2 = generate_id_cached("test");
        
        assert_ne!(id1, id2); // Should be different IDs
        assert!(id1.starts_with("test-"));
        assert!(id2.starts_with("test-"));
    }
}
