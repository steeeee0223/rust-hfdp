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
-   [x] Chain of Responsibility (cf. `HFDP A3`)
-   [ ] Interpreter Patterns (cf. `HFDP A5`)
-   [ ] Mediator Patterns (cf. `HFDP A6`)
-   [ ] Memento Patterns (cf. `HFDP A7`)
-   [ ] Visitor Patterns (cf. `HFDP A9`)

### Structural Patterns

-   [x] Decorator Patterns (cf. `HFDP Ch3`)
-   [x] Adapter Patterns (cf. `HFDP Ch7`)
    -   Adapter 1: Duck & Turkey
    -   Adapter 2: `TODO` Iterator & Enumeration
-   [x] Facade Patterns (cf. `HFDP Ch7`)
    -   Facade 1: Home Theater
-   [x] Iterator Patterns (cf. `HFDP Ch9`)
    -   Iterator 1: Menus (using `IntoIter` trait)
-   [x] Composite Patterns (cf. `HFDP Ch9`)
    -   Composite 1: Menus with Sub-Menus
    -   Composite 2: Menus with Sub-Menus (refactored with `enum`)
    -   Composite 3: `TODO` Vegetarian Menu (implemented `Iterator`)
-   [x] Proxy Patterns (cf. `HFDP Ch11`)
-   [ ] Bridge Patterns (cf. `HFDP A1`)
-   [ ] Flyweight Patterns (cf. `HFDP A4`)

### Creational Patterns

-   [x] Factory Patterns (cf. `HFDP Ch4`)
    -   Factory 1: Simple Factory
    -   Factory 2: Factory Pattern
    -   Factory 3: Abstract Factory
-   [x] Singleton Patterns (cf. `HFDP Ch5`)
    -   Singleton 1: Chocolate boiler
    -   Singleton 2: Singleton as generic macro
        -   This solution refers to [YouTube](https://www.youtube.com/watch?v=ULn2JbTpWIM)
-   [ ] Builder Patterns (cf. `HFDP A2`)
-   [ ] Prototype Patterns (cf. `HFDP A8`)
