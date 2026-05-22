# Rust Article Series Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a structured Rust learning article project with 10 beginner-friendly Markdown articles now and a roadmap for 10 deeper research articles later.

**Architecture:** Use a lightweight documentation-only repository. Keep beginner articles under `docs/rust-basics/` with numbered filenames so the reading path is stable, and keep future deep topics listed in `README.md` until they are written.

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
