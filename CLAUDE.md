# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

**Frontend (React + Vite + TypeScript):**
- `yarn dev` - Start development server
- `yarn build` - Build the frontend application (runs TypeScript compilation + Vite build)
- `yarn preview` - Preview the built application

**Backend (Tauri + Rust):**
- `yarn tauri dev` - Start the Tauri development environment (runs both frontend and backend)
- `yarn tauri build` - Build the complete Tauri application
- `cargo build` - Build only the Rust backend (from src-tauri directory)
- `cargo run` - Run the Rust backend directly (from src-tauri directory)

**Testing:**
- No specific test commands are configured. Check if tests exist before assuming test framework.

## Architecture Overview

This is a **Japanese learning application** built with:
- **Frontend**: React 19 + TypeScript + Vite + TailwindCSS
- **Backend**: Tauri v2 (Rust) with type-safe bindings
- **Data**: Japanese dictionaries (JMDict, Kanjidic2, Heisig) and local storage

### Key Components

**Frontend Structure (src/):**
- `App.tsx` - Main app with tab navigation (Words, Kanji, Parsed Words)
- `components/` - React components for different features:
  - `Words.tsx`, `WordCard.tsx`, `WordAdder.tsx` - Word management
  - `Kanjis.tsx`, `KanjiAdder.tsx`, `KanjiSelector.tsx` - Kanji management
  - `ParsedWords.tsx`, `AnnotatedText.tsx` - Text parsing with furigana
  - `Autocomplete.tsx`, `Dialog.tsx`, `Tabs.tsx` - UI components
- `bindings.ts` - Auto-generated TypeScript bindings for Rust commands
- `queries/index.ts` - React Query setup for data fetching

**Backend Structure (src-tauri/src/):**
- `lib.rs` - Main Tauri setup with command registration and module organization
- `kanji/` - Kanji-related commands and parsing (Heisig, Kanjidic2, furigana)
- `word/` - Word dictionary lookup and translation services
- `knowledge_base/` - Local storage for user's learned words and kanji
- `data/` - Data loading and management for dictionaries
- `translation/` - Translation services setup
- `conversion/` - Data conversion utilities

### Type Safety & Communication

The app uses **tauri-specta** for type-safe communication between frontend and backend:
- Rust commands are automatically exported to TypeScript bindings
- All data types are shared between Rust and TypeScript
- Commands return `Result<T, E>` types for error handling

### Data Sources

- **JMDict**: Japanese-English dictionary for word translations
- **Kanjidic2**: Kanji character information and readings
- **Heisig**: Kanji learning system with mnemonics and primitives
- **Local Store**: User's personal vocabulary using Tauri's plugin-store

### Key Libraries

**Rust dependencies:**
- `lindera` - Japanese text analysis and tokenization
- `jmdict` - Japanese dictionary access
- `furigana_parser` - Custom furigana generation
- `wana_kana` - Hiragana/Katakana/Romaji conversion
- `tauri-plugin-store` - Local data persistence

**Frontend dependencies:**
- `@tanstack/react-query` - Data fetching and caching
- `wanakana` - Japanese text conversion utilities
- `fuse.js` - Fuzzy search functionality
- `react-window` + `react-virtualized-auto-sizer` - Virtual scrolling for large lists

## Development Notes

- TypeScript bindings are auto-generated on debug builds to `src/bindings.ts`
- The app supports both vocabulary learning (words) and kanji study with mnemonics
- Text parsing generates furigana (pronunciation guides) for Japanese text
- Local storage persists user's learning progress across sessions