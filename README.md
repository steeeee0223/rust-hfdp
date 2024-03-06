# Head First Design Pattern

This repo contains a walk through of the book [Head First Design Pattern](https://github.com/bethrobson/Head-First-Design-Patterns) implemented in Rust.

## Patterns

### Behaviorial Patterns

-   [x] Strategy Patterns (cf. `HFDP Ch1`)
-   [x] Observer Patterns (cf. `HFDP Ch2`)
-   [x] Command Patterns (cf. `HFDP Ch6`)
    -   Command 1: Simple Remote Control
    -   Command 2: Remote Control (with slots of commands & undo)
    -   Command 3: Remote Control (with undo)
    -   Command 4: Macro Command - Party Mode Remote Control
-   [x] Template Method Patterns (cf. `HFDP Ch 7`)
    -   Template 1: Beverages (with hook)
    -   Template 2: Sorting Ducks (implementing built-in traits)
-   [x] State Patterns (cf. `HFDP Ch 10`)
    -   State 1: Original Gumball Machine
    -   State 2: Gumball Machine (with random winning state)
        -   This solution refers to [Rust Lang Book §17.3](https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html)
    -   State 3: Password manager (in compile time solution)
        -   This solution refers to [YouTube](https://www.youtube.com/watch?v=_ccDqRTx-JU)
        -   This solution is unappliable to Gumball Machine

### Structural Patterns

-   [x] Decorator Patterns (cf. `HFDP Ch3`)

### Creational Patterns

-   [x] Factory Patterns (cf. `HFDP Ch4`)
    -   Factory 1: Simple Factory
    -   Factory 2: Factory Pattern
    -   Factory 3: Abstract Factory
-   [x] Singleton Patterns (cf. `HFDP Ch5`)
    -   Singleton 1: Chocolate boiler
    -   Singleton 2: Singleton as generic macro
