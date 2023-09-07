fn main() {  // Programm startet hier
    // Konsole eingabe einlesen
    println!("Wie heißt du?"); // Text ausgeben
    let mut input = String::new(); // Leere Zeichenkette mit Namen input erstellen und veränderbar machen

    std::io::stdin().read_line(&mut input).unwrap(); // Zeichenkette von der Konsole einlesen und in input speichern
    // Eingabe ausgeben
    println!("Hallo {}!", input); // Eingabe ausgeben
}


