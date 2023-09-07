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

