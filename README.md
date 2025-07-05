# 🦀 My Rust Learning Journey

Welcome to my Rust learning journey! This repo documents my progress through a 6-week self-study plan designed to take me from beginner to publishing real Rust projects.

---

## 📍 Progress Tracker

| Week         | Status         |
|--------------|----------------|
| 🔵 Week 1     | Fundamentals   |
| ⬜ Week 2     | Common Collections, Lifetimes and Error Handling |
| ⬜ Week 3     | Polish CLI     |
| ⬜ Week 4     | Web & Async    |
| ⬜ Week 5     | Web API Dev    |
| ⬜ Week 6     | Deployment     |

---

## 📚 Study Plan Overview

### ✅ Goal
By the end of this plan, I aim to:
- Understand core Rust concepts
- Build and publish two complete projects:
  1. A **Todo CLI**
  2. A **Pastebin Web API**
- Learn how to test, document, and deploy Rust code

---

## 📆 Weekly Breakdown

### Week 1: Fundamentals of Rust

**Topics:**
- Ownership, Borrowing
- Variables, Functions, Control Flow
- Structs, Enums, Pattern Matching
- Vectors

**Resources:**
- 📘 [The Rust Book](https://doc.rust-lang.org/book/) – Chapters 1–6, 8.1, 19.1
- 🧪 [Rustlings](https://github.com/rust-lang/rustlings) – Chapters 1,2,3,4,5,6,7,8 and Quiz 1

**Tasks:**
- Set up this repo (`rust-learning`)
- Commit daily notes and solutions

---

### Week 2: Common Collections, Lifetimes & Error Handling

**Topics:**
- Trait & Lifetimes
- Strings, HashMaps
- Error handling with `Result` and `Option`
- Automated Test

**Resources:**
- Rust Book Chapters 7–11
- Rustlings: Chapters 9-17

**Tasks:**
- Commit daily notes and solutions
- Mini Project 1

**Project:**
#### ✅ Mini Project 1: Todo CLI App
- Features: Add/remove/list tasks
- Store tasks in a JSON file
- Crates: `clap`, `serde`, `serde_json`

---

### Week 3: Ownership in Practice + Project Polish

**Topics:**
- Iterators, Closures
- File handling, Error Propagation
- Modularization and Crates

**Resources:**
- Rust Book Chapters 11–13
- Rust by Example

**Tasks:**
- Commit daily notes and solutions
- Polish and document the Todo CLI
- Add tests and logging
- ✅ Push to GitHub: `todo-cli-rust`

---

### Week 4: Concurrency and Web Basics

**Topics:**
- Threads, `Mutex`, `Arc`, Channels
- Closures and async programming
- HTTP basics and REST APIs

**Resources:**
- Rust Book Chapters 14–16
- [Zero to Production in Rust](https://www.zero2prod.com/) – Ch. 1–4

**Tasks:**
- Commit daily notes and solutions
- Choose a web API project:
  - Pastebin clone
  - Bookmark manager
  - URL shortener

---

### Week 5: Web API Development

**Tech Stack:**
- Framework: `actix-web` or `axum`
- DB: `sqlx` or `diesel`
- Serialization: `serde`

**Project:**
#### ✅ Mini Project 2: Pastebin Web API
- Endpoints: `/new`, `/get/{id}`
- DB: Store pastes with timestamps
- Dockerize & test the app

---

### Week 6: Testing, Refactoring, Deployment

**Topics:**
- Unit and integration testing
- CI/CD with GitHub Actions
- Docs and logging
- Deploying to Fly.io or Render

**Tasks:**
- Polish and document the web API
- Add `README` with usage examples
- ✅ Push to GitHub: `rust-pastebin-api`
- ✅ Deploy live (optional)

---

## 🏁 Final Outcome

| Project               | Description                    | Link                |
|-----------------------|--------------------------------|---------------------|
| `todo-cli-rust`       | Command-line todo manager      | 🚧 In Progress      |
| `rust-pastebin-api`   | Pastebin-like web API backend  | 🚧 In Progress      |

---

## 🔖 Bonus Ideas (Optional)
- Build a web scraper
- Contribute to open source: e.g., [`regex`](https://github.com/rust-lang/regex)
- Create a small GUI app using `egui` or `iced`

---

## 🚀 Let’s Go!
Feel free to fork this plan or reach out with improvements!
