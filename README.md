# JSON Parser Challenge – Java & Rust
*NOTE: THis README was generate using chatgpt but the code here is mine. I don't really care about what is contained in this draft. I took on this challenge to practice after reading the book crafting interpreters*

This repository contains my solutions to the **JSON Parser Coding Challenge** from codingchallenges.fyi, implemented in **Java** and **Rust**.

The goal of the challenge is to build a JSON parser from scratch (no external JSON libraries) that can correctly parse valid JSON input and reject invalid input, following the JSON specification.

---

## 🧩 Challenge Overview

The challenge can be found here:

👉 [https://codingchallenges.fyi/challenges/challenge-json-parser/](https://codingchallenges.fyi/challenges/challenge-json-parser/)

Key requirements:

* Parse JSON objects, arrays, strings, numbers, booleans, and null
* Handle nested structures
* Detect and report invalid JSON
* Do **not** rely on existing JSON parsing libraries

---

## 📁 Repository Structure

```
.
├── java/        # Java implementation
├── rust/        # Rust implementation
└── README.md
```

Each implementation is self-contained and follows idiomatic practices for the respective language.

---

## ☕ Java Implementation

**Highlights:**

* Object-oriented design
* Recursive descent parsing
* Explicit tokenization and parsing stages
* Clear separation between lexer and parser

### Build & Run

```bash
cd java
javac Main.java
java Main < input.json
```

(Adjust commands if using Maven/Gradle.)

---

## 🦀 Rust Implementation

**Highlights:**

* Strongly typed AST using enums
* Pattern matching for parsing logic *lmao, no pattern matching used* 
* Ownership- and lifetime-safe design *avoided ownership problems at all points, clone everything*
* Zero external dependencies 

### Build & Run

```bash
cd rust
cargo run -- path/to/input.json
```

---

## ✅ Supported JSON Features

* Objects `{}`
* Arrays `[]`
* Strings (with escape handling)
* Numbers (integers & floating point)
* Booleans `true` / `false`
* `null`
* Arbitrary nesting

---

## ❌ Out of Scope / Limitations

* No streaming support
* Error messages focus on correctness rather than detailed diagnostics
* Performance optimizations were not the primary goal

---

## 🎯 Motivation

This project was built as a learning exercise to:

* Better understand parsing techniques
* Compare language design trade-offs between Java and Rust
* Practice low-level string processing and AST construction

---

## 📌 Notes

* The Java and Rust implementations are **independent**, not ports of each other.
* Design choices may differ to better fit each language’s strengths.

---

## 📄 License

This project is licensed under the MIT License. See `LICENSE` for details.

---

Feel free to explore, compare, and suggest improvements!
