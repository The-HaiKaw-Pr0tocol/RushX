# RushX

### Overview

RushX (Rust Shell - eXtended) is a POSIX-compliant Linux <ins>**terminal emulator**</ins> and <ins>**shell**</ins> implemented in Rust. It ships as a single binary: A GTK4-based terminal emulator that allocates a pseudoterminal (PTY) and renders output via Cairo, and calls by default an interactive POSIX-style shell that performs tokenization, redirection parsing, builtin dispatch, and `fork(2)`/`execvp(3)` execution of external commands.

RushX interfaces directly with the Linux kernel for process creation, session management, and controlling terminal assignment. No external libraries handle PTY allocation, signal delivery, or process lifecycle. The ***nix*** crate provides safe Rust wrappers around `openpty(3)`, `fork(2)`, `setsid(2)`, `dup2(2)`, `execvp(3)`, and `waitpid(2)`, while raw ***libc::ioctl*** is used for `TIOCSCTTY` where no safe wrapper exists.

> Developed & Maintained by [The HaiKaw Pr0tocol](https://github.com/The-HaiKaw-Pr0tocol) organization.

## RushX's Logo

<div align="center">
    <img alt="RushX's Logo" src="https://github.com/user-attachments/assets/4886a376-7669-49ec-965e-8b08d24b8dfd" width="1000"/>
</div>

## Authors

<div align="center">

<table>
  <tr>
    <td align="center">
        <img src="https://github.com/user-attachments/assets/18ff4153-f665-4426-b6ad-ad9717a08e1d" width="240px;" alt="Kawtar Taik"/><br />
        <b>Kawtar Taik</b> <br />
            <img src="https://github.com/user-attachments/assets/cd6bd36c-907c-49d2-a81b-5462c2e4142a" width="20" height="20" />
            <span>
                <a href="https://github.com/kei077"><img src="https://img.shields.io/badge/@kei077-8A2BE2?style=plastic"/></a>
            </span>
            <br /> <br />
            <a href="https://github.com/kei077" title="GitHub">
                <img src="https://img.shields.io/badge/-4B006E?style=flat&logo=github&logoColor=white" />
            </a>
            <a href="https://www.linkedin.com/in/kawtar-ta%C3%AFk-7544a11b9/" 
            title="LinkedIn">
                <img src="https://img.shields.io/badge/-4B006E.svg?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHNoYXBlLXJlbmRlcmluZz0iZ2VvbWV0cmljUHJlY2lzaW9uIiB0ZXh0LXJlbmRlcmluZz0iZ2VvbWV0cmljUHJlY2lzaW9uIiBpbWFnZS1yZW5kZXJpbmc9Im9wdGltaXplUXVhbGl0eSIgZmlsbC1ydWxlPSJldmVub2RkIiBjbGlwLXJ1bGU9ImV2ZW5vZGQiIHZpZXdCb3g9IjAgMCA1MTIgNTEyIj48cGF0aCBmaWxsPSIjZmZmIiBkPSJNNDc0LjkxOSAwSDM4LjU5MkMxNy43MiAwIDAgMTYuNTA0IDAgMzYuODQxVjQ3NS4xNEMwIDQ5NS40OTYgMTEuNjI5IDUxMiAzMi40OTIgNTEyaDQzNi4zMjdDNDg5LjcxOCA1MTIgNTEyIDQ5NS40OTYgNTEyIDQ3NS4xNFYzNi44NDFDNTEyIDE2LjUwNCA0OTUuODA5IDAgNDc0LjkxOSAwek0xOTUuMDQzIDE5NS4wNDNoNjguOTI4djM1LjEzNmguNzU1YzEwLjUwNS0xOC45NDUgNDEuNTQxLTM4LjE3NyA3OS45MjEtMzguMTc3IDczLjY1NSAwIDk0LjIxNCAzOS4xMDggOTQuMjE0IDExMS41Mzh2MTM1LjMyMWgtNzMuMTQ4VjMxNi44ODNjMC0zMi40MjctMTIuOTQ3LTYwLjg4My00My4yMjctNjAuODgzLTM2Ljc2OCAwLTU0LjI5NSAyNC44ODktNTQuMjk1IDY1Ljc1OHYxMTcuMTAzaC03My4xNDhWMTk1LjA0M3pNNzMuMTM5IDQzOC44NjFoNzMuMTQ4VjE5NS4wNDNINzMuMTM5djI0My44MTh6bTgyLjI4OS0zMjkuMTQ4YzAgMjUuMjU4LTIwLjQ1NyA0NS43MTUtNDUuNzE1IDQ1LjcxNS0yNS4yNTggMC00NS43MTUtMjAuNDU3LTQ1LjcxNS00NS43MTUgMC0yNS4yNTggMjAuNDU3LTQ1LjcxNSA0NS43MTUtNDUuNzE1IDI1LjI1OCAwIDQ1LjcxNSAyMC40NTcgNDUuNzE1IDQ1LjcxNXoiLz48L3N2Zz4=" />
            </a>
            <a href="mailto:kawtartaik123@gmail.com" 
            title="Email">
                <img src="https://img.shields.io/badge/-4B006E.svg?logo=data:image/svg+xml;base64,//48AHMAdgBnACAAeABtAGwAbgBzAD0AIgBoAHQAdABwADoALwAvAHcAdwB3AC4AdwAzAC4AbwByAGcALwAyADAAMAAwAC8AcwB2AGcAIgAgAHMAaABhAHAAZQAtAHIAZQBuAGQAZQByAGkAbgBnAD0AIgBnAGUAbwBtAGUAdAByAGkAYwBQAHIAZQBjAGkAcwBpAG8AbgAiACAAdABlAHgAdAAtAHIAZQBuAGQAZQByAGkAbgBnAD0AIgBnAGUAbwBtAGUAdAByAGkAYwBQAHIAZQBjAGkAcwBpAG8AbgAiACAAaQBtAGEAZwBlAC0AcgBlAG4AZABlAHIAaQBuAGcAPQAiAG8AcAB0AGkAbQBpAHoAZQBRAHUAYQBsAGkAdAB5ACIAIABmAGkAbABsAC0AcgB1AGwAZQA9ACIAZQB2AGUAbgBvAGQAZAAiACAAYwBsAGkAcAAtAHIAdQBsAGUAPQAiAGUAdgBlAG4AbwBkAGQAIgAgAHYAaQBlAHcAQgBvAHgAPQAiADAAIAAwACAANQAxADIAIAAzADIANwAuADUAMwAiAD4APABwAGEAdABoACAAZgBpAGwAbAA9ACIAIwBmAGYAZgAiACAAZAA9ACIATQAyADUANAAuADQAMQAgADIANwA0AC4AOQA3AGwAMQAwADAALgA5ADMALQAxADAAMAAuADkAMgAgADEANQAzAC4ANAA5ACAAMQA1ADMALgA0ADgASAAwAGwAMQA1ADMALgA0ADkALQAxADUAMwAuADQAOAAgADEAMAAwAC4AOQAyACAAMQAwADAALgA5ADIAegBNADUALgA4ADQAIAAwAGwAMgA0ADgALgA1ADcAIAAyADQAOAAuADUANgBMADUAMAAyAC4AOQA4ACAAMABIADUALgA4ADQAegBNADAAIAAyADkAMAAuADMAbAAxADMAMQAuADcANwAtADEAMwAxAC4ANwA4AEwAMAAgADIANgAuADcANQBWADIAOQAwAC4AMwB6AG0ANQAxADIAIAAxADIALgA2ADEATAAzADYANwAuADYAMQAgADEANQA4AC4ANQAyACAANQAxADIAIAAxADQALgAxADQAdgAyADgAOAAuADcANwB6ACIALwA+ADwALwBzAHYAZwA+AA==" />
            </a>
      <br />
    </td>
    <td align="center">
        <img src="https://github.com/user-attachments/assets/fb730dfb-b650-47f1-9810-e993a2e6f88d" width="240px;" alt="Haitam Bidiouane"/><br />
        <b>Haitam Bidiouane</b> <br />
            <img src="https://github.com/user-attachments/assets/cd6bd36c-907c-49d2-a81b-5462c2e4142a" width="20" height="20" />
            <span>
                <a href="https://github.com/sch0penheimer"><img src="https://img.shields.io/badge/@sch0penheimer-8A2BE2?style=plastic"/></a>
            </span>
            <br /> <br />
            <a href="https://github.com/sch0penheimer" title="GitHub">
                <img src="https://img.shields.io/badge/-4B006E?style=flat&logo=github&logoColor=white" />
            </a>
            <a href="https://www.linkedin.com/in/haitam-bidiouane" 
            title="LinkedIn">
                <img src="https://img.shields.io/badge/-4B006E.svg?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHNoYXBlLXJlbmRlcmluZz0iZ2VvbWV0cmljUHJlY2lzaW9uIiB0ZXh0LXJlbmRlcmluZz0iZ2VvbWV0cmljUHJlY2lzaW9uIiBpbWFnZS1yZW5kZXJpbmc9Im9wdGltaXplUXVhbGl0eSIgZmlsbC1ydWxlPSJldmVub2RkIiBjbGlwLXJ1bGU9ImV2ZW5vZGQiIHZpZXdCb3g9IjAgMCA1MTIgNTEyIj48cGF0aCBmaWxsPSIjZmZmIiBkPSJNNDc0LjkxOSAwSDM4LjU5MkMxNy43MiAwIDAgMTYuNTA0IDAgMzYuODQxVjQ3NS4xNEMwIDQ5NS40OTYgMTEuNjI5IDUxMiAzMi40OTIgNTEyaDQzNi4zMjdDNDg5LjcxOCA1MTIgNTEyIDQ5NS40OTYgNTEyIDQ3NS4xNFYzNi44NDFDNTEyIDE2LjUwNCA0OTUuODA5IDAgNDc0LjkxOSAwek0xOTUuMDQzIDE5NS4wNDNoNjguOTI4djM1LjEzNmguNzU1YzEwLjUwNS0xOC45NDUgNDEuNTQxLTM4LjE3NyA3OS45MjEtMzguMTc3IDczLjY1NSAwIDk0LjIxNCAzOS4xMDggOTQuMjE0IDExMS41Mzh2MTM1LjMyMWgtNzMuMTQ4VjMxNi44ODNjMC0zMi40MjctMTIuOTQ3LTYwLjg4My00My4yMjctNjAuODgzLTM2Ljc2OCAwLTU0LjI5NSAyNC44ODktNTQuMjk1IDY1Ljc1OHYxMTcuMTAzaC03My4xNDhWMTk1LjA0M3pNNzMuMTM5IDQzOC44NjFoNzMuMTQ4VjE5NS4wNDNINzMuMTM5djI0My44MTh6bTgyLjI4OS0zMjkuMTQ4YzAgMjUuMjU4LTIwLjQ1NyA0NS43MTUtNDUuNzE1IDQ1LjcxNS0yNS4yNTggMC00NS43MTUtMjAuNDU3LTQ1LjcxNS00NS43MTUgMC0yNS4yNTggMjAuNDU3LTQ1LjcxNSA0NS43MTUtNDUuNzE1IDI1LjI1OCAwIDQ1LjcxNSAyMC40NTcgNDUuNzE1IDQ1LjcxNXoiLz48L3N2Zz4=" />
            </a>
            <a href="mailto:h.bidiouane@gmail.com"
            title="Email">
                <img src="https://img.shields.io/badge/-4B006E.svg?logo=data:image/svg+xml;base64,//48AHMAdgBnACAAeABtAGwAbgBzAD0AIgBoAHQAdABwADoALwAvAHcAdwB3AC4AdwAzAC4AbwByAGcALwAyADAAMAAwAC8AcwB2AGcAIgAgAHMAaABhAHAAZQAtAHIAZQBuAGQAZQByAGkAbgBnAD0AIgBnAGUAbwBtAGUAdAByAGkAYwBQAHIAZQBjAGkAcwBpAG8AbgAiACAAdABlAHgAdAAtAHIAZQBuAGQAZQByAGkAbgBnAD0AIgBnAGUAbwBtAGUAdAByAGkAYwBQAHIAZQBjAGkAcwBpAG8AbgAiACAAaQBtAGEAZwBlAC0AcgBlAG4AZABlAHIAaQBuAGcAPQAiAG8AcAB0AGkAbQBpAHoAZQBRAHUAYQBsAGkAdAB5ACIAIABmAGkAbABsAC0AcgB1AGwAZQA9ACIAZQB2AGUAbgBvAGQAZAAiACAAYwBsAGkAcAAtAHIAdQBsAGUAPQAiAGUAdgBlAG4AbwBkAGQAIgAgAHYAaQBlAHcAQgBvAHgAPQAiADAAIAAwACAANQAxADIAIAAzADIANwAuADUAMwAiAD4APABwAGEAdABoACAAZgBpAGwAbAA9ACIAIwBmAGYAZgAiACAAZAA9ACIATQAyADUANAAuADQAMQAgADIANwA0AC4AOQA3AGwAMQAwADAALgA5ADMALQAxADAAMAAuADkAMgAgADEANQAzAC4ANAA5ACAAMQA1ADMALgA0ADgASAAwAGwAMQA1ADMALgA0ADkALQAxADUAMwAuADQAOAAgADEAMAAwAC4AOQAyACAAMQAwADAALgA5ADIAegBNADUALgA4ADQAIAAwAGwAMgA0ADgALgA1ADcAIAAyADQAOAAuADUANgBMADUAMAAyAC4AOQA4ACAAMABIADUALgA4ADQAegBNADAAIAAyADkAMAAuADMAbAAxADMAMQAuADcANwAtADEAMwAxAC4ANwA4AEwAMAAgADIANgAuADcANQBWADIAOQAwAC4AMwB6AG0ANQAxADIAIAAxADIALgA2ADEATAAzADYANwAuADYAMQAgADEANQA4AC4ANQAyACAANQAxADIAIAAxADQALgAxADQAdgAyADgAOAAuADcANwB6ACIALwA+ADwALwBzAHYAZwA+AA==" />
            </a>
      <br />
    </td>
  </tr>
</table>

</div>

---

## Abstract

This document describes the architecture and implementation of RushX, a combined terminal emulator and shell written in Rust. RushX is structured as three modules: `rushx_launcher` (CLI dispatch), `rushx_term` (GTK4 terminal emulator with integrated PTY backend), and `rushx_shell` (interactive REPL with tokenizer, redirection parser, builtin commands, and a `fork`/`execvp` execution engine). The terminal emulator spawns the shell via a self-re-exec pattern over a pseudoterminal pair, establishing a POSIX session with proper controlling terminal assignment. Bidirectional I/O between the emulator and the shell flows through the PTY master/slave file descriptors, bridged to the GTK rendering loop via a dedicated reader thread and an `mpsc` channel. This paper documents each subsystem at the syscall level, catalogs the current implementation status, and defines the roadmap toward full POSIX shell compliance.

---

## Table of Contents

- [1. Introduction](#1-introduction)
  - [1.1 Motivation](#11-motivation)
  - [1.2 Design Principles](#12-design-principles)
  - [1.3 Scope & Current State](#13-scope--current-state)
- [2. Architecture](#2-architecture)
  - [2.1 Modular Decomposition](#21-modular-decomposition)
  - [2.2 Single-Binary Self-Re-Exec Model](#22-single-binary-self-re-exec-model)
  - [2.3 Technology Stack](#23-technology-stack)
- [3. Terminal Emulator (`rushx_term`)](#3-terminal-emulator-rushx_term)
  - [3.1 PTY Allocation & Session Establishment](#31-pty-allocation--session-establishment)
  - [3.2 PTY File Descriptor Topology](#32-pty-file-descriptor-topology)
  - [3.3 Terminal I/O Pipeline](#33-terminal-io-pipeline)
  - [3.4 Configuration](#34-configuration)
- [4. Shell (`rushx_shell`)](#4-shell-rushx_shell)
  - [4.1 REPL Loop](#41-repl-loop)
  - [4.2 Parsing](#42-parsing)
  - [4.3 Builtin Commands](#43-builtin-commands)
  - [4.4 External Command Execution](#44-external-command-execution)
  - [4.5 I/O Redirection Mechanism](#45-io-redirection-mechanism)
- [5. POSIX Compliance & Syscall Interface](#5-posix-compliance--syscall-interface)
- [6. Project Status & Roadmap](#6-project-status--roadmap)
  - [6.1 Implementation Status Matrix](#61-implementation-status-matrix)
  - [6.2 Known Limitations](#62-known-limitations)
  - [6.3 Roadmap](#63-roadmap)
- [7. Building & Installation](#7-building--installation)
  - [7.1 Build from Source](#71-build-from-source)
  - [7.2 Install via APT](#72-install-via-apt)
- [8. License](#8-license)
- [9. References](#9-references)

---

## 1. Introduction

### 1.1 Motivation

Most terminal emulators delegate shell functionality to `/bin/bash` or `/bin/sh`, treating the shell as an opaque subprocess. Conversely, most shells assume they run inside an existing terminal and never concern themselves with PTY allocation, screen rendering, or keyboard translation. RushX merges **both** roles into a <ins>single binary</ins> to expose and control every layer of the stack: from `openpty(3)` allocation and `setsid(2)` session creation, through byte-level I/O over the PTY master/slave pair, down to `fork(2)`/`execvp(3)` process overlay and `waitpid(2)` child reaping.

The choice of Rust is deliberate. The `fork(2)` + `exec(3)` boundary is one of the most error-prone areas in systems programming: file descriptor leaks, use-after-fork of heap-allocated data, and async-signal-unsafe function calls in the child process are common sources of undefined behavior. Rust's ownership model and the `nix` crate's typed wrappers provide compile-time guarantees around resource lifecycle that C does not offer, while still permitting raw `libc` calls where no safe abstraction exists (e.g., `ioctl(TIOCSCTTY)`).

### 1.2 Design Principles

- **Single-binary architecture.** The terminal emulator and shell are compiled into one executable. Mode selection is determined at runtime by the presence of the `--rushx-shell` flag in `argv`. The terminal emulator spawns the shell by re-invoking itself via `/proc/self/exe`, eliminating the need for a separate shell binary or an embedded interpreter.

- **Direct syscall interface.** RushX does not depend on `libvte`, `libreadline`, or any terminal abstraction library. PTY allocation, session management, fd wiring, process creation, and child lifecycle are handled through direct system calls wrapped by the `nix` crate. The only high-level dependency is GTK4 for window management and Cairo for text rendering.

- **Modular decomposition.** The codebase is partitioned into three top-level modules (`rushx_launcher`, `rushx_term`, `rushx_shell`) with strict boundaries. The shell module is further decomposed into `parser`, `exec`, `core`, and `expand` submodules, each responsible for a single phase of command processing.

- **Linux-first target.** RushX targets Linux exclusively. It relies on `/proc/self/exe` for self-re-exec, `TIOCSCTTY` for controlling terminal assignment, and glibc's `openpty(3)` for PTY pair allocation. No portability layer exists for macOS, FreeBSD, or other POSIX systems at this time.

### 1.3 Scope & Current State

> [!IMPORTANT]
> RushX is in early development. The following capabilities are implemented and functional:

| Subsystem | Status |
|:----------|:-------|
| GTK4 terminal window with Cairo text rendering | Functional |
| PTY allocation, session establishment, self-re-exec | Functional |
| Bidirectional I/O (reader thread, poll timer, keyboard handler) | Functional |
| Shell REPL with prompt, line reading, EOF handling | Functional |
| Quote-aware argument tokenizer (single, double, backslash) | Functional |
| Output redirection parsing (`>`, `>>`, `1>`, `2>`, `2>>`) | Functional |
| Builtin commands: `exit`, `echo`, `type`, `pwd`, `cd` | Functional |
| External command execution via `fork`/`execvp`/`waitpid` | Functional |
| CSI/OSC escape sequence stripping (partial) | Functional |

The following are scaffolded (module files exist with documentation headers but no implementation): lexer, AST definitions, error types, shell state management, variable expansion, globbing, and PATH resolution (duplicated in `exec` but absent from `expand`). Pipelines, job control, signal handling, and VT100 terminal emulation are not yet implemented. Section 6 provides the complete status matrix and roadmap.

---

## 2. Architecture

> [!IMPORTANT]
> This represents our current architectural vision for RushX. As development progresses, this design may evolve based on implementation discoveries.

<div align="center">

![RushX's Lifecycle](./assets/RushX_Lifecycle.png)

_**Figure 1**: RushX Terminal & Shell Command Execution Lifecycle - Architecture overview depicting the five-phase process flow._

</div>

### 2.1 Modular Decomposition

<div align="center">
    <img alt="RushX Modular Decomposition" src="./assets/1_RushX_Modules.png" width="1000"/>
</div>

<div align="center">

_**Figure 2**: RushX module hierarchy. Rounded boxes: top-level modules. Solid-border boxes: submodules. Inner boxes: subsubmodules._

</div>

<!-- TODO -->

### 2.2 Single-Binary Self-Re-Exec Model

<div align="center">
    <img alt="RushX Self-Re-Exec Bootstrapping Sequence" src="./assets/2_RushX_Self-Re-Exec-bootstrap.png" width="1000"/>
</div>

<div align="center">

_**Figure 3**: Self-re-exec bootstrapping sequence. Left swimlane: parent process (terminal emulator). Right swimlane: child process (shell). Dashed arrow marks the `fork(2)` boundary._

</div>

<!-- TODO -->

### 2.3 Technology Stack

<!-- TODO -->

---

## 3. Terminal Emulator (`rushx_term`)

### 3.1 PTY Allocation & Session Establishment

<!-- TODO -->

### 3.2 PTY File Descriptor Topology

<div align="center">
    <img alt="RushX PTY File Descriptor Topology" src="./assets/3_RushX_FD-Topology.png" width="1000"/>
</div>

<div align="center">

_**Figure 4**: PTY file descriptor topology. Green arrows: shell input path (keyboard to stdin via master/slave). Blue arrows: shell output path (stdout/stderr through slave to master for screen rendering)._

</div>

<!-- TODO -->

### 3.3 Terminal I/O Pipeline

<div align="center">
    <img alt="RushX Terminal Emulator I/O Pipeline" src="./assets/4_RushX_terminal_shell-IO-pipeline.png" width="1000"/>
</div>

<div align="center">

_**Figure 5**: Terminal emulator bidirectional I/O pipeline. Left half: parent process (GTK main thread + reader thread). Center: kernel PTY layer. Right half: child process (shell REPL loop)._

</div>

#### 3.3.1 Reader Thread

<!-- TODO -->

#### 3.3.2 Poll Timer

<!-- TODO -->

#### 3.3.3 PTY Output Processing

<!-- TODO -->

#### 3.3.4 Rendering

<!-- TODO -->

#### 3.3.5 Keyboard Input

<!-- TODO -->

### 3.4 Configuration

<!-- TODO -->

---

## 4. Shell (`rushx_shell`)

### 4.1 REPL Loop

<div align="center">
    <img alt="RushX Shell REPL Dispatch Flowchart" src="./assets/5_RushX_Shell_REPL_Dispatch_Flowchart.png" width="700"/>
</div>

<div align="center">

_**Figure 6**: Shell REPL dispatch flowchart. Diamonds: branch conditions. Rounded boxes: operations. Green arrows: normal flow. Red arrows: error/exit paths._

</div>

<!-- TODO -->

### 4.2 Parsing

#### 4.2.1 Argument Tokenization

<!-- TODO -->

#### 4.2.2 Redirection Parsing

<!-- TODO -->

### 4.3 Builtin Commands

<!-- TODO -->

### 4.4 External Command Execution

#### 4.4.1 PATH Resolution

<!-- TODO -->

#### 4.4.2 Fork/Exec Lifecycle

<!-- TODO -->

### 4.5 I/O Redirection Mechanism

<!-- TODO -->

---

## 5. POSIX Compliance & Syscall Interface

<!-- TODO -->

---

## 6. Project Status & Roadmap

### 6.1 Implementation Status Matrix

<!-- TODO -->

### 6.2 Known Limitations

<!-- TODO -->

### 6.3 Roadmap

<!-- TODO -->

---

## 7. Building & Installation

### 7.1 Build from Source

<!-- TODO -->

### 7.2 Install via APT

```bash
curl -fsSL https://raw.githubusercontent.com/The-HaiKaw-Pr0tocol/rushx/main/install.sh | sudo bash

sudo apt-get install -y rushx
```

---

## 8. License

<!-- TODO -->

---

## 9. References

<!-- TODO -->
