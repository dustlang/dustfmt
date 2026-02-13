# Changelog - dustfmt (DPL Formatter)

All notable changes to dustfmt are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-02-12 (DPL v0.2)

### Added

- **DPL v0.2 Compliance**: Full support for v0.2 specification
- K Regime v0.2 syntax formatting support
- Variable declaration formatting (`let`, `mut let`)
- Control flow structure formatting (`if/else`, `for`, `while`)
- Match expression formatting (`match`, `_`, `=>`)
- Function definition formatting
- Structure definition formatting
- Memory operation formatting
- Float literal formatting (`3.14`)
- Char literal formatting (`'a'`)
- Range operator formatting (`..`)
- Configuration options for v0.2 features

### Changed

- Updated formatting rules for expanded syntax
- Improved indentation for nested structures
- Enhanced alignment for function parameters

### Fixed

- Formatting edge cases in new syntax
- Line width calculations for complex expressions
- Pattern matching bugs in formatter

## [0.1.0] - 2026-02-12

### Added

- Initial formatter implementation
- Basic K Regime syntax formatting
- Emit statement formatting
- Configuration file support

### Known Issues

- Limited formatting for v0.1 subset only

---

Copyright © 2026 Dust LLC