use std::collections::HashMap;
use actix_web::{web::ServiceConfig, App, HttpServer,HttpResponse, Responder,get,web};
use shuttle_actix_web::ShuttleActixWeb;




use csv::{ReaderBuilder, Trim};

struct Job {
    company: String,
    title: String,
    location: String,
}

pub fn count_positions(city: &str) ->  String  {
    let file = std::fs::File::open("jobs.csv").unwrap();
    let mut csv_reader = ReaderBuilder::new()
    .has_headers(true)
    .from_reader(file);
    let mut jobs = Vec::new();
    for result in csv_reader.records() {
        let record = result.unwrap();
        let company = &record[1];
        let title = &record[0];
        let location = &record[2];
        jobs.push(Job {
                company: company.to_string(),
                title: title.to_string(),
                location: location.to_string(),
            });
    }
    let mut total_positions = 0;
    total_positions = jobs.iter().filter(|j| j.location.contains(city)).count();
    total_positions.to_string()
    
    
}



#[get("/NYC")]
async fn New_York() -> HttpResponse {
    let city = "New York";

    let total_positions = count_positions("New York");
   // println!(total_positions)

   let html = "<html>\
                <head>\
                    <title>Total Positions in New York</title>\
                </head>\
                <body>\
                    <h1>Total Positions in New York</h1>\
                    <p>City: New York</p>\
                    <p>Total Positions: ".to_string()
        + &total_positions
        + "</p>\
                </body>\
                </html>";

HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}

#[get("/CHI")]
async fn Chicago() -> HttpResponse {

    let city = "Chicago";

    let total_positions = count_positions("Chicago");
   // println!(total_positions)

   let html = "<html>\
                <head>\
                    <title>Total Positions in Chicago</title>\
                </head>\
                <body>\
                    <h1>Total Positions in Chicago</h1>\
                    <p>City: Chicago</p>\
                    <p>Total Positions: ".to_string()
        + &total_positions
        + "</p>\
                </body>\
                </html>";

HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}
#[get("/SF")]
async fn San_Fransisco() -> HttpResponse {

    let city = "San Fransisco";

    let total_positions = count_positions("San Fransisco");
   // println!(total_positions)

   let html = "<html>\
                <head>\
                    <title>Total Positions in San Fransisco</title>\
                </head>\
                <body>\
                    <h1>Total Positions in San Fransisco</h1>\
                    <p>City: San Fransisco</p>\
                    <p>Total Positions: ".to_string()
        + &total_positions
        + "</p>\
                </body>\
                </html>";

HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}

#[get("/")]
async fn hello() -> HttpResponse {
    let html = r#"
    <html>
    <head>
        <title>Calculate Total Positions in Specific City</title>
    </head>
    <body>
        <h1>Calculate Total Positions in Specific City</h1>
        <p>Please specify a city:</p>
        <ul>
            <li><a href="/NYC">New York (NY)</a></li>
            <li><a href="/CHI">Chicago (CHI)</a></li>
            <li><a href="/SF">San Francisco (SF)</a></li>
        </ul>
    </body>
    </html>
"#;

HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)

}





#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello).service(New_York).service(Chicago).service(San_Fransisco);
    };
    
    Ok(config.into())
}



