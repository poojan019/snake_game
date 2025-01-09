# Snake Game

This repository contains a simple Snake game implemented in Rust using the [Piston game engine](https://github.com/PistonDevelopers/piston).

## Features

- Classic snake gameplay.
- Randomly generated food.
- Game over screen and restart functionality.
- Configurable game board dimensions.

## Getting Started

### Prerequisites

To build and run this project, you need:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Cargo (comes bundled with Rust)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/poojan019/snake_game.git
   cd snake_game
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the game:

   ```bash
   cargo run
   ```

## Gameplay

- Use the **Arrow Keys** to control the snake.
- Avoid hitting the walls or the snake's tail.
- Eat the food to grow the snake and increase your score.
- The game restarts automatically after a delay when you lose.

## Project Structure

- `src/main.rs`: The entry point of the game.
- `src/game.rs`: Contains the main game logic.
- `src/snake.rs`: Implements the snake's behavior and movement.
- `src/draw.rs`: Handles drawing the game elements on the screen.

## Customization

- You can change the board dimensions by modifying the `width` and `height` variables in `src/main.rs`:
  ```rust
  let (width, height) = (30, 30);
  ```

- Adjust gameplay parameters, such as speed, in `src/game.rs` by modifying constants like `MOVING_PERIOD`.

## Dependencies

This project uses the following crates:

- [piston_window](https://crates.io/crates/piston_window): For rendering and game window management.
- [rand](https://crates.io/crates/rand): For generating random food positions.

## Contributing

Contributions are welcome! If you have ideas for improvements or find any issues, feel free to open an issue or submit a pull request.

## Acknowledgments

- [Piston Developers](https://github.com/PistonDevelopers) for the amazing game engine.
- The Rust community for providing excellent learning resources.

---

Enjoy the game!
