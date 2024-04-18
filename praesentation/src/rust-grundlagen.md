# Minimale Rust Grundlagen


```rust,editable
fn main() {
    // Immutable Variable mit expliziter Typangabe
    let _logical: bool = true; // Variablen sind standardmäßig immutable

    // Mutable Variable - nur mutable Variablen können geändert werden
    let mut mutable: i32 = 12; // Mutable `i32`

    // Call by Reference: Übergeben der Variable `mutable` an eine Funktion
    modify_value(&mut mutable); // die Funktion benötigt einen mutable reference, um den Wert zu ändern

    // Ausgabe des veränderten Wertes
    println!("Modified value: {}", mutable);

    show_value(&mutable);
    //demo_for_loop();
}

// Funktion, die einen mutable reference auf eine i32 annimmt und den Wert ändert
fn modify_value(value: &mut i32) {
    *value += 9; // Dereferenzierung und Änderung des Wertes
}

// Funktion, die einen reference verarbeitet
fn show_value(value: &i32) {
    // Ausgabe des referenzieren Wertes
    // Dereferenzierungsoperator * ist nicht notwendig
    println!("Show value: {}", *value);
}

fn demo_for_loop() {
    for i in 0..3 {
        println!("Simple delay with for_loop: {i}");
    }
}
```

Obiger Code nutzt <https://play.rust-lang.org/> zum Ausführen.

## Links

[Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/) zum Nachschlagen

[Rust Book](https://doc.rust-lang.org/book/) zum Lernen

[Rustlings](https://rustlings.cool/) zum Üben




