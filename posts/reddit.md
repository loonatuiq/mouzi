# Reddit Post Drafts

## Option A: r/selfhosted

---

[OC] Mouzi – a silent, offline file organizer that sorts your Downloads automatically (Windows)

Hey r/selfhosted,

I got tired of my Downloads folder turning into chaos, so I built Mouzi – a tiny tray app that watches folders and moves files based on rules you define. Everything stays local.

**What it does:**
- Monitors folders (Downloads by default) and auto-sorts new files after a 2-second delay
- Rules by extension, regex, or custom destination paths
- One-click undo for any action
- Full history stored locally in SQLite
- Autostart with Windows

**Why I built it:**
Everything similar I found was either cloud-based, subscriptionware, or Electron bloat. I wanted something that uses minimal RAM, makes zero network calls, and just works without an account.

**Tech stack:**
- Rust backend (file watcher via `notify`, SQLite, rule engine)
- Tauri 2.x frontend (React 19 + Tailwind)
- NSIS/MSI installers

**Links:**
- GitHub: https://github.com/hsr88/mouzi
- Download: https://mouzi.cc

MIT licensed, free forever. Would love feedback from the self-hosted crowd.

EDIT: Windows 10/11 only for now. macOS/Linux maybe later if there's interest.

---

## Option B: r/rust

---

[Showcase] Mouzi – a system tray file organizer built with Rust + Tauri

Hi r/rust,

I built Mouzi, a file organizer that sits in your system tray and auto-sorts Downloads by type. The backend is pure Rust and I would love feedback on the architecture.

**Rust parts:**
- `notify` crate for filesystem watching with debounced events
- `rusqlite` for local action logging and undo history
- Regex-based rule engine with dynamic path resolution (`{year}`, `{month}`, etc.)
- Tray integration and multi-window management via Tauri

**Repo:** https://github.com/hsr88/mouzi

The code is not perfect – I am sure there are Rust idioms I missed. If anyone wants to review the watcher or rule engine logic, I would genuinely appreciate it.

---

## Option C: r/Windows10 / r/pcmasterrace

---

Mouzi – auto-organize your Downloads folder without cloud BS

I made a tiny app that sorts your Downloads automatically. No OneDrive, no Adobe Creative Cloud, no accounts – it just watches your folder and moves files where they belong.

- Sorts images, docs, archives, installers, music, video
- Undo any move if it gets it wrong
- Dark mode, 5 languages, starts with Windows
- 3 MB installer, uses basically no RAM

Free, open source: https://mouzi.cc
GitHub: https://github.com/hsr88/mouzi

If you try it, let me know what breaks.

---

**Universal Reddit tips:**
- Read the subreddit rules before posting (some ban self-promo on specific days)
- Post during US peak hours (9am–12pm ET) for max visibility
- Reply to comments quickly – early engagement pushes the post up
- If it gets downvoted early, delete and repost in 24h with a better title
- Never use link shorteners – Reddit filters them as spam
