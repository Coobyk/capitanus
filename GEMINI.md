# Capitanus Project Vision

This document summarizes the key decisions and vision for the `capitanus` project.

## 1. Core Project Details

- **Project Name:** `capitanus`
- **Core Idea:** A script manager with a beautiful, organized, and minimalistic UI.
- **Target Audience:** Developers and regular users.
- **Primary Goal (MVP):** A dashboard to view all scripts and trigger them.
- **Monetization:** Open-source and free, with an option for users to sponsor.
- **Success Metric:** Not a primary concern.
- **Tech Stack:** Rust, Linux-only for now.
- **User's Rust Knowledge:** Beginner.

## 2. User Interface (UI)

- **Type:** GUI initially. A TUI is a possibility for the future.
- **Technology:** `iced` (Chosen for its data-centric approach and Rust-native design).
- **Layout:** A grid of scripts, with a toggle for a list view.
- **Visuals:**
    - Dark mode only for MVP.
    - Background blur and colors are desired.
    - Scripts will have icons.
    - Color-coded status indicators:
        - **Idle:** Blue (fades from green over 5 mins after a successful run).
        - **Running:** Yellow.
        - **Success:** Green.
        - **Fail:** Red.
- **Features:**
    - Search bar.
    - Organization via tags.
    - Customizable sorting and grouping.

## 3. Script Management

- **Supported Languages (MVP):** Bash.
- **Execution Command:** Default is `./script`, but will be customizable per script.
- **Storage:** Scripts will be copied to a central directory (e.g., `~/.config/capitanus`).
- **Adding Scripts (MVP):** Import from a local file.
- **Organization:** Tags.
- **Metadata:** Name, Description, Author, Version, Run statistics.
- **Dependencies:** Not handled in MVP.
- **Versioning:** Will be supported.

## 4. Script Execution

- **Process:** Scripts run in their own separate process.
- **Sandboxing:** No sandboxing.
- **Permissions:** User-defined.
- **Arguments:** Scripts are expected to be self-contained without arguments for manual runs.
- **Secrets:** To be stored securely (details TBD).
- **Long-running Scripts:** A notification will be sent if a script runs 3x longer than its average.
- **Concurrency:** A key feature; multiple scripts can run at once.
- **Success/Failure:** Determined by exit code (`0` = success). Keywords in logs can also indicate failure.
- **Control:** Scripts can be stopped.

## 5. Statistics & History

- **Database:** SQLite.
- **Tracked Stats:** Run count, last run time, average duration, success/failure rate, etc.
- **History:** Stored forever by default (user can clear it).
- **Output Logs:** Past run outputs will be viewable.

## 6. Automation

- **Triggers:** Time-based (cron, intervals, etc.), filesystem changes, and on startup.
- **Sequences:** Scripts can be chained together to run in sequence.
- **Configuration:** Via context menus and a visual workflow builder.
- **Deactivation:** Scripts can be deactivated (will appear grayed out).
- **Notifications:** System notifications for automated runs (with a quiet mode).

## 7. Autostart

- **Configuration:** Per-script.
- **Platform:** Linux only for now.
- **Behavior:** The UI can be configured to start on login, or just a background agent.
- **Smooth Start:** An option to run startup scripts sequentially to avoid system overload.

## 8. Marketplace (Future Vision)

- **Platform:** A dedicated GitHub repository (`capitanus-marketplace`).
- **Cost:** Free, with an option to sponsor authors.
- **Licensing:** All scripts will be MIT licensed.
- **Review Process:** Manual review + automated AI review in PRs.
- **Discovery:** In-app search menu.
- **Updates:** The client will sync with the git repo to get script updates.

## 9. Development Process

- I want to learn Rust with this project so don't directly write any code unless I specifically tell you to do so, and just teach me how to write it instead.