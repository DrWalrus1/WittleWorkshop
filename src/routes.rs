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

    let context = tera::Context::new();

    let render_result = tera.render("index.html", &context).unwrap();
    return content::RawHtml(render_result);
}
