# 🚀 Velo: The Esoteric Programming Language of Cosmic Velocity

Velo is a **Turing-complete** Esoteric programming language (Esolang) where program execution is modeled as a **Vessel** navigating a **Cosmos**. The program's data and control flow are entirely determined by the Vessel's **Velocity** and **Entropy Level**.

The name Velo emphasizes **Velocity**, which serves a dual purpose:

1.  **Physical Speed**: The actual rate of acceleration/deceleration.
2.  **Program Pointer**: The value of Velocity acts as the **Cosmic Resonance Frequency**, indexing the data memory.

## 🌌 Velo Cosmology and Execution

### The Cosmos (The Code)

Velo code is a 2D grid of Runes. Execution begins at the **top-left corner** (0, 0). Lines of code can contain comments starting with the `#` symbol.

### The Vessel (Program State)

The core state is stored within the Vessel:

- **Velocity** (`usize`): The data pointer/frequency. The program halts if this value reaches 0.
- **Data Lattice** (`Vec<u32>`): The expandable memory structure (Data Cells).
- **Entropy Level** (`u32`): The value of the data cell currently pointed to by the Velocity.

### Execution

The Vessel moves one unit per cycle based on its current direction. Program logic is executed when the Vessel impacts a Rune, modifying its Velocity, Direction, or the Entropy Level.

## 🔠 Rune Set (Instructions)

Runes are grouped by their primary effect:

| Rune   | Symbol | Name               | Function                                                                                                                |
| :----- | :----- | :----------------- | :---------------------------------------------------------------------------------------------------------------------- |
| `^v<>` | `^v<>` | **Thrust Runes**   | Modifies Velocity (+1, -1, or no change) and/or Direction based on the alignment of the Rune and the current direction. |
| `P`    | `P`    | **Parking**        | Resets Velocity (Pointer) to 1.                                                                                         |
| `+`    | `+`    | **Entropy Charge** | Increases the current cell's Entropy Level by 1.                                                                        |
| `-`    | `-`    | **Entropy Drain**  | Decreases the current cell's Entropy Level by 1 (only if $\ge 1$).                                                      |
| `[`    | `[`    | **Steer Left**     | **Conditional Loop:** If Entropy Level $\neq 0$, forces a 90° left turn, continuing the loop.                           |
| `]`    | `]`    | **Steer Right**    | **Conditional Loop:** If Entropy Level $\neq 0$, forces a 90° right turn, redirecting the Vessel.                       |
| `,`    | `,`    | **Input**          | Reads a byte from stdin and stores its value (ASCII code) in the current cell.                                          |
| `.`    | `.`    | **Output**         | Prints the current cell's Entropy Level as an ASCII character.                                                          |
| `D`    | `D`    | **Debug**          | Prints the full state of the Vessel when debugging/tracing is enabled.                                                  |

## ⚙️ Command Line Interface

Velo supports standard execution and powerful debugging flags:

| Flag            | Name        | Function                                                                                |
| :-------------- | :---------- | :-------------------------------------------------------------------------------------- |
| `-d`, `--debug` | Debug Mode  | Prints Vessel state only when a `D` (Debug) Rune is encountered.                        |
| `-t`, `--trace` | Trace Mode  | Prints Vessel state at every execution step. Overrides `--debug`.                       |
| `--ignore-void` | Ignore Void | Used with `--trace`, prevents printing the state when the Vessel impacts a `Void` Rune. |

## 🛑 Termination

The Velo program halts if:

1.  The **Velocity** (Pointer) reaches **0**.
2.  The Vessel attempts to travel **out of the Cosmos boundaries** (NoSignal).
3.  The Vessel starts on a Rune that is **not a Thrust Rune** (NoInitialVelocityOrDirection).
