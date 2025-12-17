# 在线评测系统 (Online Judge)

[![Rust](https://img.shields.io/badge/Language-Rust-black?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Actix Web](https://img.shields.io/badge/Web_Framework-Actix--Web-green?style=flat-square)](https://actix.rs/)
[![Tokio](https://img.shields.io/badge/Async_Runtime-Tokio-blue?style=flat-square)](https://tokio.rs/)
[![SQLite](https://img.shields.io/badge/Database-SQLite-lightgrey?style=flat-square&logo=sqlite)](https://www.sqlite.org/)

> 2024 年夏季学期《程序设计训练》 Rust 课堂大作业（二）。

## 目录

- [项目简介](#项目简介)
- [核心功能与技术实现](#核心功能与技术实现)
- [作业要求](#作业要求)
- [Honor Code](#honor-code)
- [自动测试](#自动测试)
- [English Version](#online-judge-system)

## 项目简介

本项目是一个基于 Rust 语言开发的高性能在线评测系统（OJ）。该系统采用了现代化的异步架构，旨在提供稳定、高效的代码评测服务。系统不仅支持多用户的并发提交与比赛管理，还实现了数据持久化、实时评测反馈以及灵活的排行榜机制，能够满足复杂的评测需求。

## 核心功能与技术实现

### 1. 高并发异步 Web 服务
- **功能描述**：系统提供了一套完整的 RESTful API，支持题目获取、代码提交、比赛创建、用户注册及状态查询等操作。
- **技术实现**：基于 **`actix-web`** 框架构建 HTTP 服务器，利用 Rust 强大的 **`tokio`** 异步运行时，实现了非阻塞的 I/O 处理。这使得服务器能够在单线程或少量线程下高效处理成千上万的并发请求，显著降低了资源消耗并提升了响应速度。

### 2. 异步评测与任务队列
- **功能描述**：用户提交的代码会被立即接收并进入后台队列，系统无需等待评测完成即可响应前端。评测过程（编译、运行、判题）在后台独立进行。
- **技术实现**：采用 **生产者-消费者模型**。Web 接口作为生产者，将任务通过 `tokio::sync::mpsc` 通道发送给后台评测器（消费者）。评测器使用 `tokio::process::Command` 异步调用外部编译器和运行环境，并通过轮询机制实时更新任务状态（Queuing -> Compiling -> Running -> Finished），实现了评测流程与 Web 服务的解耦。

### 3. 数据持久化存储
- **功能描述**：系统能够持久保存用户数据、比赛配置及历史提交记录，确保在服务重启或异常崩溃后数据不丢失。
- **技术实现**：集成 **`rusqlite`** 库管理 SQLite 数据库。系统启动时自动检测并初始化数据库表结构，运行过程中将关键状态变更实时写入磁盘数据库文件，保证了数据的一致性与可靠性。

### 4. 高级比赛管理与动态排行
- **功能描述**：支持创建包含多道题目的比赛，允许用户报名参与。系统能够根据配置的评分规则（如 ACM 赛制或得分制）和排名策略（如罚时、提交时间）实时生成并更新排行榜。
- **技术实现**：设计了灵活的数据结构来存储比赛上下文。通过内存中的状态管理（结合 `Mutex` 或 `RwLock` 保证并发安全）快速计算排名，同时支持 `ScoringRule` 和 `TieBreaker` 的动态策略模式，以适应不同类型的比赛需求。

### 5. 资源限制与安全隔离
- **功能描述**：为防止用户提交的恶意代码或无限循环程序耗尽服务器资源，系统对每个评测任务实施严格的时间和内存限制。
- **技术实现**：
    - **时间限制**：利用 `wait-timeout` 库或异步定时器监控子进程运行时间，一旦超时立即终止进程。
    - **内存限制**：通过 `libc` 库调用系统级 API（如 `setrlimit`）在子进程启动前设置资源上限（ulimit），确保评测环境的安全稳定。

### 6. 灵活的可配置性
- **功能描述**：系统支持通过配置文件自定义监听端口、数据库路径、各类语言的编译/运行命令等。
- **技术实现**：使用 **`serde_json`** 解析 JSON 配置文件，结合 **`clap`** 处理命令行参数，使得系统部署和环境适配变得非常灵活。

## 作业要求

具体要求请查看[作业文档](https://lab.cs.tsinghua.edu.cn/rust/projects/oj/)。

## Honor Code

请在 `HONOR-CODE.md` 中填入你完成作业时参考的内容，包括：

*   开源代码仓库（直接使用 `crate` 除外）
*   查阅的博客、教程、问答网站的网页链接
*   与同学进行的交流

## 自动测试

本作业的基础要求和部分提高要求可使用 Cargo 进行自动化测试。
- 运行基础测试：`cargo test --test basic_requirements -- --test-threads=1`
- 运行提高测试：`cargo test --test advanced_requirements -- --test-threads=1`

如果某个测试点运行失败，将会打印 `case [name] incorrect` 的提示（可能会有额外的 `timeout` 提示，可以忽略）。你可以使用 `cargo test test_name` 单独运行此测试，也可以在 `tests/cases` 目录下查看相应测试用例的内容，并按照文档的说明调试。

自动测试运行每个测试点后，会生成以下的文件：

*   `[case_name].stdout/stderr`：OJ 程序的标准输出和标准错误。可以在代码中添加打印语句，然后结合输出内容来调试代码。
*   `[case_name].http`：测试过程中发送的 HTTP 请求和收到的响应。调试时，可以先自己启动一个 OJ 服务端（`cargo run`），然后用 VSCode REST Client 来手动发送这些 HTTP 请求，并观察响应。

项目配置了持续集成（CI）用于帮助测试。在推送改动后，可以在 GitLab 网页上查看 CI 结果和日志。上述文件也会被收集到对应任务的 artifacts 中，可在 GitLab 网页上下载查看。

---

# Online Judge System

[![Rust](https://img.shields.io/badge/Language-Rust-black?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Actix Web](https://img.shields.io/badge/Web_Framework-Actix--Web-green?style=flat-square)](https://actix.rs/)
[![Tokio](https://img.shields.io/badge/Async_Runtime-Tokio-blue?style=flat-square)](https://tokio.rs/)
[![SQLite](https://img.shields.io/badge/Database-SQLite-lightgrey?style=flat-square&logo=sqlite)](https://www.sqlite.org/)

> 2024 Summer Term "Programming Practice Training" Rust Course Project (II).

## Table of Contents

- [Project Overview](#project-overview)
- [Core Features & Implementation](#core-features--implementation-details)
- [Assignment Requirements](#assignment-requirements)
- [Honor Code](#honor-code-1)
- [Automatic Testing](#automatic-testing)

## Project Overview

This project is a high-performance Online Judge (OJ) system developed in Rust. Built upon a modern asynchronous architecture, it provides stable and efficient code evaluation services. The system supports concurrent user submissions, contest management, data persistence, real-time feedback, and dynamic ranking, catering to complex judging requirements.

## Core Features & Implementation Details

### 1. High-Performance Async Web Service
- **Feature**: Provides a comprehensive RESTful API for problem retrieval, code submission, contest creation, user registration, and status querying.
- **Implementation**: Built with the **`actix-web`** framework and powered by the **`tokio`** asynchronous runtime. This non-blocking I/O architecture allows the server to efficiently handle thousands of concurrent requests with minimal resource consumption and high responsiveness.

### 2. Asynchronous Judging & Job Queue
- **Feature**: User submissions are immediately queued and processed in the background, allowing the system to respond to the frontend without waiting for the evaluation to finish. The compilation, execution, and judging processes run independently.
- **Implementation**: Utilizes a **Producer-Consumer model**. Web endpoints act as producers, sending jobs to the background judge (consumer) via `tokio::sync::mpsc` channels. The judge uses `tokio::process::Command` to asynchronously invoke external compilers and runtimes, updating the job status (Queuing -> Compiling -> Running -> Finished) in real-time via polling.

### 3. Data Persistence
- **Feature**: Ensures that user data, contest configurations, and submission history are preserved across server restarts or crashes.
- **Implementation**: Integrated with **`rusqlite`** for SQLite database management. The system initializes table structures on startup and writes critical state changes to a disk-based database file in real-time, ensuring data consistency and reliability.

### 4. Advanced Contest Management & Dynamic Ranking
- **Feature**: Supports creating contests with multiple problems and user registration. The system generates and updates ranklists in real-time based on configurable scoring rules (e.g., ACM style vs. Score style) and tie-breaking strategies.
- **Implementation**: Designed with flexible data structures for contest contexts. It uses in-memory state management (protected by `Mutex` or `RwLock` for thread safety) to rapidly calculate rankings, supporting dynamic strategies for `ScoringRule` and `TieBreaker`.

### 5. Resource Limits & Safety Sandbox
- **Feature**: Enforces strict time and memory limits on user submissions to prevent malicious code or infinite loops from exhausting server resources.
- **Implementation**:
    - **Time Limit**: Uses `wait-timeout` or async timers to monitor child processes, terminating them immediately if they exceed the time allowance.
    - **Memory Limit**: Utilizes `libc` to invoke system-level APIs (like `setrlimit`) to set resource limits (ulimit) before launching child processes, ensuring a secure and stable evaluation environment.

### 6. Flexible Configuration
- **Feature**: Allows customization of server ports, database paths, and compilation/execution commands for different languages via configuration files.
- **Implementation**: Uses **`serde_json`** to parse JSON config files and **`clap`** to handle command-line arguments, making deployment and environment adaptation highly flexible.

## Assignment Requirements

Please refer to the [Project Documentation](https://lab.cs.tsinghua.edu.cn/rust/projects/oj/) for specific requirements.

## Honor Code

Please list the references used during the completion of this assignment in `HONOR-CODE.md`, including:

*   Open source repositories (excluding directly used `crate`s)
*   Links to blogs, tutorials, Q&A websites consulted
*   Communications with classmates

## Automatic Testing

Basic requirements and some advanced requirements of this assignment can be tested automatically using Cargo.

- Run basic tests: `cargo test --test basic_requirements -- --test-threads=1`
- Run advanced tests: `cargo test --test advanced_requirements -- --test-threads=1`

If a test case fails, a hint `case [name] incorrect` will be printed (you may ignore extra `timeout` hints). You can run a specific test using `cargo test test_name`, or check the corresponding test cases in the `tests/cases` directory and debug according to the documentation.

After running each test point, the following files will be generated:

*   `[case_name].stdout/stderr`: Standard output and error of the OJ program. You can add print statements in your code and debug using these outputs.
*   `[case_name].http`: HTTP requests sent and responses received during testing. For debugging, you can start the OJ server (`cargo run`) and use VSCode REST Client to manually send these requests.

The project is configured with Continuous Integration (CI) to help you test. After pushing your changes, you can view CI results and logs on the GitLab webpage. The aforementioned files will also be collected in the corresponding job artifacts for download.
