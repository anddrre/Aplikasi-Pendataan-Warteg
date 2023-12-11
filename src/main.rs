use std::collections::HashMap;
use std::io::{self, Write};

struct Warteg {
    menu: HashMap<String, f64>,
}

impl Warteg {
    fn new() -> Warteg {
        Warteg {
            menu: HashMap::new(),
        }
    }

    fn add_menu_item(&mut self, name: String, price: f64) {
        self.menu.insert(name, price);
    }

    fn view_menu(&self) {
        println!("Daftar Menu Warteg:");
        for (item, price) in &self.menu {
            println!("{}: Rp {}", item, price);
        }
    }

    fn search_menu(&self, query: &str) {
        match self.menu.get(query) {
            Some(price) => println!("Harga {} adalah Rp {}", query, price),
            None => println!("Menu {} tidak ditemukan.", query),
        }
    }
}

fn main() {
    let mut warteg = Warteg::new();

    loop {
        println!("\nMenu:");
        println!("1. Tambah Item Menu");
        println!("2. Lihat Menu");
        println!("3. Cari Harga Menu");
        println!("4. Keluar");

        print!("Pilih opsi: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");

        match input.trim() {
            "1" => {
                print!("Masukkan nama menu: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Gagal membaca input");

                print!("Masukkan harga menu: ");
                io::stdout().flush().unwrap();
                let mut price = String::new();
                io::stdin().read_line(&mut price).expect("Gagal membaca input");

                let name = name.trim().to_string();
                let price: f64 = price.trim().parse().expect("Gagal mengonversi harga");

                warteg.add_menu_item(name, price);
                println!("Menu berhasil ditambahkan!");
            }
            "2" => {
                warteg.view_menu();
            }
            "3" => {
                print!("Masukkan nama menu yang ingin dicari: ");
                io::stdout().flush().unwrap();
                let mut query = String::new();
                io::stdin().read_line(&mut query).expect("Gagal membaca input");

                warteg.search_menu(query.trim());
            }
            "4" => {
                println!("Terima kasih! Keluar dari aplikasi.");
                break;
            }
            _ => {
                println!("Opsi tidak valid. Silakan pilih opsi 1-4.");
            }
        }
    }
}