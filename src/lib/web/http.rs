use crate::data::AppDatabase;
use crate::service;
use crate::service::action;
use crate::web::{ctx, form, renderer::Renderer};
use crate::{ServiceError, ShortCode};
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::content::RawHtml;
use rocket::response::{status, Redirect};
use rocket::{uri, State};

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();
    RawHtml(renderer.render(context, &[]))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home]
}

pub mod catcher {
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(req:&Request) -> &'static str {
        eprintln!("General Error: {:?}", req);
        "Something went wrong..."
    }
    #[catch(500)]
    fn internal_error(req:&Request) -> &'static str {
        eprintln!("Internal Error: {:?}", req);
        "Internal Server Error"
    }
    #[catch(404)]
    fn page_not_found(req:&Request) -> &'static str {
        eprintln!("Page Not Found: {:?}", req);
        "404 Page Not Found"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![page_not_found, internal_error, default]
    }
}
