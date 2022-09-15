mod user;
mod rfid;

use csv::{Reader, Writer, WriterBuilder, ReaderBuilder};
use rfid::RFIDReader;
use user::{User, UserList, UserStatus};

use inquire::{Text, Select};
use std::fs::OpenOptions;
use std::error::Error;

#[allow(unreachable_code)]
fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = RFIDReader::new()?;
    let mut list = UserList::new("./res/user.csv")?;

    // list.add(&User::new("1", "Doe", "John", UserStatus::Student))?;


    clear();
    println!("Attente d'une carte");

    loop {
        let uid = reader.wait_until_read()?;
        let user: Box<User>;

        match list.get(&uid) {
            None => {

                clear();
                println!("Enregistrement d'un nouvel utilisateur");
                
                let new_user = ask_user_info(&uid)?;
                list.add(&new_user)?;
                
                user = Box::new(new_user);
            }
            Some(u) => {
                user = Box::new(u.clone());
            }
        }

        clear();
        println!("Bonjour {} !", user.surname);

    }
    
    Ok(())
}

fn clear() {
    terminal::stdout()
        .act(terminal::Action::ClearTerminal(terminal::Clear::All))
        .expect("Couldn't clear terminal");
}

fn ask_user_info(uid: &str) -> Result<User, Box<dyn Error>> {
    let surname = Text::new("Votre Prénom: ").prompt()?;
    let name = Text::new("Votre Nom: ").prompt()?;
    
    let status = Select::new("Vous êtes: ", vec!["Etudiant", "Prof", "Autre"])
    .prompt()?;
    
    let status = UserStatus::from(status);
    Ok(User::new(uid, name.as_str(), surname.as_str(), status))
}