extern crate rpassword;
use std::io;
use std::io::Write;
use rpassword::read_password;

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

  println!("Your PIN is {:?}", card_pin);
  println!("Menu {:?} choosed", choosen);
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
