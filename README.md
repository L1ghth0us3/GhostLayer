# GhostLayer
### _A tool written in Rust to detect any installed Kernel Level Anti Cheat software on a Windows PC._

# 👻 GhostLayer

GhostLayer is a privacy-first, open-source tool for scanning your system for traces of kernel-level anti-cheat software — like Riot Vanguard, XIGNCODE3, BattlEye, and others.

It's built in Rust with a focus on transparency, user control, and eventually full removal capabilities. Think of it as a watchdog that alerts you to what’s digging deep into your operating system, and gives you the power to deal with it.

---

## 🧠 Why I Made This

Kernel-level anti-cheats have become increasingly common in games — often running with the highest system privileges, installing services and drivers that persist beyond gameplay. Many users aren’t even aware of what gets installed, let alone how to remove it cleanly.

GhostLayer was born out of that frustration. I wanted to clear up and be direct with knowing exactly which kernel anti-cheats are on my system, understand where they live, how they work, and choosing whether I want to remove them or not.

---

## ✅ Currently Implemented

- Detects known anti-cheat `.sys` drivers by filename
- Scans the Windows registry for anti-cheat driver **services**
- Displays clear removal instructions for each detected AC

---

## 🛠️ Coming Soon

These are the next features I plan to implement:

- Scan running system **services and processes**
- Export detection results to JSON or text report
- Add a basic GUI
- Create/Load anti-cheat database from JSON for community contributions

---

## 🎯 End Goal

The final vision for GhostLayer is a powerful, user-friendly tool that gives you full visibility into invasive anti-cheat software.

I want GhostLayer to be:
- Easy enough for any user to run
- Transparent enough to be audited and trusted
- Respectful of users; never sharing information, never removing anything without consent

---

If you care about what runs on your machine, GhostLayer is for you.

Stay tuned ♥
