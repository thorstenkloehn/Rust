---
title: "Installieren"
date: 2023-09-07T21:37:24+02:00
draft: false
type: "page"
menu: 
  main:
    name: "Installieren"
    weight: 1
    
---

# Rust Programmiersprache
Rust ist eine moderne Programmiersprache, die sich durch ihre Sicherheit und Geschwindigkeit auszeichnet. Sie ist eine gute Alternative zu C und C++ und wird von Mozilla entwickelt. Rust ist eine Multiparadigmen-Sprache, die funktionale, imperative und objektorientierte Programmierung unterstützt. Sie ist eine statisch typisierte Sprache, die aber auch Typinferenz unterstützt. Rust ist eine kompilierte Sprache, die sich durch ihre Geschwindigkeit auszeichnet. 

## Installation
Rust kann über den Paketmanager installiert werden. Auf Ubuntu und Debian kann Rust mit folgendem Befehl installiert werden:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Nach der Installation muss die Rust-Toolchain aktiviert werden. Dazu muss folgender Befehl ausgeführt werden:
```bash
source $HOME/.cargo/env
```
Um die Installation zu überprüfen, kann folgender Befehl ausgeführt werden:
```bash
rustc --version
```
Dieser sollte die Version von Rust ausgeben.

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

Was ist ein String::from
`String::from` ist eine Funktion, die eine Zeichenkette erstellt. Sie kann mit folgendem Code aufgerufen werden:
```rust
let a = String::from("Hallo Welt!");
```



