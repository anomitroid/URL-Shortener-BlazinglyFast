use actix_web::{web, HttpResponse};
use crate::{AppState, handlers::render_template};

pub async fn index(data: web::Data<AppState>) -> HttpResponse {
    let mut rows = String::new();
    
    for entry in data.store.iter() {
        rows.push_str(&format!(
            r#"<tr>
                <td><a href="{}">{}</a></td>
                <td><a href="/{}">{}</a></td>
                <td>{}</td>
            </tr>"#,
            entry.value().full_url,
            entry.value().full_url,
            entry.key(),
            entry.key(),
            entry.value().clicks
        ));
    }

    HttpResponse::Ok()
        .content_type("text/html")
        .body(render_template(&rows))
}