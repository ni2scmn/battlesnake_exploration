# Battlesnake Exploration

This Repo is a **work in progress** aimed at exploring strategies and techniques for the [Battlesnake](https://docs.battlesnake.com/) challenge. The project leverages Rust for the main API and game logic—with support from Python and shell scripts for additional tooling and simulation.

## Overview

Battlesnake is a programming challenge where developers build their own snake AI to compete against others in real-time games. This project explores various aspects of the game, including:

- **Web Server/API**:\
  Developed using Rust and the Rocket framework to handle Battlesnake endpoints.

- **Game Logic & Strategy**:\
  Experimenting with different strategies and decision-making algorithms.

- **Simulation & Analysis**:\
  Utilizing Python and shell scripts for testing, simulation, and logging game data.

## Project Structure

```text
battlesnake_exploration/
├── .cargo/             # Cargo configuration files
├── logs/               # Log files generated during execution
├── scripts/            # Python and shell scripts for utilities
│   ├── example.py
│   ├── utility.sh
│   └── ...
├── src/                # Rust source code
│   ├── main.rs         # Main entry point
│   ├── game_logic.rs   # Game strategy and logic
│   ├── api.rs          # API endpoints for Battlesnake
│   └── ...
├── .gitignore          # Git ignore rules
├── .replit             # Replit configuration (if applicable)
├── Cargo.lock          # Rust dependency lock file
├── Cargo.toml          # Rust package configuration
├── Rocket.toml         # Rocket framework configuration
└── README.md           # Project documentation
```

## Getting Started

### Prerequisites
- Rust (latest stable version) w. Cargo
- Python (3.11+)

## Installation
1. Clone the repository
   ```bash
   git clone https://github.com/ni2scmn/battlesnake_exploration.git
   cd battlesnake_exploration
   ```

2. Build the project
   ```bash
   cargo build
   ```

3. Run the server with a specified strategy (e.g., `random`, `simple`)
   ```bash
   cargo run [STRATEGY]
   ```

## Running Additional Scripts
Some Python or shell scripts in the `scripts/` folder may require extra dependencies. Check the individual script files for usage instructions and required libraries.

## Usage
Once the project is running, the Battlesnake API will be available on the port specified in `Rocket.toml`. Use your preferred tools (e.g., cURL, Postman, or a web browser) to interact with the endpoints, test strategies, and simulate game scenarios.


**_This project is a work in progress. Your feedback and contributions are appreciated as we continue to develop and improve the project._**