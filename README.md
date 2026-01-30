# RushX

### Overview

RushX (Rust Shell - eXtended) is a POSIX-compliant *NIX shell and terminal implemented in Rust, focusing on low-level control over process creation, PTY management, job control, and terminal I/O. Rather than delegating behavior to external helpers, RushX directly interfaces with the operating system to manage sessions, process groups, signals, and controlling terminals. This approach enables precise control over foreground and background jobs while maintaining strong safety guarantees through Rust.

RushX is architected as a modular shell runtime, with clearly defined stages for lexical analysis, parsing, expansion, and execution. The execution engine is designed to handle pipelines, redirections, and builtins while maintaining POSIX semantics. Terminal handling is tightly integrated with the job-control layer, allowing RushX to function as both an interactive shell and a terminal driver.

> Developed & Maintained by [The HaiKaw Pr0tocol](https://github.com/The-HaiKaw-Pr0tocol) organization.

## RushX's Logo
<div align="center">
    <img alt="RushX's Logo" src="https://github.com/user-attachments/assets/502b94f8-c209-4fc8-af59-ff6e952603ca" width="1000"/>
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