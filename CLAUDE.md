# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust terminal UI (TUI) application that provides an interactive interface for exploring Slurm `sacct` command output. The application uses `ratatui` for the TUI framework and `crossterm` for terminal handling.

## Architecture

- **main.rs**: Entry point with CLI argument parsing, terminal setup, panic handling, and main event loop
- **sacct.rs**: Handles execution of the `sacct` command and parsing of its pipe-delimited output into structured data
- **ui.rs**: Contains all UI rendering logic including job list display and detailed job information popup

## Key Components

- `App` struct: Main application state including job list, selection index, and quit flag
- `SacctData` struct: Represents a single job record with all relevant Slurm job information
- UI layout uses a three-section vertical layout: header, job list, and footer with help text
- `setup_panic_hook()`: Ensures graceful terminal restoration on application panic

## Dependencies

- `ratatui`: TUI framework (v0.28)
- `crossterm`: Cross-platform terminal manipulation
- `clap`: Command line argument parsing with derive feature
- `serde`: Serialization framework with derive feature
- `chrono`: Date and time handling
- `tokio`: Async runtime with full features

## Development Commands

- `cargo run`: Build and run the application
- `cargo build`: Build the project
- `cargo check`: Check compilation without building
- `cargo test`: Run tests (when implemented)
- `cargo run -- --sacct-args "--starttime=2024-01-01"`: Run with additional sacct arguments

## Usage

The application executes `sacct` with predefined formatting options and displays the results in a navigable list. Users can:
- Navigate with arrow keys or j/k
- Press 'q' to quit
- View detailed job information in a popup (future enhancement)

## Slurm Integration

The application calls `sacct` with specific formatting:
- `--format=JobID,JobName,Partition,Account,AllocCPUS,State,ExitCode,Start,End,Elapsed,Timelimit,Submit,User,WorkDir`
- `--parsable2`: Pipe-delimited output
- `--noheader`: Excludes column headers from output