# ShellStash (st)

ShellStash (`st`) is a small CLI tool that lets you save and run frequently used shell commands so you don’t have to memorize them or repeatedly retype them.

---

## Features

- Save shell commands by name
- Run them later with a short alias
- Supports pipes (`|`), chaining (`&&`), redirects, etc.
- Commands are stored locally (per user)
- Quick and easy to type

---

## Installation

### Option 1: One-line install (Linux / macOS) (No Rust required)
```bash
curl -fsSL https://raw.githubusercontent.com/AlexAxenti/ShellStash/main/install.sh | sh
```

Consider saving the installer itself into ShellStash for future updates!
```bash
st s update-st "curl -fsSL https://raw.githubusercontent.com/AlexAxenti/ShellStash/main/install.sh | sh"
```

### Option 2: Install with Cargo (Rust required)

Remotely from GitHub repo:
```bash
cargo install --git https://github.com/AlexAxenti/ShellStash --locked
```

Or after cloning locally:
```bash
cargo install --path . --force
```

### Option 3: Download a prebuilt binary (no Rust required)
1. Go to [Releases](https://github.com/AlexAxenti/ShellStash/releases) on GitHub
2. Download the binary for your OS:
    - st-windows-x86_64.exe
    - st-linux-x86_64
    - st-macos-x86_64
3. Put it somewhere already on your `PATH`

#### Linux/macOS
```bash
chmod +x st-<os>-x86_64
sudo mv st-<os>-x86_64 /usr/local/bin/st
```

#### Windows
1. Rename to `st.exe`
2. Move it to a folder on PATH

Test installation with:
```bash
st --version
```

---

## Usage:

### Save a command

```bash
st s up -- docker compose up -d

# Or use quotations, required with shell operators:
st s update "sudo apt update && sudo apt upgrade" 
```

### Run a command

```bash
st r up
```

### List all commands

```bash
st ls

# Print both saved name and command:
st ls -a
```

### Show a command

```bash
st sh up
```

### Remove a command

```bash
st rm up
```

## Shell operators (important)

If your command uses shell operators like:
- pipes (|)
- chaining (&&, &)
- redirects (>, >>)
- parentheses (())

wrap the entire command in quotes, otherwise your shell will interpret it before st can save it: 
```bash
st s example "(echo one && echo two) | grep two"
```
