# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-10-20

### Added
- Initial release of HTML Minifier with Rust backend
- High-performance HTML minification using FFI
- Support for minifying inline CSS and JavaScript
- Configurable minification options with presets (default, conservative, minimal)
- Multi-platform support (macOS ARM64/x86_64, Linux ARM64/x86_64)
- Comprehensive test suite (92 tests, 293 assertions)
- CLI tool for testing and benchmarking
- UTF-8 validation and proper encoding handling
- Input size limits and validation
- Detailed error messages and exception handling
- Version compatibility checking between PHP wrapper and Rust library
- Production-ready compiled libraries for multiple architectures

### Features
- Tokenized HTML parsing (not regex-based)
- Context-aware processing for `<pre>`, `<script>`, `<style>` tags
- Remove comments (with IE conditional comment preservation option)
- Collapse whitespace
- Remove optional closing tags
- Remove unnecessary attribute quotes
- Collapse boolean attributes
- Remove default attributes
- Remove empty attributes
- Minify inline JavaScript
- Minify inline CSS

### Performance
- 1-5ms processing time for typical web pages
- 50-60% average size reduction
- Linear memory scaling
- Optimized with Rust's zero-cost abstractions

### Documentation
- Comprehensive README with usage examples
- PHPDoc comments on all public methods
- Server configuration examples (Nginx, Apache)
- Installation and setup instructions
- CLI tool documentation

[1.0.0]: https://github.com/wexowgt/minifier/releases/tag/v1.0.0
