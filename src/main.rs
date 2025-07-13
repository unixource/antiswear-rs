use antiswear::{Antiswear, AntiswearGroup};
use std::io;
use std::time::SystemTime;

fn main() {
    let antiswear = AntiswearGroup {
       elems: vec![
            Antiswear::en(),
            Antiswear::ru(),
       ] 
    };
    
    loop {
        let mut to_check = String::new();

        io::stdin().read_line(&mut to_check).expect("WTF");

        let time = SystemTime::now();

        match antiswear.check(&to_check) {
            Some(result) => println!("Найдено матное слово \"{}\" с индексом {}", result.word, result.index),   
            None => println!("Не найдено"),
        }

        println!("Проверено за: {:?} мc\n", (time.elapsed().expect("WTF").as_millis()));
    };
}
