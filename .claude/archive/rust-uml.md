Here is the Rust-to-UML Structural Mapping formatted as a markdown "Skill Reference." You can add this to your AI agent's system prompt or knowledge base to ensure it generates consistent, high-quality diagrams for your Rust projects.
------------------------------
🛠 Skill: Rust-to-UML Structural Mapping (Mermaid)
Objective: Translate Rust source code (Structs, Traits, Enums, and Impls) into standards-compliant UML Class Diagrams using Mermaid.js.
1. Visibility & Access Modifiers
Map Rust visibility keywords to UML symbols to represent encapsulation within modules and crates.

| Rust Keyword | UML Symbol | Description |
|---|---|---|
| pub | + | Public: Accessible outside the module/crate. |
| (none) | - | Private: Module-private (default). |
| pub(crate) | ~ | Package: Visible only within the current crate. |
| pub(super) | # | Protected: Visible to the parent module. |

2. Component Archetypes
Use Mermaid stereotypes (<<...>>) to distinguish between Rust's different data structures.

* Structs: class Name { <<struct>> }
* Traits: class Name { <<interface>> }
* Enums: class Name { <<enumeration>> }

3. Method & Member Syntax

* Instance Fields: [Symbol][Type] [Name] (e.g., -u32 id)
* Instance Methods: [Symbol][Name]([Args]) [Return] (e.g., +get_data(u32) String)
* Static Methods (Associated Functions): Append $ to the method name (e.g., +new()$ Self).
* Generics: Use the tilde syntax for brackets: class Container~T~.

4. Relationship Logic (Arrows)
Rust does not have inheritance. Relationships must reflect Ownership, References, and Implementations.

| Relationship Type | Rust Concept | Mermaid Syntax |
|---|---|---|
| Realization | impl Trait for Struct | `Struct .. |
| Composition | Owned field (self.x = X) | Parent *-- Child |
| Aggregation | Reference (&T, Box<T>, Rc<T>) | Parent o-- Child |
| Dependency | Usage in function params | A --> B |

------------------------------
5. Implementation Reference TemplateSource Code Input:

pub trait Drawable { fn draw(&self); }
pub enum Color { Red, Blue }
pub struct Window {
    pub title: String,
    background: Color, // Owned
    border_width: u32,
}
impl Drawable for Window {
    fn draw(&self) { /* ... */ }
}

Generated Mermaid Code:

classDiagram
    class Drawable {
        <<interface>>
        +draw() void
    }

    class Color {
        <<enumeration>>
        Red
        Blue
    }

    class Window {
        <<struct>>
        +String title
        -Color background
        -u32 border_width
        +draw() void
    }

    Window ..|> Drawable : implements
    Window *-- Color : owns

------------------------------
6. Edge Cases: Enums with Data
For Rust Enums with associated data (e.g., Variant(u32)), represent the variant as a nested member or a linked note to indicate it is a "Sum Type" rather than a simple flag.
------------------------------
Should I provide a Python script that can parse basic Rust structs and automatically output this Mermaid markdown?


