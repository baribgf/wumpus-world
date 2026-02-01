# The `Wumpus World` Game

A Rust implementation of the classic Wumpus World AI problem, featuring both interactive player-controlled and autonomous knowledge-based agent gameplay.

## Overview

Wumpus World is a grid-based exploration game where an agent navigates a cave system to find gold while avoiding dangers like pits and the Wumpus (a dangerous creature). This project implements both a playable game mode and an intelligent AI agent that uses knowledge-based reasoning to make decisions.

## Features

- **Player Mode**: Manually control an agent as it explores the cave
- **Agent Mode**: Watch an AI-driven agent automatically navigate using knowledge-based reasoning
- **Interactive TUI**: Terminal-based user interface for game interaction
- **Knowledge Base System**: Logic-based reasoning for agent decision-making
- **Score Tracking**: Track performance with penalties for moves and hazards, rewards for gold

## Getting Started

### Prerequisites
- Rust 1.70 or later
- Cargo

### Installation & Running

1. Clone the repository
2. Build and run the project:
   ```bash
   cargo run
   ```

## Project Structure

- **`src/main.rs`**: Entry point and game loop
- **`src/agent.rs`**: Core agent types and actions
- **`src/agents.rs`**: Knowledge-based agent implementation
- **`src/env.rs`**: Game environment and rules
- **`src/grid.rs`**: Grid and position utilities
- **`src/kb.rs`**: Knowledge base data structures
- **`src/logic.rs`**: Logical reasoning engine
- **`src/room.rs`**: Room and object types
- **`src/tui.rs`**: Terminal user interface

## How the AI Agent Works

The knowledge-based agent maintains a knowledge base of facts about the world, including:
- Safe and unsafe locations
- Wumpus and pit potential positions
- Locations with sensory cues

Based on observations, the agent uses logical reasoning to infer new facts and make decisions about which direction to move or whether to shoot an arrow.
