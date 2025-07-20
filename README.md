# 🦀 My Rust Learning Journey

Welcome to my Rust learning journey! This repo documents my progress through a TBD-week self-study plan designed to take me from beginner to publishing real Rust projects.

---

## 📍 Progress Tracker

| Week         | Status         |
|--------------|----------------|
| ✅ Week 1     | Fundamentals   |
| ✅ Week 2/3     | Beginner Rust |
| 🔵 Week 4     | Midterm Project |
| ⬜ Week 5     | Web & Async    |
| ⬜ Week 6     | Web API Dev    |
| ⬜ Week 7     | Deployment     |

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
- 📘 [The Rust Book](https://doc.rust-lang.org/book/) – Chapters 1–6, 8.1
- 🧪 [Rustlings](https://github.com/rust-lang/rustlings) – Chapters 1-8, Quiz 1

**Tasks:**
- Set up this repo (`rust-learning`)
- Commit daily notes and solutions

---

### Week 2/3: Beginner Rust

**Topics:**
- Generic Types, Trait & Lifetimes
- Strings, HashMaps
- Error handling with `Result` and `Option`
- Automated Test

**Resources:**
- Rust Book Chapters 7,8.2,8.3,9–11
- Rustlings: Chapters 9-17, Quiz 2, Quiz 3

**Tasks:**
- Commit daily notes and solutions

---

### Week 4: Midterm Project

**Topics:**
- Iterators, Closures
- Cargo and Crates.io
- Smart Pointers

**Resources:**
- Rust Book Chapters 13-15
- Rustlings: Chapters 18-19

**Project:**
#### ✅ Mini Project 1: Todo CLI App
- Features: Add/remove/list tasks
- Store tasks in a JSON file
- Crates: `clap`, `serde`, `serde_json`

**Tasks:**
- Commit daily notes and solutions
- Start the project Todo CLI App
- Add tests and logging
- ✅ Push to GitHub: `todo-cli-rust`

---

### Week 5: Concurrency and Web Basics

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

### Week 6: Web API Development

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

### Week 7: Testing, Refactoring, Deployment

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
