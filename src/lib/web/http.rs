use crate::data::{AppDatabase, DatabasePool};
use crate::service;
use crate::service::action;
use crate::web::{ctx, form, renderer::Renderer};
use crate::{ServiceError, ShortCode};
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::content::RawHtml;
use rocket::response::{status, Redirect};
use rocket::{uri, State};
use crate::web::PASSWORD_COOKIES;

use super::PageError;

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();
    RawHtml(renderer.render(context, &[]))
}

#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>
) -> Result<status::Custom<RawHtml<String>>, PageError>{
    fn render_with_status<T: ctx::PageContext + serde::Serialize + std::fmt::Debug>(
        status: Status,
        context: T,
        renderer: &Renderer
    ) -> Result<status::Custom<RawHtml<String>>, PageError> {
        Ok(status::Custom(status, RawHtml(renderer.render(context, &[]))))
    }
    match action::get_clip(shortcode.clone().into(), database.get_pool()).await {
        Ok(clip) => {
            let context = ctx::ViewClip::new(clip);
            render_with_status(Status::Ok, context, renderer)
        }
        Err(e) => match e {
            ServiceError::PermissionError(_) => {
                let context = ctx::PasswordRequired::new(shortcode);
                render_with_status(Status::Unauthorized, context, renderer)
            }

            ServiceError::NotFound => Err(PageError::NotFound("Clip not Found".to_owned())),
            _ => Err(PageError::Internatl("Internal Server Error".to_owned()))
        }
    }
}

#[rocket::post("/", data = "<form>")]
async fn new_clip(
    form: Form<Contextual<'_, form::NewClip>>,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>
) -> Result<Redirect, (Status, RawHtml<String>)> {
    let form = form.into_inner();
    if let Some(value) = form.value {
        let req = service::ask::NewClip{
            content: value.content,
            title: value.title,
            expires: value.expires,
            password: value.password
        };
        match action::new_clip(req, database.get_pool()).await {
            Ok(clip) => Ok(Redirect::to(uri!(get_clip(shortcode = clip.shortcode)))),
            Err(e) => {
                eprintln!("Internal Error: {}",e);
                Err((
                    Status::InternalServerError,
                    RawHtml(renderer.render(
                        ctx::Home::default(),
                        &["A Internal Server Error: please try again"]
                    ))
                ))
            }
        }
    }else{
        let errors = form
            .context
            .errors()
            .map(|err| {
                if let rocket::form::error::ErrorKind::Validation(msg) = &err.kind {
                    msg.as_ref()
                }else{
                    eprintln!("Unhandled error: {}", err);
                    "An Error has occured Please try again"
                }
            }).collect::<Vec<_>>();

            Err((
                Status::BadRequest,
                RawHtml(renderer.render_with_data(ctx::Home::default(),("clip", &form.context), &errors ))
            ))
    }
}

#[rocket::post("/clip/<shortcode>", data = "<form>")]
async fn submit_clip_password(
    cookies: &CookieJar<'_>,
    form: Form<Contextual<'_, form::GetPasswordProtectedClip>>,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
    shortcode: ShortCode
) -> Result<RawHtml<String>, PageError> {
    if let Some(clip) = &form.value {
        let req = service::ask::GetClip{
            shortcode: shortcode.clone(),
            password: form.password.clone(),
        };
        match action::get_clip(req, database.get_pool()).await {
            Ok(clip) => {
                let context = ctx::ViewClip::new(clip);
                cookies.add(Cookie::new(
                    PASSWORD_COOKIES,
                    form.password.clone().into_inner().unwrap_or_default(),
                ));
                Ok(RawHtml(renderer.render(context, &[])))
            }

            Err(e) => match e {
                ServiceError::PermissionError(e) => {
                    let context = ctx::PasswordRequired::new(shortcode);
                    Ok(RawHtml(renderer.render(context, &[e.as_str()])))
                },
                ServiceError::NotFound => Err(PageError::NotFound("Clip not Found".to_owned())),
                _ => Err(PageError::Internatl("Internal server Error {}".to_owned()))
            }
        }
    }else{
        let context = ctx::PasswordRequired::new(shortcode);
        Ok(RawHtml(renderer.render(
            context,
            &["Password is Required for this clip"]
        )))
    }   
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home, get_clip, new_clip, submit_clip_password]
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
