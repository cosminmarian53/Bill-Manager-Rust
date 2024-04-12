use std::io;
use std::collections::HashMap;
#[derive(Debug, Clone)]
//--------------Structs--------------
struct Bill {
    name: String,
    amount: f64,
}
//Struct to store the bills
struct Bills {
    inner: HashMap<String, Bill>,
}
//-----------------------------------
//Implementation of the struct Bills
impl Bills {
    //----------Functions----------
    fn new()->Self{
        Self { 
            inner: HashMap::new() 
        }
    }
    fn add_bill(&mut self,bill:Bill){
        self.inner.insert(bill.name.clone(),bill);
    }
    fn remove_bill(&mut self, name: &str) -> bool {
        // Chaning the is_some() function call will allow us to return
        // whether an item was removed or not.
        self.inner.remove(name).is_some()
    }
    fn edit_bill(&mut self,name:&str,amount:f64) -> bool{

        match self.inner.get_mut(name){
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
    fn get_all(&self) -> Vec<&Bill>{
        let mut bills = vec![];
        for bill in self.inner.values(){
            bills.push(bill);
        }
        bills
    }
    //---------End of bill functions---------
}
fn get_input() -> Option <String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){
        println!("Please enter your data!");
    }
    let input = buffer.trim().to_owned();
    if input.is_empty(){
        None
    } else {
        Some(input)
    }
}
//----------Functions for the bill menu----------
fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}
fn add_bill_menu(bills: &mut Bills) {
    println!("Bill name:");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };
    let bill = Bill { name, amount };
    bills.add_bill(bill);
    println!("Bill added");
}
fn view_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}
fn edit_bill_menu(bills:&mut Bills){
        println!("Enter bill name to edit:");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        let amount=match get_bill_amount(){
            Some(amount) => amount,
            None => return,
        };
        if bills.edit_bill(&name,amount){
            println!("Bill edited");
        } 
}  
fn remove_bill_menu(bills: &mut Bills) {
    println!("Bill name:");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    if bills.remove_bill(&name) {
        println!("Bill removed");
    } else {
        println!("Bill not found");
    }
}
//-------------End of Functions----------------

//--------------Main Menu------------------------
fn main_menu(){
    fn show(){
        println!("=====Manage Bills=====");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Edit bill");
        println!("5. Exit");
    }
    let mut bills = Bills::new();
    loop{
        show();
        let input= get_input().unwrap();
        match input.as_str(){
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            "4" => edit_bill_menu(&mut bills),
            "5" => println!("Exit program!"),
            _ => println!("Invalid input"),
        } 
        if input == "5".to_owned(){
            break;
        }else if input>"5".to_owned(){
            println!("Invalid input! Please enter a number between 1 and 5.");
            continue;
        }
    }
//--------------End of Main Menu-----------------
}
fn main() {
    main_menu();
}