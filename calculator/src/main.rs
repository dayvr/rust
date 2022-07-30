use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct Parameters {
    a: u64,
    b: u64,
}

fn calculate(a: u64, b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    let result = a + b;
    result
}

fn post_calculate(form: web::Form<Parameters>) -> HttpResponse {
    if form.a == 0 || form.b == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Enter a number greater than zero.");
    }
    
    let response =
        format!("Result: {}  +  {}  =  <b>{}</b>",
                form.a, form.b, calculate(form.a, form.b));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn get_page() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#" 
               <title>Sum Calculator</title>
               <h1>Sum of Numbers Calculator</h1>
               <form action="/calculate" method="post">
               <input type="text" name="a"/>
               <input type="text" name="b"/>
               <button type="submit">Calculate</button>
               </form>
            "#,
        )
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_page))
            .route("/calculate", web::post().to(post_calculate))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run().expect("error running server");
}
