---
name: system-health
description: Check disk and memory usage for AEC field workstations.
steps:
  - id: disk
    run: df -h / | tail -n 1
  - id: mem
    run: free -h | grep Mem
---
# System Health Check
This workflow provides a quick diagnostic for Intel i7-4600U field workstations running Debian.
