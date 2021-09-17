#![feature(decl_macro)]
use rocket::*;
use rocket::Request;
use rocket::response::content::Json;
use rocket::request::Form; //importing Form type for /book post route and new_book fn type
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(FromForm, Debug)]  //[FromData] trait?
struct Book {
    title: String,
    author: String,
    isbn: String,
}

#[get("/")]
fn index() -> Template {
  #[derive(Serialize)]

  struct Context {
    first_name: String,
    last_name: String
  }
  let context = Context {
    first_name: String::from("Alexander"),
    last_name: String::from("Davis")
  };
  Template::render("home", context) //template must be registered in order to render it
}

#[get("/hello")] //our function expects a GET request to the /hello route
fn hello() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")  // a GET reqiest os sent to the /hello route and returns a JSON response with a 'status':'success' and a 'message':'Hello API!' body
}
#[catch(404)] //Rocket returns a 404 error when route is called
fn not_found(req: &Request) -> String { // not_found() is passed the req paramet with Request type and has a String return type
    format!("Oh no! We couldn't find the requested path '{}'", req.uri()) //error message that's returned when a bad route is used - said bad route is displayed because of uri method used on req parameter
}

#[post("/book", data = "<book_form>")] //defines which type of data Rocket should expect when watching for requests as second arg of route attrib #[post()]
fn new_book(book_form: Form<Book>) -> String {
let book: Book = book_form.into_inner(); //gets the request body from the page/user
let mut dummy_db: Vec<Book> = Vec::new(); //dummy db defined as a vector with the type "Book"
dummy_db.push(book); //push the data received from user to dummy_db -this is an expression
format!("Book added successfully: {:?}", dummy_db) //return string using the format! method/macro because the we added the dummy_db vector to the string response
} 


fn main() {
  rocket::ignite()
  .register(catchers![not_found]) //the catchers! macro allows the register() method to engage the not_found route
  .mount("/", routes![index]) //index is mounted separately because the landing page as a different base path ("/")
  .mount("/api", routes![hello, new_book])
  .attach(Template::fairing())
  .launch(); // the ignite() method from the rocket crate creates a new Rocket instance and mounts the /hello route with the mount() method and base path /api, lastly the launch() method starts the app server and listens for requests
}
