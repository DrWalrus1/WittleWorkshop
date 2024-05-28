use rocket::{get, response::content};
use tera::Tera;

pub mod docker_routes;
pub mod service_routes;


#[get("/")]
pub fn tera_test() -> content::RawHtml<String> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut context = tera::Context::new();

    let mut renderResult = tera.render("index.html", &context).unwrap();
    return content::RawHtml(renderResult);
}
