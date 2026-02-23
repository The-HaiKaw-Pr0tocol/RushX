# RushX

### Overview

RushX (Rust Shell - eXtended) is a POSIX-compliant Linux <ins>**terminal emulator**</ins> & <ins>**shell**</ins> implemented in Rust. It ships as a single binary: A GTK4-based terminal emulator that allocates a pseudoterminal (PTY) and renders output via Cairo, and calls by default an interactive POSIX-style shell that performs tokenization, redirection parsing, builtin dispatch, and `fork(2)`/`execvp(3)` execution of external commands.

RushX interfaces directly with the Linux kernel for process creation, session management, and controlling terminal assignment. No external libraries handle PTY allocation, signal delivery, or process lifecycle, and the <ins>**_nix_**</ins> crate provides safe Rust wrappers around the main syscalls, while raw <ins>**_libc::ioctl_**</ins> is used where no safe wrapper exists.

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

## Table of Contents

- [1. Introduction & Motivation](#1-introduction--motivation)
- [2. Architecture](#2-architecture)
  - [2.1 Modular Decomposition](#21-modular-decomposition)
  - [2.2 Single-Binary Self-Re-Exec Model](#22-single-binary-self-re-exec-model)
- [3. Terminal Emulator (`rushx_term`)](#3-terminal-emulator-rushx_term)
  - [3.1 PTY Allocation & Session Establishment](#31-pty-allocation--session-establishment)
  - [3.2 PTY File Descriptor Topology](#32-pty-file-descriptor-topology)
  - [3.3 Terminal I/O Pipeline](#33-terminal-io-pipeline)
    - [3.3.1 Reader Thread](#331-reader-thread)
    - [3.3.2 Poll Timer](#332-poll-timer)
    - [3.3.3 PTY Output Processing](#333-pty-output-processing)
    - [3.3.4 Rendering](#334-rendering)
    - [3.3.5 Keyboard Input](#335-keyboard-input)
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

<br />

---

## 1. Introduction & Motivation

<div align="center">
    <img alt="Terminal emulators under the hood" src="./assets/terminal_emulators_unnder-the-hood.png" width="800"/>
</div>

<div align="center">

_How classic terminals input/output flows: from the physical terminal over UART into the kernel’s TTY stack (driver + line discipline), and then to user processes. Source: [Terminal Emulators Under the Hood](https://funinkina.is-a.dev/blog/terminal-emulators-under-the-hood/)._

</div>

<br/>

Early UNIX systems used **physical terminals** connected over serial lines. The kernel exposed them through the **TTY subsystem**, giving programs a uniform byte-stream interface.

As hardware terminals disappeared, <ins>**terminal emulators**</ins> is what replaced them, and to preserve the TTY abstraction, UNIX introduced **pseudoterminals (PTYs)**: the emulator talks to the _PTY master process_, while shells and programs attach to the _slave side_ as if it were a **real terminal**.

---

Now mosst terminal emulators treat the shell as an opaque subprocess, while shells assume an existing terminal. This creates a strict split: terminal emulator ↔ PTY ↔ shell.

**RushX collapses this split into a single binary.** One process owns PTY setup, session creation, byte-level I/O, `fork`/`exec`, and child reaping, merging terminal and shell into one program.

<br/>

## 2. Architecture

<div align="center">

![RushX's Lifecycle](./assets/RushX_Lifecycle.png)

_**Figure 1**: RushX Terminal & Shell Command Execution Lifecycle - Architecture overview depicting the five-phase process flow._

</div>

> [!IMPORTANT]
> This represents our current architectural vision for RushX. As development progresses, this design may evolve based on implementation discoveries.

<br/>

Well, the considered full command execution lifecycle of RushX could be mainly sectioned into five phases:

- **_Phase I_: <ins>Terminal startup, PTY plumbing, and shell spawn</ins>**

The terminal emulator process allocates a PTY master/slave pair from the Kernel, then forks and re-execs itself to spawn the shell as a child process attached to the PTY slave side. The shell then sets up its environment, and prints the prompt. This is the "plumbing" stage: after it completes, the master fd belongs to the emulator and the slave fd is wired to the shell's **stdin/stdout/stderr**.

- **_Phase II_: <ins>REPL loop</ins>**

The user types a command into the terminal window. Keystrokes travel through the PTY master into the slave, where the shell reads a line as a byte stream, tokenizes it, and identifies whether it maps to a builtin or an external program.

- **_Phase III_: <ins>fork/exec command execution</ins>**

For external commands, the shell calls `fork(2)` to create a child program process that inherits the PTY slave as its foreground terminal. The child then calls `exec(*path, **args)` to overlay itself with the target binary. The running program issues its own syscalls against the kernel directly.

- **_Phase IV_: <ins>Termination and recovery</ins>**

When the child calls `_exit()`, it becomes a zombie. The kernel delivers `SIGCHLD` to the shell, which wakes up, collects the exit status via `waitpid(2)`, reclaims PTY foreground ownership, and reprints the prompt.

- **_Phase V_: <ins>Re-loop</ins>**

Control returns to Phase II. The shell waits for the next line of input.

Every external command repeats Phases II through V. The PTY pair established in Phase I persists for the lifetime of the terminal session, serving as the single communication channel between the emulator and the shell. The emulator never interprets commands; the shell never renders pixels. Each process owns exactly one side of the PTY and one half of the responsibility.

<br/>

### 2.1 Modular Decomposition

<div align="center">
    <img alt="RushX Modular Decomposition" src="./assets/1_RushX_Modules.png" width="1000"/>
</div>

<div align="center">

_**Figure 2**: RushX module hierarchy. Rounded boxes: top-level modules. Solid-border boxes: submodules. Inner boxes: subsubmodules._

</div>

<br/>

Figure 2 maps out the full module tree. RushX is partitioned into 3 top-level modules ( for now :) ), each declared in [main.rs](./src/main.rs) and routed through the **rushx_launcher** module:

1. **`rushx_launcher`** is the thinnest layer. It contains a single function : **run()**, that inspects **_argv_** for an experimental <ins>**--rushx-shell**</ins> flag and branches into either **rushx_term::run_rushx_terminal()** or **rushx_shell::run_rushx_shell()**. No other logic lives here. Its used to test the shell independently and to kind of fork the shell process from the same binary when the terminal emulators spawns it.

---

2. **`rushx_term`** is the terminal emulator. Its root : [mod.rs](./src/rushx_term/mod.rs) builds the GTK4 window, wires the I/O pipeline (reader thread, poll timer, draw function, blink timer, keyboard handler), and runs the **process_pty_output()** state machine that strips escape sequences and feeds characters to the Cairo renderer. Two submodules support it:
   - **2.1** _<ins>`pty`</ins>_ : Allocates the PTY master/slave pair and spawns the shell child process via fork/exec.

   - **2.2** _<ins>`config`</ins>_ : Defines compile-time constants: application ID, window geometry, colors, font, shell path (**_/proc/self/exe_**), shell flag (**_--rushx-shell_**), and buffer sizes.

---

3. **`rushx_shell`** is the RushX shell. Its root [mod.rs](./src/rushx_shell/mod.rs) runs the REPL loop: print prompt, read line, dispatch to builtin or external command. Four submodules handle the rest:
   - **3.1** _<ins>`parser`</ins>_ : Implements the quote-aware argument tokenizer and the redirection parser. **lexer.rs** is scaffolded for a future token-stream lexer but currently empty.

   - **3.2** _<ins>`exec`</ins>_ : Implements **run_external()** (fork, fd redirection via **open/dup2**, **execvp**, **waitpid**), **find_executable_in_path()**, and **is_builtin()**.

   - **3.3** _<ins>`core`</ins>_ (scaffolded) : [ast.rs](./src/rushx_shell/core/ast.rs), [error.rs](./src/rushx_shell/core/error.rs), and [state.rs](./src/rushx_shell/core/state.rs) exist as empty files reserved for AST node definitions, structured error types, and shell state (variables, exit codes, options).

   - **3.4** _<ins>`expand`</ins>_ (scaffolded) : [glob.rs](./src/rushx_shell/expand/glob.rs), [vars.rs](./src/rushx_shell/expand/vars.rs), and [path.rs](./src/rushx_shell/expand/path.rs) are reserved for glob expansion, variable/tilde expansion, and PATH resolution respectively.

<br/>

### 2.2 Single-Binary Self-Re-Exec Model

<div align="center">
    <img alt="RushX Self-Re-Exec Bootstrapping Sequence" src="./assets/2_RushX_Self-Re-Exec-bootstrap.png" width="1000"/>
</div>

<div align="center">

_**Figure 3**: Self-re-exec bootstrapping sequence. Left swimlane: parent process (terminal emulator). Right swimlane: child process (shell). Dashed arrow marks the fork(2) boundary._

</div>

<br />

Figure 3 traces the bootstrapping sequence step by step. The entire ceremony happens inside <ins>**spawn_shell()**</ins> in [pty.rs](src/rushx_term/pty.rs), called once at terminal startup.

The parent process (terminal emulator) begins by calling **openpty(3)** to allocate a PTY master/slave pair. It then prepares the **CString** arguments for **execvp** while still single-threaded, before any **fork(2)**.

> [!CAUTION]
> This is important: heap allocation after fork is unsafe in a multithreaded process, so all `CString` construction happens on the parent side.

The parent then calls **fork(2)**. From this point, two processes exist with identical memory.

**<ins>Child path (right swimlane)</ins>:**

1. Close the master fd. The child has no use for it.
2. **_setsid(2)_** to create a new session. The child becomes session leader, detached from the parent's controlling terminal.
3. **_ioctl(slave_fd, TIOCSCTTY)_** to claim the PTY slave as the session's controlling terminal. This is a raw `libc::ioctl` call because no safe Rust wrapper exists.
4. **_dup2(slave_fd, 0)_**, **_dup2(slave_fd, 1)_**, **_dup2(slave_fd, 2)_** to wire stdin, stdout, and stderr to the slave.
5. Close the original slave fd (it is now duplicated onto fds 0, 1, 2).
6. **_execvp("/proc/self/exe", ["rushx", "--rushx-shell"])_** to overlay the child's memory with a fresh invocation of the same binary. The **--rushx-shell** flag causes **_rushx_launcher::run()_** to branch into **_rushx_shell::run_rushx_shell()_** instead of launching another terminal window.

If **_execvp_** fails, the child calls **_libc::\_exit(1)_** directly to avoid running Rust destructors or **_atexit_** handlers in the forked address space.

**<ins>Parent path (left swimlane):</ins>**

1. Close the slave fd. Only the child uses it.
2. Return `SpawnedShell { master_fd, child_pid }` to the terminal emulator, which uses **_master_fd_** for all subsequent I/O and **_child_pid_** for lifecycle management.

The key insight is that `/proc/self/exe` always points to the currently running binary. The child does not spawn an external shell; it re-executes itself. The flag in **_argv_** is the only thing that distinguishes a terminal emulator process from a shell process.

<br />

> **Check File**

> [rushx_term/pty.rs](./src/rushx_term/pty.rs)

<br />

## 3. Terminal Emulator (`rushx_term`)

### 3.1 PTY Allocation & Session Establishment

The terminal emulator's first action in **_run_rushx_terminal()_** is to set up the PTY pair and spawn the shell. Two function calls in [pty.rs](src/rushx_term/pty.rs) handle the whole sequence:

```rust
    let pty_pair = pty::open_pty_pair().expect("Failed to allocate PTY pair");
    let shell    = pty::spawn_shell(pty_pair).expect("Failed to spawn shell process");
```

`open_pty_pair()` calls _**openpty(3)**_ through the `nix` crate (internally performs **_posix_openpt_** + **_grantpt_** + **_unlockpt_** + **_ptsname_**). It returns a <ins>**PtyPair**</ins> struct holding two raw file descriptors: `master` (retained by the emulator) and `slave` (passed to the child). No termios configuration is applied; the PTY uses kernel defaults.

**`spawn_shell()`** consumes the <ins>**PtyPair**</ins> and performs the full POSIX session establishment ceremony described in [Section 2.2](#22-single-binary-self-re-exec-model).

### 3.2 PTY File Descriptor Topology

<div align="center">
    <img alt="RushX PTY File Descriptor Topology" src="./assets/3_RushX_FD-Topology.png" width="1000"/>
</div>

<div align="center">

_**Figure 4**: PTY file descriptor topology. Green arrows: shell input path (keyboard to stdin via master/slave). Blue arrows: shell output path (stdout/stderr through slave to master for screen rendering)._

</div>

Figure 4 shows the fd layout after `spawn_shell()` completes and both processes have closed the fds they do not own.

The **Terminal Emulator Process** (parent). It holds a single fd: the **PTY master**. This is the only handle the emulator uses for all communication with the shell. Bytes written to the master appear on the shell's stdin; bytes the shell writes to stdout or stderr are readable from the master.

The **Kernel PTY layer** connects the two sides. The kernel's line discipline sits between master and slave, handling echo, line buffering, and signal generation (Ctrl-C, Ctrl-Z) transparently. Neither the emulator nor the shell manages these behaviors manually.

Finally, we have the **Shell Process** (child). After the `dup2` calls in **_spawn_shell()_**, the PTY slave fd has been duplicated onto three standard file descriptors:

- **fd 0** (stdin) : The shell reads user input from here.
- **fd 1** (stdout) : Builtin output and external command output goes here.
- **fd 2** (stderr) : Error messages go here.

All three point to the same underlying PTY slave device. The original slave fd is closed after duplication (assuming it was > 2), so the shell's fd table contains exactly fds 0, 1, and 2.

Keystrokes captured by the GTK keyboard handler are written to the master fd, pass through the kernel, and arrive at the shell's fd 0. For the shell's output, bytes written by the shell or its child programs to fd 1 or fd 2 travel back through the slave, through the kernel, and become readable on the master fd, where the emulator's reader thread picks them up for rendering.

This two-fd-endpoint topology (one master fd in the parent, three slave-backed fds in the child) is the standard POSIX PTY pattern. RushX does not deviate from it.

### 3.3 Terminal I/O Pipeline

<div align="center">
    <img alt="RushX Terminal Emulator I/O Pipeline" src="./assets/4_RushX_terminal_shell-IO-pipeline.png" width="1000"/>
</div>

<div align="center">

_**Figure 5**: Terminal emulator bidirectional I/O pipeline. Left half: parent process (GTK main thread + reader thread). Center: kernel PTY layer. Right half: child process (shell REPL loop)._

</div>

The previous section diagrammed the file descriptor topology: one master fd in the emulator, three slave-backed fds in the shell, kernel PTY layer in between. But that view only showed the shell's side of the flow and the raw fd wiring. It did not show what actually happens inside **<ins>the terminal emulator</ins>**, the process that feeds the master fd on the input side and consumes it on the output side.

Figure 5 fills that gap. It expands the emulator (left half) into its internal components and traces the full bidirectional datapath end to end.

On the **output path** (shell to screen): the shell writes bytes to fd 1 or fd 2. Those bytes travel through the PTY slave, through the kernel, and become readable on the master fd. A dedicated **reader thread** in the emulator blocks on that master fd and pushes raw byte chunks through an **mpsc channel** to the GTK main thread. There, a **poll timer** firing every 16 ms drains the channel, feeds each chunk through a byte-level state machine (**process_pty_output**) that strips escape sequences and interprets control characters, and triggers a **Cairo redraw** that paints the updated text buffer to the screen.

On the **input path** (keyboard to shell): the GTK main thread captures key-press events through a **keyboard handler**, translates them into terminal byte sequences (printable characters, ANSI escapes, control codes), and writes them directly to the master fd. The bytes pass through the kernel into the slave, where the shell reads them from fd 0.

The five subsections below walk through each emulator-side component in detail.

#### 3.3.1 Reader Thread

`spawn_reader_thread()` creates a standard mpsc channel, then spawns a background thread that loops indefinitely, calling **read()** on the PTY master fd into a 4096-byte stack buffer. Each successful read copies the live portion of the buffer into a byte vector and sends it through the channel.

The thread terminates on any of three conditions: **read()** returns 0 (EOF, the shell closed stdout), **read()** returns EIO (the slave side of the PTY was closed), or the channel receiver has been dropped (the GTK side shut down). In all cases the thread exits silently, which disconnects the sending half of the channel and signals the poll timer to close the window.

The function returns the receiving end of the channel to **build_ui**, which hands it off to the poll timer.

#### 3.3.2 Poll Timer

`setup_poll_timer()` registers a GLib timeout on the GTK main loop running every **16 ms** (roughly 60 fps). Each tick executes a tight **try_recv** loop that drains every pending message from the reader thread's channel without blocking.

For each received byte chunk, the timer calls **process_pty_output()** to interpret the raw bytes and append the results to the shared text buffer. If at least one chunk arrived during the tick, the timer resets cursor visibility to true (interrupting any ongoing blink) and calls **queue_draw()** on the DrawingArea to schedule a Cairo repaint.

When **try_recv** returns Disconnected, the shell has exited and the reader thread has terminated. The timer responds by closing the application window, which causes GTK to quit.

#### 3.3.3 PTY Output Processing

`process_pty_output()` is the byte-level state machine that sits between raw PTY data and the text buffer. It converts the incoming bytes to a string via lossy UTF-8 decoding, then iterates character by character with a peekable iterator.

The rules it applies, in order of matching priority:

- **`\x08`** (Backspace): pops the last character from the buffer, unless the buffer is empty or the last character is a newline.
- **`\r\n`**: consumed as a single newline (peek ahead to detect the pair).
- **`\r`** alone (standalone carriage return): truncates the buffer back to the start of the current line, or clears entirely if there is no newline. This handles progress bars and overwrite-style output.
- **`\x1b[`** (CSI sequence): consumes all parameter bytes until a final byte in the @..~ range. Currently only `\x1b[2J` and `\x1b[3J` (Erase in Display) are acted upon, both of which clear the buffer. All other CSI sequences are silently stripped.
- **`\x1b]`** (OSC sequence): consumes everything until BEL (`\x07`) or ST (`\x1b\\`). Used by shells to set the terminal title; RushX discards the payload.
- **`\x07`** (BEL), **`\x00`** (NUL): silently ignored.
- **Everything else**: appended to the buffer verbatim.

The function modifies the buffer in place. No allocations occur per character; only push and truncate operations on the existing string.

#### 3.3.4 Rendering

`setup_draw_func()` installs a Cairo draw callback on the GTK DrawingArea. Every time **queue_draw()** is called (by the poll timer or the blink timer), GTK invokes this callback with a Cairo context and the current widget dimensions.

The rendering proceeds in three passes:

1. **Background fill.** The entire widget area is filled with the background color (#1e1e2e, a dark blue-gray).

2. **Text.** The callback selects a monospace font at size 14.0 via Cairo's toy text API, sets the foreground to #cdd6f4 (light gray), and splits the text buffer on newlines. It computes how many lines fit the window height, then takes the last N lines (auto-scroll to bottom). Each visible line is drawn with y incrementing by LINE_HEIGHT (18.0 px).

3. **Cursor.** If the cursor is visible, a filled rectangle is drawn at the end of the last visible line. The x position is computed from the text extents of that line, and the height from the font's ascent + descent. The result is a block cursor one "M"-width wide.

A separate **blink timer** (setup_blink_timer) toggles cursor visibility every 500 ms and calls **queue_draw()**, producing the standard blinking block cursor. The poll timer resets visibility to true whenever new output arrives, so the cursor stays solid while the shell is actively printing.

#### 3.3.5 Keyboard Input

`setup_keyboard()` attaches a GTK EventControllerKey to the DrawingArea. On every key-press event, the handler translates the keyval and modifier state into a byte sequence and writes it directly to the PTY master fd via **write()**.

The translation table:

| Input                          | Bytes written to PTY                                    |
| ------------------------------ | ------------------------------------------------------- |
| Ctrl + letter                  | Control character `0x01`..`0x1A` (e.g. Ctrl+C = `0x03`) |
| Enter                          | `\r`                                                    |
| Backspace                      | `\x7f` (DEL)                                            |
| Tab                            | `\t`                                                    |
| Escape                         | `\x1b`                                                  |
| Arrow Up / Down / Right / Left | `\x1b[A` / `\x1b[B` / `\x1b[C` / `\x1b[D`               |
| Delete                         | `\x1b[3~`                                               |
| Home / End                     | `\x1b[H` / `\x1b[F`                                     |
| Printable characters           | UTF-8 encoded bytes (up to 4 bytes per codepoint)       |

Ctrl+letter combinations are checked first: if the CONTROL_MASK modifier is active and the keyval maps to an ASCII letter, the handler computes (lowercase - 'a' + 1) to produce the corresponding control byte and returns immediately. Otherwise, named keys are matched against their GDK key variants, and printable characters fall through to Unicode conversion and UTF-8 encoding.

All writes go to the same master fd that the reader thread reads from on the other side. The kernel PTY layer echoes the bytes back through the slave if the line discipline has echo enabled, so typed characters appear on screen through the normal output path (reader thread to poll timer to renderer), not by direct insertion into the text buffer.

### 3.4 Configuration

All tunable parameters live in a single file: [config.rs](./src/rushx_term/config.rs). Nothing is read from disk or environment variables at runtime; every value is a compile-time constant.

The constants break into five groups:

1. **Application identity.** APP_ID ("haikaw.rushx.terminal") is the GTK/D-Bus application identifier. WINDOW_TITLE ("RushX") is the string shown in the title bar.

2. **Window geometry.** WINDOW_WIDTH (800) and WINDOW_HEIGHT (500) set the default window size in pixels.

3. **Color scheme.** BG_COLOR is a dark blue-gray (#1e1e2e) and FG_COLOR is a light gray (#cdd6f4), both stored as (f64, f64, f64) RGB triples normalized to 0.0..1.0 for Cairo.

4. **Font and text layout.** FONT_FAMILY is "monospace", rendered through Cairo's toy text API. FONT_SIZE is 14.0 (Cairo units, roughly points at 96 DPI). LINE_HEIGHT is 18.0 px (baseline to baseline). TEXT_PADDING is 4.0 px from the window edge.

5. **Shell invocation.** SHELL_PATH is "/proc/self/exe" and SHELL_FLAG is "--rushx-shell". These two constants control the self-re-exec mechanism described in [Section 2.2](#22-single-binary-self-re-exec-model). PTY_READ_BUF_SIZE is 4096 bytes, the stack buffer size used by the reader thread.

Because everything is const, changing any value requires recompilation. There is no runtime configuration file, no CLI flag parsing beyond the single --rushx-shell switch, and no theming support yet.

<br />

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
