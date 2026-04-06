import subprocess
import time
import os

def measure():
    start = time.time()
    # Use a dummy settings.toml if needed, but here we just test help first
    p = subprocess.Popen(["./lobster-rust/target/release/lobster-rust", "--help"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    p.wait()
    end = time.time()
    print(f"Startup time: {(end - start) * 1000:.2f} ms")

measure()
