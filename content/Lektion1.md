---
title: "Lektion 1"
date: 2023-09-08T08:28:53+02:00
draft: false
type: "page"
menu: 
  main:
    name: "Lektion 1"
    weight: 2
    
---
# Variablen und Konsole in Rust
## Erstes Programm

Um ein erstes Programm zu schreiben, muss eine Datei mit der Endung `.rs` erstellt werden. In dieser Datei kann dann der folgende Code geschrieben werden:
```rust
fn main() {
    println!("Hello World!");
}
```
Um das Programm zu kompilieren, muss folgender Befehl ausgeführt werden:
```bash
rustc main.rs
```
Dieser Befehl erzeugt eine ausführbare Datei mit dem Namen `main`. Diese kann dann mit folgendem Befehl ausgeführt werden:
```bash
./main
```
Dieser Befehl sollte `Hello World!` ausgeben.



## Übungen
```rust
fn main() {  // Programm startet hier
    println!("Hello, world!"); // Ausgabe von Text auf der  Konsole ausgeben Hello, world!
} // Programm endet hier
```

## formatiert text erstellen
```rust
    let a="Hallo"; // Variable a wird erstellt und mit dem Wert Hallo belegt
    let b=format!("{} Welt!",a); // Variable b wird erstellt und mit dem Wert Hallo Welt! belegt
    println!("{}",b); // Ausgabe von Text auf der  Konsole ausgeben Hallo Welt!
```
## Rust Text einlesen in Rust
```rust
fn main() {  // Programm startet hier
    // Konsole eingabe einlesen
    println!("Wie heißt du?"); // Text ausgeben
    let mut input = String::new(); // Leere Zeichenkette mit Namen input erstellen und veränderbar machen

    std::io::stdin().read_line(&mut input).unwrap(); // Zeichenkette von der Konsole einlesen und in input speichern
    // Eingabe ausgeben
    println!("Hallo {}!", input); // Eingabe ausgeben
}
```

## Was ist Zeichenkette in Rust
Eine Zeichenkette ist eine Folge von Zeichen. In Rust wird eine Zeichenkette mit dem Datentyp `String` dargestellt. Eine Zeichenkette kann mit folgendem Code erstellt werden:
```rust
let a = String::from("Hallo Welt!");
```

## Zeichenkette in Rust ausgeben

Eine Zeichenkette kann mit dem Befehl `println!` ausgegeben werden:
```rust
let a = String::from("Hallo Welt!");
println!("{}", a);
```

## Zeichenkette in Rust einlesen
Um eine Zeichenkette einzulesen, muss eine Variable vom Typ `String` erstellt werden. Diese kann dann mit dem Befehl `read_line` eingelesen werden:
```rust
let mut a = String::new();
std::io::stdin().read_line(&mut a).unwrap();
```
## Was ist unwrapped in Rust
Unwrap ist eine Funktion, die einen Wert aus einem `Result`-Objekt extrahiert. Wenn das `Result`-Objekt den Wert `Ok` enthält, wird der Wert zurückgegeben. Wenn das `Result`-Objekt den Wert `Err` enthält, wird das Programm mit einer Fehlermeldung beendet.

## Was ist ein Result-Objekt in Rust
Ein `Result`-Objekt ist ein Objekt, das entweder den Wert `Ok` oder den Wert `Err` enthält. Wenn das `Result`-Objekt den Wert `Ok` enthält, ist alles in Ordnung. Wenn das `Result`-Objekt den Wert `Err` enthält, ist etwas schief gelaufen.

## Was ist ein String::from
`String::from` ist eine Funktion, die eine Zeichenkette erstellt. Sie kann mit folgendem Code aufgerufen werden:
```rust
let a = String::from("Hallo Welt!");
```

## Was sind Ganzzahlen in Rust
Ganzzahlen sind Zahlen, die keine Nachkommastellen haben. In Rust werden Ganzzahlen mit dem Datentyp `i32` dargestellt. Eine Ganzzahl kann mit folgendem Code erstellt werden:
```rust
let a = 42;
```

## Ganzzahlen in Rust ausgeben
Eine Ganzzahl kann mit dem Befehl `println!` ausgegeben werden:
```rust
let a = 42;
println!("{}", a);
```
## Datentypen in Rust

Rust ist eine statisch typisierte Sprache. Das bedeutet, dass jede Variable einen festen Datentyp hat. Der Datentyp einer Variable kann mit dem Befehl `type_of` ermittelt werden:
```rust
let a = 42;
println!("{}", std::any::type_name_of_val(&a));
```
### Ganzzahlen
```rust
let a: i8 =0; // 8-Bit Ganzzahl mit Vorzeichen,variable a wird erstellt und mit dem Wert 0 belegt
let a: i16 =0; // 16-Bit Ganzzahl mit Vorzeichen,variable a wird erstellt und mit dem Wert 0 belegt
let a: i32 =0; // 32-Bit Ganzzahl mit Vorzeichen,variable a wird erstellt und mit dem Wert 0 belegt
let a: i64 =0; // 64-Bit Ganzzahl mit Vorzeichen,variable a wird erstellt und mit dem Wert 0 belegt
let a: i128 =0; // 128-Bit Ganzzahl mit Vorzeichen,variable a wird erstellt und mit dem Wert 0 belegt

let a: u8 =0; // 8-Bit Ganzzahl ohne Vorzeichen,variable a wird erstellt und mit dem Wert 0 belegt
let a: u16 =0; // 16-Bit Ganzzahl ohne Vorzeichen,variable a wird erstellt und mit dem Wert 0 belegt
let a: u32 =0; // 32-Bit Ganzzahl ohne Vorzeichen,variable a wird erstellt und mit dem Wert 0 belegt
let a: u64 =0; // 64-Bit Ganzzahl ohne Vorzeichen,variable a wird erstellt und mit dem Wert 0 belegt
let a: u128 =0; // 128-Bit Ganzzahl ohne Vorzeichen,variable a wird erstellt und mit dem Wert 0 belegt
```
### Gleitkommazahlen
```rust
let a: f32 =0.0; // 32-Bit Gleitkommazahl,variable a wird erstellt und mit dem Wert 0.0 belegt
let a: f64 =0.0; // 64-Bit Gleitkommazahl,variable a wird erstellt und mit dem Wert 0.0 belegt
```
### Boolesche Werte
```rust
let a: bool =true; // Boolescher Wert,variable a wird erstellt und mit dem Wert true belegt
```
### Zeichen
```rust
let a: char ='a'; // Zeichen,variable a wird erstellt und mit dem Wert a belegt
```
### Zeichenkette
```rust
let a: String ="Hallo Welt!"; // Zeichenkette,variable a wird erstellt und mit dem Wert Hallo Welt! belegt
```
