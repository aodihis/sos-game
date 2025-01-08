# SOS Game

A simple SOS game built using [Yew](https://yew.rs/) and Rust.
I used to play this game with my friends or family when I was a kid.  
I built this to be nostalgic and to remind me of my childhood.
Despite it is not as much fun as playing using paper and pen with a loved one, but you know...

This project demonstrates the power of Rust for building frontend applications with interactive game logic.

![SOS Game Demo](https://raw.githubusercontent.com/aodihis/sos-game/refs/heads/main/assets/images/show.gif)

---

## 🚀 Getting Started

### Prerequisites
- Install **Rust**: [Get Started with Rust](https://www.rust-lang.org/tools/install)
- Install **Trunk** (for serving the Yew application):
  ```bash
  cargo install trunk
  ```

### Run the Project
1. Clone this repository:
   ```bash
   git clone <repo-url>
   cd sos-game
   ```
2. Serve the project using Trunk:
   ```bash
   trunk serve
   ```
3. Open the application in your browser at [http://localhost:8000](http://localhost:8080).

---

## 📂 Project Structure

```
sos-game/
├── Cargo.lock         # Dependency lockfile
├── Cargo.toml         # Rust project configuration
├── index.html         # Main HTML file
├── Trunk.toml         # Trunk configuration
├── assets/            # Static assets
│   └── main.css       # Main CSS file
└── src/               # Source code
    ├── main.rs        # Main Rust entry point
    ├── components/    # Frontend components
    │   ├── board.rs
    │   ├── cell.rs
    │   ├── constants.rs
    │   └── state.rs
    └── engine/        # Game logic
        ├── bot.rs
        ├── cell.rs
        └── game.rs
```

---

## ⚙️ Customization

To modify the size of the game board:
1. Open `src/main.rs`.
2. Update the `col` and `row` variables to your desired dimensions.

---

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/) - Programming language
- [Yew](https://yew.rs/) - Rust framework for building front-end web apps
- [Trunk](https://trunkrs.dev/) - Build and bundling tool for Rust web apps

---