# Rust Article Series Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a structured Rust learning article project with 10 beginner-friendly Markdown articles and 10 deeper research articles.

**Architecture:** Use a lightweight documentation-only repository. Keep beginner articles under `docs/rust-basics/` and deep-dive articles under `docs/rust-deep-dive/`, both with numbered filenames so the reading path is stable. Keep `README.md` as the top-level index for both series.

**Tech Stack:** Markdown, Rust code examples, Cargo command examples.

---

### Task 1: Create Project Index

**Files:**
- Modify: `README.md`

**Steps:**
1. Add project purpose.
2. Link all 10 basic articles in reading order.
3. Add 10 future deep Rust article topics.
4. Add reading guidance.

### Task 2: Create Basic Articles

**Files:**
- Create: `docs/rust-basics/01-getting-started.md`
- Create: `docs/rust-basics/02-variables-and-types.md`
- Create: `docs/rust-basics/03-ownership-move-copy.md`
- Create: `docs/rust-basics/04-borrowing-references-slices.md`
- Create: `docs/rust-basics/05-structs-enums-pattern-matching.md`
- Create: `docs/rust-basics/06-functions-control-flow-modules.md`
- Create: `docs/rust-basics/07-error-handling.md`
- Create: `docs/rust-basics/08-collections-strings-iterators.md`
- Create: `docs/rust-basics/09-generics-traits-lifetimes.md`
- Create: `docs/rust-basics/10-cargo-testing-workflow.md`

**Steps:**
1. Give each article a focused title and learning goals.
2. Explain the concept in practical language.
3. Include small Rust examples.
4. Add common pitfalls and exercises.

### Task 3: Verify

**Files:**
- All Markdown files.

**Steps:**
1. Check that all expected files exist.
2. Check article count.
3. Inspect generated Markdown headings and links.

### Task 4: Create Deep-Dive Articles

**Files:**
- Create: `docs/rust-deep-dive/01-ownership-compile-time-reasoning.md`
- Create: `docs/rust-deep-dive/02-borrow-checker-and-nll.md`
- Create: `docs/rust-deep-dive/03-traits-dynamic-dispatch-object-safety.md`
- Create: `docs/rust-deep-dive/04-generics-monomorphization-code-size.md`
- Create: `docs/rust-deep-dive/05-memory-layout-drop-unsafe-boundaries.md`
- Create: `docs/rust-deep-dive/06-async-await-future-executor.md`
- Create: `docs/rust-deep-dive/07-send-sync-concurrency-safety.md`
- Create: `docs/rust-deep-dive/08-macros-declarative-procedural-codegen.md`
- Create: `docs/rust-deep-dive/09-ffi-abi-cross-language-boundaries.md`
- Create: `docs/rust-deep-dive/10-rust-compiler-mir-llvm-pipeline.md`

**Steps:**
1. Use the same numbered structure as the basic series.
2. Focus each article on language mechanisms and engineering tradeoffs.
3. Include code examples, common misunderstandings, and follow-up research directions.
4. Update `README.md` so every deep-dive article is linked from the project index.
