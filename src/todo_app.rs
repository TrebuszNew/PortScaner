use colored::{Color, Colorize};
use std::{collections::HashMap, io::Write};

const PATH: &str = "/home/Trebusz/.local/share/save.json";

fn main() {
    let mut lista: Lista;
    if let Ok(content) = std::fs::read_to_string(PATH) {
        lista = serde_json::from_str(&content).unwrap();
    } else {
        lista = Lista::new();
    }
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("wystąpił niespodziewany błąd!");
        let input = input.trim();

        match input {
            "add" => {
                let nazwa = zapytaj_usera("nazwa");
                let opis = zapytaj_usera("opis");
                lista.dodaj_zadanie(&nazwa, &opis, Status::DoZrobienia);
            }
            "list" => {
                if lista.0.is_empty() {
                    println!("Aktualnie brak zadań, wykonad add aby dodać nowe.");
                }
                for i in lista.0.iter() {
                    let mut wypisz = String::new();
                    let (nazwa, zadanie) = i;
                    wypisz.push_str(&format!("{}:\n    {}", nazwa.trim(), &zadanie.tresc.trim()));
                    let status = match zadanie.status {
                        Status::DoZrobienia => "Do Zrobienia".color(Color::Black),
                        Status::RobiSie => "W Trakcie".color(Color::Yellow),
                        Status::Gotowe => "Zrobione".color(Color::Green),
                    };
                    println!("{} : {}", wypisz, status);
                }
            }
            "clear" => {
                clearscreen::clear().unwrap_or_else(|e| {
                    println!("{}{}", "Wystąpił Błąd: ".red(), e);
                })
            }
            "exit" => {
                std::process::exit(0);
            }
            "help" => {
                println!(
                r###"
help - wypisuje wszystkie możliwe komendy
add - dodaje nowe zadanie
list - wypisuje wszystkie zadania
clear - czyści konsole
exit - wychodzi z programu
                "###
                );
            }
            "remove" => {
                let nazwa = zapytaj_usera("nazwa");
                lista.usun_zadanie(&nazwa);
            }
            "status" => {
                let nazwa = zapytaj_usera("nazwa");
                if let Some(zadanie) = lista.0.get(&nazwa) {
                    let new_status = match zadanie.status {
                        Status::DoZrobienia => Status::RobiSie,
                        Status::RobiSie => Status::Gotowe,
                        Status::Gotowe => Status::Gotowe
                        
                    };
                    lista.zmien_status(&nazwa, new_status);
                } else {
                    println!("{}", "Wystąpił błąd!".red());
                }
            }
            _ => {
                println!("To jaka komenda wariacie? Wpisz help jeśli nie wiesz jakie są ;>");
            },
        }
        
        let serial = serde_json::to_string(&lista).unwrap();
        let mut file_handle = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(PATH)
            .expect("msg");
        file_handle.write(serial.as_bytes()).expect("Doszło tu do błędu :/");
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
enum Status {
    DoZrobienia,
    RobiSie,
    Gotowe,
}
#[derive(serde::Deserialize, serde::Serialize)]
struct Zadanie {
    tresc: String,
    status: Status,
}
#[derive(serde::Deserialize, serde::Serialize)]
struct Lista(HashMap<String, Zadanie>);
impl Lista {
    fn new() -> Self {
        Lista(HashMap::new())
    }
    fn dodaj_zadanie(&mut self, nazwa: &str, tresc: &str, status: Status) {
        self.0.insert(
            nazwa.to_string(),
            Zadanie {
                tresc: tresc.to_string(),
                status,
            },
        );
    }
    fn usun_zadanie(&mut self, nazwa: &str) {
        self.0.remove(nazwa);
    }
    fn zmien_status(&mut self, nazwa: &str, status: Status) {
        if let Some(zadanie) = self.0.get_mut(nazwa) {
            zadanie.status = status;
        } else {
            println!("{}", "Wystąpił błąd!".red());
        }
    }
}

fn zapytaj_usera(co: &str) -> String {
    println!("{}?", co);
    let mut nazwa = String::new();
    std::io::stdin().read_line(&mut nazwa).unwrap();
    nazwa
}