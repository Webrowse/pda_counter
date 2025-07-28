# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), with simplified entries suitable for individual or small-team open-source projects.

---

## [0.1.0] - 2025-07-28

### Added
- Initial Solana Anchor smart contract `pda_counter` with three core instructions:
  - `initialize` - Creates a new PDA account with count set to 0.
  - `increment` - Increases the count stored in the PDA.
  - `reset` - Closes the PDA account and sends rent back to the user.
- JavaScript-based Anchor tests to verify:
  - PDA derivation
  - Initialization
  - Incrementing
  - Reset and account closure

### Changed
- PDA seed changed from `"counter"` to `"counte"` to avoid reinitialization issues during development.

### Misc
- Project formatted using `cargo fmt`.
- Added basic README with setup, test instructions, and project structure.
- Screenshot of test results added to README under test section.
