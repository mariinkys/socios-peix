pub mod components;
pub mod core;
pub mod pages;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;
    use socios_peix::{app::*, core::database::init_database};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    // Database
    // Ej: export DATABASE_URL="sqlite:socios.db"
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_database(&database_url).await.unwrap();

    // Email client
    let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
    let email_client: core::email_client::EmailClient =
        core::email_client::init_email_client(&smtp_username, &smtp_password).unwrap();

    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        App::new()
            // database
            .app_data(web::Data::new(pool.clone()))
            // email_client
            .app_data(web::Data::new(email_client.clone()))
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", &site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            // .route("/test-extraction", web::get().to(test_email_extraction))
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use socios_peix::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}

// async fn test_email_extraction(
//     pool: actix_web::web::Data<sqlx::Pool<sqlx::Sqlite>>,
//     email_client: actix_web::web::Data<core::email_client::EmailClient>,
// ) -> Result<actix_web::HttpResponse, actix_web::Error> {
//     println!(
//         "Database extraction: SUCCESS - {:?}",
//         std::any::type_name::<sqlx::Pool<sqlx::Sqlite>>()
//     );
//     println!(
//         "EmailClient extraction: SUCCESS - {:?}",
//         std::any::type_name::<core::email_client::EmailClient>()
//     );

//     Ok(actix_web::HttpResponse::Ok().json("Both extractions successful"))
// }
