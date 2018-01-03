extern crate rpassword;
use std::io;
use std::io::Write;
use rpassword::read_password;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  print_banner();

  let mut card_number = String::new();
  println!("Please type your ATM card number, just the number!");
  print!("Number: ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut card_number).expect("ATM Error!");

  print!("PIN: ");
  io::stdout().flush().unwrap();
  let card_pin = read_password().unwrap();

  print_menu();
  print!("Choose menu (1-5): ");
  io::stdout().flush().unwrap();
  let mut choosen = String::new();
  io::stdin().read_line(&mut choosen).expect("ATM Error!");
  print_separator();

  println!("Your PIN is {:?}", card_pin);
  println!("Menu {:?} choosed", choosen);

  let mut f_handler = File::open(format!("{}.txt", card_number)).expect("Card invalid!");
  let mut card_info = String::new();
  f_handler.read_to_string(&mut card_info).expect("Something went wrong with file.");

  println!("Info {:?} ", card_info);
}

fn print_banner(){
  println!("==========================");
  println!("-     ArtosTinaMesin     -");
  print_separator();
}

fn print_menu(){
  print_separator();
  println!("Menu:");
  println!("  1. Balance info");
  println!("  2. Transfer");
  println!("  3. Pay bill");
  println!("  4. Change PIN");
  println!("  5. Exit");
  print_separator();
}

fn print_separator(){
  println!("--------------------------");
}
