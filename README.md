## My Progress through the book _[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)_

[Rust](https://www.rust-lang.org/) has started to catch my ear as it shows up in articles and comes up in conversation.
 ["Why Discord is switching from Go to Rust"](https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f) was especially intriguing as Discord engineers found "Even with just basic optimization, Rust was able to outperform the hyper hand-tuned Go version." After a classmate told me about how every variable is immutable by default and explained the concept of ownership, I knew I had to check it out.

### Index
- [x] [Chapter 1: Getting Started](#chapter-1-getting-started)
- [x] [Chapter 2: Programming a Guessing Game](#chapter-2-programming-a-guessing-game)
- [x] [Chapter 3: Common Programming Concepts](#chapter-3-common-programming-concepts)
- [x] [Chapter 4: Understanding Ownership](#chapter-4-understanding-ownership)
- [x] [Chapter 5: Using Structs to Structure Related Data](#chapter-5-using-structs-to-structure-related-data)
- [x] [Chapter 6: Enums and Pattern Matching](#chapter-6-nums-and-pattern-matching)
- [ ] Chapter 7: Managing Growing Projects with Packages, Crates, and Modules
- [ ] Chapter 8: Common Collections
- [ ] Chapter 9: Error Handling
- [ ] Chapter 10: Generic Types, Traits, and Lifetimes
- [ ] Chapter 11: Writing Automated Tests
- [ ] Chapter 12: An I/O Project: Building a Command Line Program
- [ ] Chapter 13: Functional Language Features: Iterators and Closures
- [ ] Chapter 14: More About Cargo and Crates.io
- [ ] Chapter 15: Smart Pointers
- [ ] Chapter 16: Fearless Concurrency
- [ ] Chapter 17: Object-Oriented Programming Features of Rust
- [ ] Chapter 18: Patterns and Matching
- [ ] Chapter 19: Advanced Features
- [ ] Chapter 20: Final Project: Building a Multithreaded Web Server

## Chapter 1: Getting Started
Installation was fairly straightforward. Cargo seems like a great package manager.

## Chapter 2: Programming a Guessing Game
I like that by default variables are immutable. That's one of the design choices that originally caught my eye and made me want to learn Rust.
I appreciate how Cargo handles crates and updating.

## Chapter 3: Common Programming Concepts
I mostly skimmed this chapter since it's written without the assumption that everyone reading has used compiled languages before.

## Chapter 4: Understanding Ownership

Ownership is a really elegant solution to all the memory issues you can run into in a language like C++. The ownership system is one of the main reasons I wanted to learn Rust and I can see how it informs programming in Rust.

## Chapter 5: Using Structs to Structure Related Data

Pretty straightforward. The Rust specific rules around structs for memory safety are all great. I especially appreciate [automatic referencing and dereferencing](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator).

## Chapter 6: Enums and Pattern Matching

My only experience with functional programming is using JavaScript in a functional way. Algebraic data types and enums seem like a more graceful way of handling this type of struct/class pattern matching. It's one of those language quirks I'll have to use a little before I see how it clicks in my head.

Looking forward to putting this stuff into use once the book gets to a project.
