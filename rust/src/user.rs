use std::{collections::HashMap, fs::{File, OpenOptions}, error::Error, fmt::{self, Display}};
use csv::{Writer, WriterBuilder, Reader, ReaderBuilder};
use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub enum UserError {
    LoadingError,
    SaveError
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl Error for UserError {}


// USER STATUS

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UserStatus {
    Student,
    Teacher,
    Other
}

impl UserStatus {
    pub fn from(name: &str) -> Self {
        match name {
            "Etudiant"  => UserStatus::Student, 
            "Prof"      => UserStatus::Teacher, 
            "Autre" | _ => UserStatus::Other
        }
    }
}

impl Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


// USER

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub uid: String,
    pub surname: String,
    pub name: String,
    pub status: UserStatus
}

impl User {

    pub fn new(uid: &str, name: &str, surname: &str, status: UserStatus) -> Self {
        Self {
            uid: String::from(uid),
            name: String::from(name),
            surname: String::from(surname),
            status: status
        }
    }

    pub fn from(uid: &String, name: &String, surname: &String, status: &UserStatus) -> Self {
        Self { 
            uid: uid.clone(), 
            name: name.clone(), 
            surname: surname.clone(), 
            status: status.clone() 
        }
    }

}


// USER LIST

pub struct UserList {
    map: HashMap<String, User>,
    writer: Writer<File>
}

impl UserList {
    
    pub fn new(path: &str) -> Result<Self, UserError> {
        Ok(Self { 
            map: UserList::load_users(path)?,
            writer: UserList::create_writer(path)?
        })
    }

    fn load_users(path: &str) -> Result<HashMap<String, User>, UserError> {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)
            .map_err(|_| UserError::LoadingError)?;
        let mut map = HashMap::new();

        for result in reader.deserialize() {
            let user: User = result.map_err(|_| UserError::LoadingError)?;
            UserList::add_inner(&mut map, &user);
        }

        Ok(map)
    }

    fn create_writer(path: &str) -> Result<Writer<File>, UserError> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|_| UserError::LoadingError)?;

        let writer = WriterBuilder::new().has_headers(false).from_writer(file);
        return Ok(writer);
    }

    fn add_inner(map: &mut HashMap<String, User>, user: &User) {
        map.insert(user.uid.clone(), user.clone());
    }

    pub fn add(&mut self, user: &User) -> Result<(), UserError> {
        UserList::add_inner(&mut self.map, user);

        self.writer.serialize(user).map_err(|_| UserError::SaveError)?;
        self.writer.flush().map_err(|_| UserError::SaveError)?;

        Ok(())
    }

    pub fn get(&mut self, uid: &String) -> Option<&User> {
        self.map.get(uid)
    }
}