# RushX

### Overview

RushX (Rust Shell - eXtended) is a POSIX-compliant Linux <ins>**terminal emulator**</ins> & <ins>**shell**</ins> implemented in Rust. It ships as a single binary: A GTK4-based terminal emulator that allocates a pseudoterminal (PTY), and calls by default an interactive POSIX-style shell that performs tokenization, redirection parsing, builtin dispatch, and execution of external commands.

RushX interfaces directly with the Linux kernel for process creation, session management, and controlling terminal assignment. No external libraries handle PTY allocation, signal delivery, or process lifecycle, it uses safe Rust wrappers around the main syscalls, while raw invocations are used where no safe wrapper exists.

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
  - [3.5 ANSI Escape Sequence Parser](#35-ansi-escape-sequence-parser)
- [4. Shell (`rushx_shell`)](#4-shell-rushx_shell)
  - [4.1 REPL Loop](#41-repl-loop)
  - [4.2 Parsing](#42-parsing)
    - [4.2.1 Argument Tokenization](#421-argument-tokenization)
    - [4.2.2 Redirection Parsing](#422-redirection-parsing)
  - [4.3 Builtin Commands](#43-builtin-commands)
  - [4.4 External Command Execution](#44-external-command-execution)
    - [4.4.1 PATH Resolution](#441-path-resolution)
    - [4.4.2 Fork/Exec Lifecycle](#442-forkexec-lifecycle)
  - [4.5 I/O Redirection Mechanism](#45-io-redirection-mechanism)
  - [4.6 Tab Completion](#46-tab-completion)
- [5. POSIX Compliance & Syscall Interface](#5-posix-compliance--syscall-interface)
- [6. Project Status & Roadmap](#6-project-status--roadmap)
  - [6.1 Implementation Status Matrix](#61-implementation-status-matrix)
  - [6.2 Known Limitations](#62-known-limitations)
  - [6.3 Roadmap](#63-roadmap)

---

- [7. Installation via APT](#7-installation-via-apt)

- [8. References](#8-references)

<br />

---

## 1. Introduction & Motivation

<div align="center">
    <img alt="Terminal emulators under the hood" src="./assets/terminal_emulators_unnder-the-hood.png" width="800"/>
</div>

<div align="center">

Source: [Terminal Emulators Under the Hood](https://funinkina.is-a.dev/blog/terminal-emulators-under-the-hood/)

_How classic terminals input/output flows: from the physical terminal over UART into the kernel’s TTY stack (driver + line discipline), and then to user processes._

</div>

<br/>

Early UNIX systems used **physical terminals** connected over serial lines. The kernel exposed them through the **TTY subsystem**, giving programs a uniform byte-stream interface.

As hardware terminals disappeared, <ins>**terminal emulators**</ins> is what replaced them, and to preserve the TTY abstraction, UNIX introduced **pseudoterminals (PTYs)**: the emulator talks to the _PTY master process_, while shells and programs attach to the _slave side_ as if it were a **real terminal**.

---

Now mosst terminal emulators treat the shell as an opaque subprocess, while shells assume an existing terminal. This creates a strict split: terminal emulator ↔ PTY ↔ shell.

**RushX collapses this split into a single binary.** One process owns PTY setup, session creation, byte-level I/O, `fork`/`exec`, and child reaping, merging terminal and shell into one program.

<br/>

## 2. Architecture

> [!IMPORTANT]
> This represents our current architectural vision for RushX. As development progresses, this design may evolve based on implementation discoveries.

<div align="center">

![RushX's Lifecycle](./assets/RushX_Lifecycle.png)

_**Figure 1**: RushX Terminal & Shell Command Execution Lifecycle - Architecture overview depicting the five-phase process flow._

</div>


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

2. **`rushx_term`** is the terminal emulator. Its root : [mod.rs](./src/rushx_term/mod.rs) builds the GTK4 window, wires the I/O pipeline (reader thread, poll timer, draw function, blink timer, keyboard handler), and runs **process_pty_output()** which feeds raw PTY bytes through a stateful ANSI parser and produces styled text spans for the Cairo renderer. Three submodules support it:
   - **2.1** _<ins>`pty`</ins>_ : Allocates the PTY master/slave pair and spawns the shell child process via fork/exec.

   - **2.2** _<ins>`config`</ins>_ : Defines compile-time constants: application ID, window geometry, default colors, font, shell path (**_/proc/self/exe_**), shell flag (**_--rushx-shell_**), and buffer sizes.

   - **2.3** _<ins>`ansi`</ins>_ : A full ANSI/VT100 escape sequence parser. Implements a four-state machine (Normal, Esc, Csi, Osc), color types (16-color ANSI palette, 256-color indexed, 24-bit RGB), SGR attribute tracking (bold, dim, italic, underline, reverse, strikethrough), and a Catppuccin Mocha color scheme. Produces structured styled spans that the renderer draws with per-segment colors.

---

3. **`rushx_shell`** is the RushX shell. Its root [mod.rs](./src/rushx_shell/mod.rs) runs the REPL loop in raw terminal mode: build a user@hostname:cwd prompt, read input character by character with tab completion, and dispatch to builtin or external command. Four submodules handle the rest:
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

Figure 3 traces the bootstrapping sequence step by step. The entire ceremony happens inside **spawn_shell()** in [pty.rs](src/rushx_term/pty.rs), called once at terminal startup.

The parent process starts by allocating a PTY master/slave pair, then prepares the arguments it will need for exec while still single-threaded.

> [!CAUTION]
> Heap allocation after fork is unsafe in a multithreaded process, so all string construction happens before the fork.

It then calls **fork(2)**. From this point, two processes exist with identical memory.

**<ins>Child path (right swimlane)</ins>:**

The child closes the master fd (only the parent needs it), creates a new session with **setsid**, and claims the PTY slave as its controlling terminal via **ioctl(TIOCSCTTY)**. It then wires stdin, stdout, and stderr to the slave with three **dup2** calls, closes the now-redundant original slave fd, and calls **execvp** to re-execute the same binary with the **--rushx-shell** flag. That flag is what tells the launcher to start the shell instead of another terminal window.

If exec fails, the child exits immediately without running any Rust cleanup, to avoid corrupting shared state in the forked address space.

**<ins>Parent path (left swimlane):</ins>**

The parent closes the slave fd (only the child needs it) and returns the master fd and child PID to the terminal emulator. From this point on, the master fd is the emulator's sole communication channel with the shell.

The key insight is that /proc/self/exe always points to the currently running binary. The child does not spawn an external shell; it re-executes itself. The flag in argv is the only thing that distinguishes a terminal emulator process from a shell process.

<br />

> **Check File**

> [rushx_term/pty.rs](./src/rushx_term/pty.rs)

<br />

## 3. Terminal Emulator (`rushx_term`)

### 3.1 PTY Allocation & Session Establishment

The terminal emulator's first action is to set up the PTY pair and spawn the shell. Two function calls in [pty.rs](src/rushx_term/pty.rs) handle the whole sequence.

**open_pty_pair()** calls **openpty(3)** through the nix crate to allocate a master/slave pair. It returns a struct holding the two raw file descriptors: one for the emulator (master) and one for the child (slave). No termios configuration is applied; the PTY uses kernel defaults.

**spawn_shell()** consumes that pair and performs the full POSIX session establishment ceremony described in [Section 2.2](#22-single-binary-self-re-exec-model): fork, setsid, ioctl for controlling terminal, dup2 onto standard fds, and execvp to re-launch the binary in shell mode.

### 3.2 PTY File Descriptor Topology

<div align="center">
    <img alt="RushX PTY File Descriptor Topology" src="./assets/3_RushX_FD-Topology.png" width="1000"/>
</div>

<div align="center">

_**Figure 4**: PTY file descriptor topology. Green arrows: shell input path (keyboard to stdin via master/slave). Blue arrows: shell output path (stdout/stderr through slave to master for screen rendering)._

</div>

Figure 4 shows the fd layout after shell spawning completes and both processes have closed the fds they do not own.

The **Terminal Emulator Process** (parent) holds a single fd: the **PTY master**. This is its only handle for all communication with the shell. Bytes written to the master appear on the shell's stdin; bytes the shell writes to stdout or stderr become readable from the master.

The **Kernel PTY layer** connects the two sides. The line discipline sits between master and slave, handling echo, line buffering, and signal generation (Ctrl-C, Ctrl-Z) transparently. Neither the emulator nor the shell manages these behaviors manually.

The **Shell Process** (child) sees three standard file descriptors, all pointing to the same PTY slave device:

- **fd 0** (stdin) : reads user input.
- **fd 1** (stdout) : builtin and command output.
- **fd 2** (stderr) : error messages.

Keystrokes captured by the GTK keyboard handler are written to the master fd, pass through the kernel, and arrive at the shell's fd 0. In the other direction, bytes written to fd 1 or fd 2 travel back through the slave, through the kernel, and become readable on the master fd, where the emulator's reader thread picks them up for rendering.

This is the standard POSIX PTY pattern. RushX does not deviate from it.

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

A dedicated background thread sits in a blocking read loop on the PTY master fd, reading into a 4096-byte stack buffer. Each successful read is copied into a byte vector and sent through a standard mpsc channel to the GTK main thread.

The thread terminates when the read returns EOF (shell closed stdout), EIO (slave side closed), or the channel receiver has been dropped (GTK shut down). In all cases the thread exits silently, disconnecting the channel and signaling the poll timer to close the window.

#### 3.3.2 Poll Timer

A GLib timeout fires every **16 ms** (roughly 60 fps) on the GTK main loop. Each tick drains all pending messages from the reader thread's channel without blocking.

For each byte chunk received, the timer feeds it through the output processing state machine and appends the results to the shared text buffer. If anything arrived, it resets the cursor to visible (interrupting any ongoing blink) and schedules a Cairo repaint.

When the channel disconnects (shell exited, reader thread gone), the timer closes the application window, which causes GTK to quit.

#### 3.3.3 PTY Output Processing

The **process_pty_output()** function sits between raw PTY data and the styled text buffer. It feeds each incoming byte chunk to the **AnsiParser** (described in [Section 3.5](#35-ansi-escape-sequence-parser)), which returns a stream of discrete **Action** values: styled text spans, backspace, carriage return, erase-display, or bell.

Each action is applied to the buffer by **apply_action()**: text spans are appended with their current color and attribute state, backspace removes the last character, carriage return truncates to the start of the current line, and erase-display clears the buffer entirely. If any bell action is encountered, the function signals the poll timer to play the system beep via GDK.

The parser is stateful across calls. An escape sequence that arrives split across two read chunks is reassembled correctly because the parser retains its position in the state machine between invocations.

#### 3.3.4 Rendering

A Cairo draw callback is installed on the GTK DrawingArea. Before painting, **build_render_lines()** transforms the flat span buffer into a two-dimensional structure: a list of lines, where each line is a list of (text, style) segments. This function handles newline splitting and preserves per-segment color and attribute information.

The draw pass then proceeds in three stages:

1. **Background fill** with the default dark blue-gray (#1e1e2e).
2. **Styled text**, rendered segment by segment. Each segment carries its own foreground color, background color, and attributes (reverse video, etc.). If a segment has a non-default background, a colored rectangle is painted behind it before the text. Reverse video swaps the foreground and background. Only the last N lines that fit the window height are drawn (auto-scroll to bottom).
3. **Block cursor** at the end of the last visible line, one character wide.

Color values come from two sources: the default terminal foreground and background live in [config.rs](./src/rushx_term/config.rs), while the 16-color ANSI palette (Catppuccin Mocha theme), 256-color indexed values, and 24-bit RGB are resolved by the Color type in [ansi.rs](./src/rushx_term/ansi.rs).

A separate **blink timer** toggles cursor visibility every 500 ms. The poll timer resets it to visible whenever new output arrives, so the cursor stays solid while the shell is actively printing.

#### 3.3.5 Keyboard Input

A GTK key event controller is attached to the DrawingArea. On every key press, it translates the key into a byte sequence and writes it to the PTY master fd.

The mapping follows standard terminal conventions: Enter sends a carriage return, Backspace sends DEL (0x7F), arrow keys send ANSI escape sequences (ESC [ A/B/C/D), Ctrl+letter sends the corresponding control character (0x01 through 0x1A), and printable characters are encoded as UTF-8.

All writes go to the same master fd that the reader thread reads from on the other side. The kernel PTY layer echoes the bytes back through the slave if the line discipline has echo enabled, so typed characters appear on screen through the normal output path, not by direct insertion into the text buffer.

### 3.4 Configuration

All tunable parameters live in a single file: [config.rs](./src/rushx_term/config.rs). Nothing is read from disk or environment variables at runtime; every value is a compile-time constant.

The constants cover application identity (GTK/D-Bus ID, window title), window geometry (800 x 500 default), color scheme (dark blue-gray background #1e1e2e, light gray text #cdd6f4), font and text layout (monospace at 14pt, 18 px line height, 4 px padding), and shell invocation (/proc/self/exe path, --rushx-shell flag, 4096-byte read buffer).

Because everything is const, changing any value requires recompilation. There is no runtime configuration file, no CLI flags beyond the single --rushx-shell switch, and no theming support yet.

### 3.5 ANSI Escape Sequence Parser

The ANSI parser in [ansi.rs](./src/rushx_term/ansi.rs) converts raw PTY byte streams into styled, colored output. It is built around a four-state machine (Normal, Esc, Csi, Osc). In Normal state, printable characters accumulate into styled text spans; an ESC byte transitions to Esc, which branches into Csi (for CSI sequences like SGR color commands and erase-display) or Osc (for operating system commands, which are consumed and discarded). Control characters are handled directly: backspace, carriage return, and BEL each produce a corresponding action. The parser is stateful across calls, so escape sequences split across read chunks are reassembled correctly.

The SGR interpreter supports the full standard set: text attributes (bold, dim, italic, underline, blink, reverse video, strikethrough), foreground and background colors via codes 30--37 / 40--47 and their bright variants 90--97 / 100--107, 256-color indexed palettes (38;5;n / 48;5;n), and 24-bit RGB (38;2;r;g;b / 48;2;r;g;b). The base 16-color palette uses the **Catppuccin Mocha** color scheme. The parser produces **StyledSpan** values, each carrying a text string and an attribute snapshot (colors and flags at the time of emission), which are collected into a flat buffer and transformed into renderable lines by **build_render_lines()** for the Cairo draw callback.

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

Figure 6 traces the shell's main loop. The cycle begins at the top with the prompt ("$ "), flows down through line reading, tokenization, and a branch: is the command a builtin or an external program? Each path leads to execution, then loops back to the prompt. The red exit paths show the two ways the loop terminates: the user types "exit", or stdin hits EOF.

The REPL lives in **run_rushx_shell()** in [mod.rs](./src/rushx_shell/mod.rs). Each iteration prints the prompt, flushes stdout, reads a line from stdin, and passes it through the tokenizer and redirection parser. If the first token matches a builtin name, the shell handles it inline. Otherwise, control passes to the external execution engine. The loop repeats until the "exit" builtin is invoked or the read fails (EOF / broken pipe).

The shell maintains a single piece of persistent state across iterations: the previous working directory (**OLDPWD**), used by "cd -" to jump back.

### 4.2 Parsing

#### 4.2.1 Argument Tokenization

The tokenizer in [parse.rs](./src/rushx_shell/parser/parse.rs) splits a raw input line into an argument vector, respecting POSIX-style quoting rules.

It walks the input character by character, tracking whether it is currently inside single quotes, double quotes, or unquoted context. **Single quotes** preserve everything literally with no escape processing. **Double quotes** allow backslash escapes for a small set of characters (backslash itself, double quote, dollar sign, backtick, and newline). **Unquoted backslash** escapes the immediately following character. Whitespace outside quotes delimits arguments.

The result is a flat vector of strings, one per argument. No variable expansion, glob expansion, or tilde expansion happens at this stage; those modules are scaffolded but not yet implemented.

#### 4.2.2 Redirection Parsing

After tokenization, the argument vector is scanned for redirection operators. The parser recognizes six forms:

- **>** and **1>** : redirect stdout to a file (truncate)
- **>>** and **1>>** : redirect stdout to a file (append)
- **2>** : redirect stderr to a file (truncate)
- **2>>** : redirect stderr to a file (append)

Each operator consumes the next token as the target filename. The parser separates redirection instructions from command arguments and returns a structured result containing the clean argument list, an optional stdout redirection, and an optional stderr redirection.

Input redirection (< ) and pipe operators ( | ) are not yet supported.

### 4.3 Builtin Commands

The shell recognizes five builtin commands, dispatched inline in the REPL loop without forking a child process:

- **exit** : terminates the shell loop immediately.
- **echo** : prints its arguments joined by spaces, followed by a newline. With no arguments, prints a blank line.
- **type** : reports whether a command is a builtin or an external program. For externals, prints the resolved path. If the command is not found, says so.
- **pwd** : prints the current working directory.
- **cd** : changes the working directory. Supports "~" (home), "~/subdir" (home-relative), "-" (previous directory via OLDPWD), and absolute or relative paths. Prints an error if the target does not exist or is not a directory.

All builtins write to the redirected output if a redirection is active, rather than always writing to the real stdout/stderr. This is handled by passing a writer object (either a file or the standard stream) into each builtin.

### 4.4 External Command Execution

#### 4.4.1 PATH Resolution

When a command is not a builtin, the shell searches the PATH environment variable for a matching executable. It splits PATH on the platform separator, joins the command name to each directory, and checks whether the resulting path points to a regular file with at least one execute permission bit set (owner, group, or other).

The first match wins. If no match is found, the shell prints "command not found" and returns to the prompt without forking.

#### 4.4.2 Fork/Exec Lifecycle

Once a valid executable path is resolved, the shell forks a child process. The child applies any active redirections (opening the target file and using dup2 to replace the standard fd), then calls **execvp** to overlay itself with the target program. The parent blocks on **waitpid** until the child terminates, collects the exit status, and returns to the prompt.

If the child exits with a non-zero status and stderr is not redirected, the shell prints the exit code. If fork itself fails, an error is printed and the loop continues.

All string arguments are converted to null-terminated C strings before the fork, since allocating after fork is unsafe in a multithreaded process (same principle as the terminal emulator's spawn_shell ceremony in [Section 2.2](#22-single-binary-self-re-exec-model)).

### 4.5 I/O Redirection Mechanism

Redirection is handled at two levels depending on whether the command is a builtin or an external.

For **builtins**, the REPL loop opens the target file (in truncate or append mode as specified) and wraps it in a generic writer. The builtin writes to that writer instead of stdout or stderr. This happens entirely in the parent process with no forking.

For **external commands**, redirection is applied in the child process after fork but before exec. The child opens the target file with the appropriate flags (O_WRONLY | O_CREAT, plus O_TRUNC or O_APPEND), calls dup2 to replace fd 1 (stdout) or fd 2 (stderr) with the opened file descriptor, then closes the original fd. When exec replaces the child's memory, the new program inherits the redirected file descriptors and writes to the file without knowing it.

Both paths support independent stdout and stderr redirection in the same command.

### 4.6 Tab Completion

The shell provides interactive tab completion whose behavior depends on cursor position. On the **first word** (command position), Tab searches the builtin names and every executable in PATH (filtered by execute permission, deduplicated, sorted). On **subsequent words** (argument position), Tab performs file and directory completion: it splits the current word at the last path separator, substitutes ~ for the home directory if needed, scans the target directory, and suffixes directory matches with / so the user can keep completing deeper paths.

When exactly one candidate matches, the remaining characters are filled in automatically with a trailing space (or / for directories). When multiple candidates share a common prefix longer than what is already typed, a single Tab extends to that prefix and stops. A second Tab press lists all candidates, reprints the prompt, and waits for further input. If nothing matches, the terminal bell sounds.

---

## 5. POSIX Compliance & Syscall Interface

RushX interfaces with the Linux kernel through two crates: **nix** (0.26) for safe Rust wrappers around POSIX syscalls, and **libc** (0.2) for raw FFI where no safe wrapper exists.

The syscalls used, grouped by subsystem:

**PTY allocation and session setup** (rushx_term): openpty to create the master/slave pair, fork to split into parent and child, setsid to create a new session, ioctl with TIOCSCTTY to assign the controlling terminal (raw libc, no nix wrapper), dup2 to wire fds 0/1/2 to the slave, and execvp to re-exec the binary in shell mode.

**Process lifecycle** (rushx_shell): fork to create a child for each external command, execvp to overlay the child with the target program, waitpid to block until the child exits and collect its status, and open + dup2 + close for fd redirection in the child before exec.

**I/O** (rushx_term): read on the master fd in the reader thread, write on the master fd from the keyboard handler. Both go through nix wrappers.

No signals are currently handled explicitly. The kernel's default SIGCHLD behavior (via waitpid) and the PTY line discipline's signal generation (Ctrl-C sends SIGINT to the foreground process group) are relied upon, but the shell does not install custom signal handlers. This is a known gap.

The shell's quoting rules (single quotes literal, double quotes with limited escapes, unquoted backslash) follow POSIX shell grammar conventions, though the implementation is a hand-written character walker rather than a formal grammar parser.

---

## 6. Project Status & Roadmap

### 6.1 Implementation Status Matrix

| Component                                             | Status      | Notes                                                           |
| ----------------------------------------------------- | ----------- | --------------------------------------------------------------- |
| PTY allocation (openpty)                              | Done        | Via nix crate                                                   |
| Session establishment (fork/setsid/ioctl/dup2/execvp) | Done        | Full POSIX ceremony                                             |
| Self-re-exec launcher                                 | Done        | /proc/self/exe + --rushx-shell                                  |
| GTK4 window + DrawingArea                             | Done        | Cairo toy text API rendering                                    |
| Reader thread + mpsc channel                          | Done        | 4096-byte buffer, blocking read                                 |
| Poll timer (16 ms)                                    | Done        | Drains channel, triggers redraw                                 |
| PTY output state machine                              | Done        | Full ANSI parser with SGR, ED, BEL (see Section 3.5)           |
| ANSI color rendering                                  | Done        | 16-color palette + 256-color + 24-bit RGB, per-segment styling  |
| Bell support (BEL 0x07)                               | Done        | Detected by parser, played via GDK system beep                  |
| Cursor rendering + blink                              | Done        | Block cursor, 500 ms blink                                      |
| Keyboard input + ANSI escapes                         | Done        | Ctrl, arrows, Delete, Home/End, UTF-8                           |
| REPL loop                                             | Done        | Raw mode input, user@hostname:cwd prompt, tab completion        |
| Argument tokenizer                                    | Done        | POSIX-style single/double quote handling                        |
| Redirection parser                                    | Done        | >, >>, 1>, 1>>, 2>, 2>>                                         |
| Builtin commands                                      | Done        | exit, echo (-e/-n), type, pwd, cd                               |
| Tab completion                                        | Done        | Command names (builtins + PATH) and file/directory paths        |
| Raw terminal mode                                     | Done        | Disables ICANON/ECHO/ISIG/ICRNL, restores before exec          |
| External command execution                            | Done        | fork/execvp/waitpid                                             |
| PATH resolution                                       | Done        | Search + execute-bit check                                      |
| Lexer (token stream)                                  | Stub        | lexer.rs exists but is empty                                    |
| AST                                                   | Stub        | ast.rs exists but is empty                                      |
| Variable/tilde expansion                              | Stub        | expand/vars.rs empty                                            |
| Glob expansion                                        | Stub        | expand/glob.rs empty                                            |
| Pipeline ( \| )                                       | Not started |                                                                 |
| Input redirection ( < )                               | Not started |                                                                 |
| Job control (bg/fg/jobs)                              | Not started |                                                                 |
| Signal handling (SIGINT/SIGTSTP)                      | Not started | Relies on kernel defaults                                       |
| Scrollback buffer                                     | Not started | Only visible lines are kept                                     |
| Cursor movement (CUP, CUU, etc.)                     | Not started | CSI cursor sequences are discarded                              |
| Termios configuration                                 | Not started | PTY uses kernel defaults                                        |
| Runtime config file                                   | Not started | All values are compile-time consts                              |

### 6.2 Known Limitations

- **No cursor movement commands.** CSI sequences for cursor positioning (CUP, CUU, CUD, CUF, CUB) are discarded. ANSI colors and text attributes are fully supported, but programs that rely on cursor addressing (vim, htop, less) will not render correctly.
- **No scrollback.** The text buffer grows unbounded in memory but there is no scroll mechanism; only the last N lines that fit the window are visible.
- **No signal handling.** The shell does not install handlers for SIGINT, SIGTSTP, or SIGCHLD. Ctrl-C and Ctrl-Z work through the kernel's line discipline, but the shell cannot trap or forward signals to job groups.
- **No pipelines or input redirection.** Only output redirection (> and >>) is implemented. Pipes and < are not parsed.
- **No variable or glob expansion.** $VAR, ~, and wildcard patterns are passed through as literal strings.
- **Child process safety.** The child path after fork uses .expect() in several places (dup2, open, execvp). A panic in the child would run Rust unwinding in a forked address space, which is undefined behavior. These should be replaced with raw \_exit(1) on failure.
- **Zombie risk.** External commands are reaped via waitpid in the parent, but if the shell is interrupted between fork and waitpid, the child becomes a zombie. No SIGCHLD handler exists to catch this.
- **Linux only.** The binary depends on /proc/self/exe, TIOCSCTTY, and glibc's openpty. It will not compile or run on macOS, BSDs, or Windows.

### 6.3 Roadmap

Near-term priorities:

1. Implement the lexer (proper token stream with position tracking) to replace the hand-written character walker.
2. Build an AST layer so the parser can represent pipelines, conditionals, and compound commands.
3. Add pipeline support ( | ) with pipe(2) between child processes.
4. Implement variable expansion ($VAR, $?, $$) and tilde expansion.
5. Install signal handlers for SIGINT and SIGTSTP in the shell, and forward them correctly to foreground process groups.

Longer-term goals:

- Glob expansion (\*, ?, [abc]).
- Input redirection ( < ) and here-documents ( << ).
- Job control (bg, fg, jobs, Ctrl-Z suspend/resume).
- Scrollback buffer with keyboard scrolling.
- Runtime configuration file for colors, font, keybindings.
- Persistent command history (up/down arrow recall across sessions).

---

## 7. Installation via APT

```bash
curl -fsSL https://raw.githubusercontent.com/The-HaiKaw-Pr0tocol/rushx/main/install.sh | sudo bash

sudo apt-get install -y rushx
```

## 8. References

1. Cefboud. _Exploring Terminals, TTYs, and PTYs_. [https://cefboud.com/posts/terminals-pty-tty-pyte/](https://cefboud.com/posts/terminals-pty-tty-pyte/)

2. _UNIX Like Shells_ (video). [https://www.youtube.com/watch?v=ubt-UjcQUYg](https://www.youtube.com/watch?v=ubt-UjcQUYg)

3. Funinkina. _Terminal Emulators Under the Hood_. [https://funinkina.is-a.dev/blog/terminal-emulators-under-the-hood](https://funinkina.is-a.dev/blog/terminal-emulators-under-the-hood)
