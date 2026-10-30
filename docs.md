# API notes

See the README for the supported exports and build steps. The streaming checker is a bracket and quote tracker, not a complete JSON validator. Create a new validator for each stream. WASM callers should release validator instances with `free()` when finished.
