#!/usr/bin/env python3
"""
Benchmark script for shiko-prompt
Measures performance by calling the binary 1 million times
"""

import subprocess
import time
import sys
import os
from pathlib import Path

def benchmark_shiko_binary(binary_path, iterations=1000):
    """
    Benchmark the shiko binary by calling it multiple times
    
    Args:
        binary_path: Path to the shiko binary
        iterations: Number of times to call the binary
    """
    print(f"🚀 Benchmarking shiko-prompt")
    print(f"📊 Binary: {binary_path}")
    print(f"🔢 Iterations: {iterations:,}")
    print("-" * 50)
    
    # Validate binary exists
    if not Path(binary_path).exists():
        print(f"❌ Error: Binary not found at {binary_path}")
        print("💡 Make sure to build the binary first with: cargo build --release")
        return False
    
    # Warm up (run a few times to cache any initialization)
    print("🔥 Warming up...")
    for i in range(10):
        try:
            subprocess.run([binary_path, "--left"], capture_output=True, timeout=1)
        except subprocess.TimeoutExpired:
            print("❌ Error: Binary taking too long to respond")
            return False
        except Exception as e:
            print(f"❌ Error during warmup: {e}")
            return False
    
    # Actual benchmark
    print("🏃 Starting benchmark...")
    start_time = time.time()
    
    for i in range(iterations):
        try:
            # Capture both stdout and stderr to avoid buffering issues
            result = subprocess.run(
                [binary_path, "--left"], 
                capture_output=True, 
                timeout=2,  # Timeout to prevent hanging
                text=True
            )
            
            # Check for errors
            if result.returncode != 0:
                print(f"❌ Error in iteration {i}: {result.stderr}")
                return False
                
        except subprocess.TimeoutExpired:
            print(f"❌ Timeout in iteration {i}")
            return False
        except Exception as e:
            print(f"❌ Error in iteration {i}: {e}")
            return False
        
        # Progress indicator
        if (i + 1) % 100000 == 0:
            elapsed = time.time() - start_time
            avg_ms = (elapsed * 1000) / (i + 1)
            print(f"📈 Progress: {i + 1:,}/{iterations:,} | Avg: {avg_ms:.3f}ms")
    
    end_time = time.time()
    total_time = end_time - start_time
    
    # Calculate results
    avg_time_ms = (total_time * 1000) / iterations
    calls_per_second = iterations / total_time
    
    # Display results
    print("-" * 50)
    print("🎯 Benchmark Results:")
    print(f"⏱️  Total time: {total_time:.3f} seconds")
    print(f"📈 Average time per call: {avg_time_ms:.3f}ms")
    print(f"🚀 Calls per second: {calls_per_second:,.0f}")
    print(f"📊 Calls per millisecond: {1000/avg_time_ms:,.1f}")
    
    # Performance assessment
    if avg_time_ms < 0.1:
        print("🎉 Excellent! < 0.1ms per call")
    elif avg_time_ms < 0.5:
        print("✅ Good! < 0.5ms per call")
    elif avg_time_ms < 1.0:
        print("👍 Acceptable! < 1.0ms per call")
    else:
        print("⚠️  Slow! > 1.0ms per call - consider optimizations")
    
    return True

def main():
    # Find the binary path
    script_dir = Path(__file__).parent
    binary_path = script_dir / "target" / "release" / "shiko"
    
    # Check if we need to build
    if not binary_path.exists():
        print("❌ Binary not found. Building first...")
        print("💡 Running: cargo build --release")
        try:
            subprocess.run(["cargo", "build", "--release"], check=True)
            print("✅ Build complete!")
        except subprocess.CalledProcessError:
            print("❌ Build failed. Please fix compilation errors first.")
            return 1
    
    # Run benchmark
    if not benchmark_shiko_binary(str(binary_path)):
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
