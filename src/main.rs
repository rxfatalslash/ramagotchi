use std::{thread::sleep, time::Duration, io::{self, Write}};
use colored::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Feliz,
    Triste,
    Hambriento,
    Muerto,
}

#[derive(Debug, Clone, Copy)]
enum Position {
    P1,
    P2,
}

struct Tamagotchi {
    estado: State,
    comida: u8,
    felicidad: u8,
    peso: u8,
    posicion: Position,
}

impl Tamagotchi {
    fn new() -> Tamagotchi {
        Tamagotchi {
            estado: State::Feliz,
            comida: 0,
            felicidad: 0,
            peso: 5,
            posicion: Position::P1,
        }
    }

    fn feed(&mut self) {
        self.comida = self.comida.saturating_add(1).min(4);

        if self.comida == 4 {
            self.peso = self.peso.saturating_add(2).min(100);
        }
    }

    fn play(&mut self) {
        self.felicidad = self.felicidad.saturating_add(1).min(4);
    }

    fn change_position(&mut self) {
        let last_pos = self.posicion;

        self.posicion = match last_pos {
            Position::P2 => Position::P1,
            Position::P1 => Position::P2,
        };
    }

    fn update(&mut self) {
        self.change_position();
        self.estado = if self.felicidad >= 2 && self.comida >= 2 {
            State::Feliz
        } else if self.felicidad < 2 {
            State::Triste
        } else {
            State::Hambriento
        };

        self.estado = if self.peso == 100 {
            State::Muerto
        } else {
            self.estado
        };

        if rand::random::<f64>() < 0.05 && self.comida > 0 {
            self.comida = self.comida.saturating_sub(1);
        }

        if rand::random::<f64>() < 0.05 && self.felicidad > 0 {
            self.felicidad = self.felicidad.saturating_sub(1);
        }

        if rand::random::<f64>() < 0.01 && self.peso > 5 {
            self.peso = self.peso.saturating_sub(2);
        }
    }

    fn get_char(&self) -> &'static str {
        match (self.posicion, self.estado) {
            (Position::P1, State::Triste) => r#"
                    (\(\
                    (-_-)
                    o(")(")o
"#,
            (Position::P1, State::Hambriento) => r#"
                    (\(\
                    ( -_-)o
                    o_(")(")
"#,
            (Position::P1, State::Feliz) => r#"
                    (\(\
                    ( ºoº)o
                    o_(")(")
"#,
            (Position::P1, State::Muerto) => r#"
                    (\(\
                    (x_x)
                    o(")(")o
"#,
            (Position::P2, State::Triste) => r#"
                    (\(\
                    ( -_-)o
                    o_(")(")
"#,
            (Position::P2, State::Feliz) => r#"
                    (\(\
                    (OvO)
                    o(")(")o
"#,
            (Position::P2, State::Hambriento) => r#"
                    (\(\
                    ( O_O)o
                    o(")(")
"#,
            (Position::P2, State::Muerto) => r#"
                    (\(\
                    (x_x)
                    o(")(")o
"#,
        }
    }

    fn print_status(&self) {
        println!("{}", self.get_char());
        println!(
            "{} {:?} {} {} {} {} {} {} kg",
            "Estado:".green(), self.estado, "Comida:".green(), self.comida, "Felicidad:".green(), self.felicidad, "Peso:".green(), self.peso
        );
    }
}

fn read_input() -> Option<String> {
    println!("\n1. Alimentar");
    println!("2. Jugar");
    println!("3. Salir");
    print!("\n¿Qué quieres hacer? ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|_| panic!("{} Fallo al leer el input", "ERROR".red()));

    let t_input = input.trim().to_lowercase();
    if t_input.is_empty() {
        None
    } else {
        Some(t_input.to_string())
    }
}

fn main() {
    let mut tamagotchi = Tamagotchi::new();

    loop {
        tamagotchi.update();
        tamagotchi.print_status();

        if let Some(input) = read_input() {
            match input.trim() {
                "1" | "alimentar" | "a" => tamagotchi.feed(),
                "2" | "jugar" | "j" => tamagotchi.play(),
                "3" | "salir" | "q" => break,
                _ => eprintln!("{} Opción inválida", "ERROR".red()),
            }
        }

        sleep(Duration::from_millis(500));
        if tamagotchi.estado == State::Muerto {
            eprintln!("\n{}", "Tu Ramagotchi ha muerto".red());
            break;
        } else {
            print!("\x1B[2J\x1B[1;1H");
        }
    }
}
