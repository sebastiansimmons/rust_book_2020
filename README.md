## My Progress through the book _[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)_

[Rust](https://www.rust-lang.org/) has started to catch my ear as it shows up in articles and comes up in conversation.
 ["Why Discord is switching from Go to Rust"](https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f) was especially intriguing as Discord engineers found "Even with just basic optimization, Rust was able to outperform the hyper hand-tuned Go version." After a classmate told me about how every variable is immutable by default and explained the concept of ownership, I knew I had to check it out.

 ### Projects

 At the end of each chapter are some suggested projects to

 [Chapter 8: Common Collections](#chapter-8-common-collections)
 - [Pig Latin translator](./chapter_8/pig_latin/)

 [Chapter 12: An I/O Project: Building a Command Line Program](#chapter-12-an-io-project-building-a-command-line-program)
 - [MiniGrep](./chapter_12/minigrep/)

### Index/Chapter Notes
- [x] [Chapter 1: Getting Started](#chapter-1-getting-started)
- [x] [Chapter 2: Programming a Guessing Game](#chapter-2-programming-a-guessing-game)
- [x] [Chapter 3: Common Programming Concepts](#chapter-3-common-programming-concepts)
- [x] [Chapter 4: Understanding Ownership](#chapter-4-understanding-ownership)
- [x] [Chapter 5: Using Structs to Structure Related Data](#chapter-5-using-structs-to-structure-related-data)
- [x] [Chapter 6: Enums and Pattern Matching](#chapter-6-nums-and-pattern-matching)
- [x] [Chapter 7: Managing Growing Projects with Packages, Crates, and Modules](#chapter-7-managing-growing-projects-with-packages-crates-and-modules)
- [x] [Chapter 8: Common Collections](#chapter-8-common-collections)
- [x] [Chapter 9: Error Handling](#chapter-9-error-handling)
- [x] [Chapter 10: Generic Types, Traits, and Lifetimes](#chapter-10-generic-types-traits-and-lifetimes)
- [x] [Chapter 11: Writing Automated Tests](#chapter-11-writing-automated-tests)
- [x] [Chapter 12: An I/O Project: Building a Command Line Program](#chapter-12-an-io-project-building-a-command-line-program)
- [x] [Chapter 13: Functional Language Features: Iterators and Closures](#chapter-13-functional-language-features-iterators-and-closures)
- [x] [Chapter 14: More About Cargo and Crates.io](#chapter-14-more-about-cargo-and-cratesio)
- [x] [Chapter 15: Smart Pointers](#chapter-15-smart-pointers)
- [x] [Chapter 16: Fearless Concurrency](#chapter-16-fearless-concurrency)
- [x] [Chapter 17: Object-Oriented Programming Features of Rust](#chapter-17-object-oriented-programming-features-of-rust)
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

Algebraic data types and enums seem like a more graceful way of handling this type of struct/class pattern matching. It's one of those language quirks I'll have to use a little before I see how it clicks in my head.

Looking forward to putting this stuff into use once the book gets to a project.

## Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

Modules are pretty straightforward and look like they act exactly as expected. Cargo seems like a pretty good dependency manager. I'll have to build some projects before really knowing. I've browse through some crates and read some project write ups and it seems like most common needs can be met at this point in the languages life. From the little I've looked, there are enough crates out there to get projects kickstarted without having to reinvent the wheel at every turn. However, Rust is still relatively young as a language and has plenty of room for rapid expansion and meaningful contribution.

## Chapter 8: Common Collections

Ownership adds an extra element to think about with collections. Vectors seem fairly normal aside from the extra thinking required with ownership. Maybe as a quick basic project I will implement a few sorting/search algorithms to get the feel for manipulating collections.

The end of the strings chapter sums it up nicely, they are not that simple. Strings in Rust are a lot more complicated than they are in a language like Python and 99% of its use cases.

**Project:** [Pig Latin Translator](./chapter_8/pig_latin/)

## Chapter 9: Error Handling

I had heard that error handling is one of the places rust code can start to look a bit messy. 
I can definitely see how the results<> return type could lead to a lot of seemingly extra work in early stages of development. However, I think functionally it's a cool feature.

## Chapter 10: Generic Types, Traits, and Lifetimes

I really like traits. I think that it fixes the problems that come up with class based languages and inheritence. Lifetimes are also a great feature though it'll definitely take some getting used to actually writing code that uses them.

## Chapter 11: Writing Automated Tests

I appreciate that Rust has a built in testing framework. I think it makes sense. I have only used one other testing framework so I can't compare it too much.

## Chapter 12: An I/O Project: Building a Command Line Program

This was really and a good first little program to write in Rust. 

## Chapter 13: Functional Language Features: Iterators and Closures

Iterators are really powerful and seem worth learning more than what this chapter covers.

Closures are very useful and seem like they are easy to forget but powerful when used properly.

## Chapter 14: More About Cargo and Crates.io

I have already poked around crates.io a bit and enjoy Cargo. It seems like workspaces and Cargo provide the tools needed for managing projects as they start to scale up in complexity.

## Chapter 15: Smart Pointers

Rc<T> seems like a good solution for multiple ownership cases. Pointers and smart pointers seem to be where the concept of ownership, something I really like about Rust, introduces extra complications in the world of pointers and data structures that have inherent multiple reference points. The solutions of weak and strong ownership sharing seem like good solutions but I'll have to really write some code before getting a good feel for where it makes sense to use these concepts.

## Chapter 16: Fearless Concurrency

I'm excited for chapter 20 when this chapter really gets put in context. It seems like Rust has everything needed for safe concurrency. I was already somewhat familiar with mutexes so that wasn't too much of a conceptual challenge. I want to see how this all looks in practice.

## Chapter 17: Object-Oriented Programming Features of Rust
