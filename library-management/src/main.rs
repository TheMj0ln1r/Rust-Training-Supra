use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Clone)]
struct Book{
    title : String,
    author : String,
    isbn : u32,
    // borrowed : bool,
    due: SystemTime,
    borrower : u32
}
struct User{
    name : String,
    id : u32
}
struct Library{
    books : HashMap<String, Book>,
    users : HashMap<u32, User>,
    register : HashMap<u32, Vec<Book>> // userId => Book
}

impl Library {
    fn new() -> Self{
        Library {
            books : HashMap::new(),
            users : HashMap::new(),
            register : HashMap::new()
        }
    }
    fn add_book(&mut self, _title: String, _author: String, _isbn: u32) {
        self.books.insert(_title.clone(), Book { title: _title, author: _author, isbn: _isbn, due: SystemTime::now(), borrower: 0 });

    }
    fn remove_book(&mut self, _title: String)-> Result<(), &str>{
        let b =  self.books.get(&_title);
        match b {
            Some(book) => {
                if book.borrower == 0{
                    if self.books.remove(&_title).is_some(){ // remove returns Some(_title)
                        return Ok(());
                    }
                    else{
                        return Err("Remove failed");
                    }
                }
                else{
                    return Err("Book is borrowed");
                };
            },
            None => return Err("Book not found"),
        };
        
    }
    fn borrow_book(&mut self, _title: String, _userid: u32, _uname: String) -> Result<(), &str>{
        if _userid == 0 {
            return Err("Invalid userId");
        }
        let b = self.books.get_mut(&_title); // get book from books

        match b{
            Some(book) => 
            {// A book existed 
                if book.borrower == 0{ // not borrowed by anyone
                    // Adding user to users map if it's his first time
                    self.users.entry(_userid).or_insert(User{name: _uname, id: _userid});
                    
                    book.borrower = _userid;
                    book.due = SystemTime::now() + Duration::new(10*24*3600,0); // 10 days due

                    // update register 
                    let r = self.register.get_mut(&_userid); 
                    match r {
                        Some(list) => {
                            list.push(book.clone());
                        },
                        None => {
                            self.register.insert(_userid, vec![book.clone()]);
                        },
                    };
               }
               else {
                return Err("Book already borrowed");
               }
            },
            None => return Err("No book found")
        };
        return Ok(());
        
    }
    fn return_book(&mut self, _title: String, _userid: u32) -> Result<(), &str>{
        let b = self.books.get_mut(&_title);
        match b{
            Some(book)=>{
                if book.borrower == _userid {
                    // calculate fee
                    let timenow = SystemTime::now();
                    let elapsedtime = timenow.duration_since(book.due);
                    
                    let mut fee = 10; // 10 per day
                    match elapsedtime{
                        Ok(duration) => {
                            fee = (duration.as_secs() / 24 * 3600) * fee; 
                        },
                        Err(_e) => fee = 0, // fee is 0 if the elapsed time less than a day
                    }

                    book.borrower = 0;
                    book.due = SystemTime::now();
                    println!("Your extra fee : {}",fee);
                    // can also update register here but not required
                }
                else{
                    return Err("Book is not borrowed by this user");
                }
            },
            None => return Err("No book found"),
        }

        return Ok(());
    }

    fn list_books(&self){
        println!("........Available Books........");
        for (title, book) in self.books.iter(){
            if book.borrower == 0{
                println!("Title : {title}, Author : {}, ISBN : {}", book.author, book.isbn);
            }
        }
    }
    fn list_books_of_user(&self, userid: u32) {
        let list =  self.register.get(&userid);
        match list {
            Some(l) => {
                for i in l{
                    println!("Title : {}, Author : {}, ISBN: {} ",i.title, i.author, i.isbn);
                }
            },
            None => println!("No user found"),
        };
    }

    fn get_user_info(&self, userid: u32){
        match self.users.get(&userid){
            Some(user) => {
                println!("Usern Id : {}, User Name : {}", user.id, user.name);
            }
            None => println!("User not found"),
        };
    }

    fn display_actions(&self) -> u32{
        println!("
        1. Add Book
        2. Remove Book
        3. List Books
        4. Borrow Book
        5. Return Book 
        6. Get user records
        7. Get user info
        Please input your choice : "
        );
        let mut buf = String::new();
        std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read choice : ");
        
        match buf.trim().parse() {
            Ok(num) => {
                if num >= 1 && num<= 7{
                    return num;
                }
                else{
                    return 0;
                }
            },
            Err(_) => {
                return 0;
            },
        };
    }

}

fn main(){
    let mut library = Library::new();
    println!("Library Opened...");
    loop {
        let choice = library.display_actions();
        // println!("{}",choice);
        match choice{
            1 => {
                println!("Enter Title of the Book : ");
                let mut title = String::new();
                std::io::stdin()
                .read_line(&mut title)
                .expect("Failed to read title");
                let title = title.trim();

                println!("Enter Author of the Book : ");
                let mut author = String::new();
                std::io::stdin()
                .read_line(&mut author)
                .expect("Failed to read author");
                let author: &str = author.trim();


                println!("Enter ISBN of the Book : ");
                let mut isbn = String::new();
                std::io::stdin()
                .read_line(&mut isbn)
                .expect("Failed to read title");
                let isbn: u32 = match isbn.trim().parse(){
                    Ok(_isbn) => _isbn,
                    Err(_) => {println!("Invalid ISBN"); continue},
                };

                library.add_book(title.to_string(), author.to_string(), isbn);

                println!("NEW BOOK ADDED!!!!");

            },
            2 => {
                println!("Enter Title of the Book : ");
                let mut title = String::new();
                std::io::stdin()
                .read_line(&mut title)
                .expect("Failed to read title");
                let title = title.trim();

                match library.remove_book(title.to_string()){
                    Ok(()) => println!("BOOK REMOVED!!!!"),
                    Err(e) => {println!("{}",e); continue},
                };

            },
            3 => {
                library.list_books();
            },
            4 => {
                println!("Enter Title of the Book : ");
                let mut title = String::new();
                std::io::stdin()
                .read_line(&mut title)
                .expect("Failed to read title");
                let title = title.trim();

                println!("Enter User Id : ");
                let mut userid = String::new();
                std::io::stdin()
                .read_line(&mut userid)
                .expect("Failed to read userId");
                let userid: u32 = match userid.trim().parse(){
                    Ok(_userid) => _userid,
                    Err(_) => {println!("Invalid user Id"); continue},
                };

                println!("Enter User Name : ");
                let mut uname = String::new();
                std::io::stdin()
                .read_line(&mut uname)
                .expect("Failed to read uname");
                let uname: &str = uname.trim();

                match library.borrow_book(title.to_string(), userid, uname.to_string()){
                    Ok(()) => println!("{} Book Borrowed", title),
                    Err(e) => println!("{}",e),
                };
            },
            5 => {
                println!("Enter Title of the Book : ");
                let mut title = String::new();
                std::io::stdin()
                .read_line(&mut title)
                .expect("Failed to read title");
                let title = title.trim();

                println!("Enter User Id : ");
                let mut userid = String::new();
                std::io::stdin()
                .read_line(&mut userid)
                .expect("Failed to read userId");
                let userid: u32 = match userid.trim().parse(){
                    Ok(_userid) => _userid,
                    Err(_) => {println!("Invalid user Id"); continue},
                };

                let r = library.return_book(title.to_string(), userid);
                match r{
                    Ok(()) => println!("Book Returned"),
                    Err(e) => println!("{}",e),
                }

            },
            6 => {
                println!("Enter User Id : ");
                let mut userid = String::new();
                std::io::stdin()
                .read_line(&mut userid)
                .expect("Failed to read userId");
                let userid: u32 = match userid.trim().parse(){
                    Ok(_userid) => _userid,
                    Err(_) => {println!("Invalid user Id"); continue},
                };

                println!("The records of {}", userid);
                library.list_books_of_user(userid);
            },
            7 => {
                println!("Enter User Id : ");
                let mut userid = String::new();
                std::io::stdin()
                .read_line(&mut userid)
                .expect("Failed to read userId");
                let userid: u32 = match userid.trim().parse(){
                    Ok(_userid) => _userid,
                    Err(_) => {println!("Invalid user Id"); continue},
                };

                println!("User info of {}", userid);
                library.get_user_info(userid);
            },
            _ => {
                println!("Invalid Choice ");
            }
        }
    }
}