mod config;
mod connection;
mod error;
mod model;
mod repo;
mod seed;
mod service;
mod validation;

use crate::{
    config::Config, error::AppResult, model::Grades, repo::StudentRepo, service::StudentService,
};

use std::fmt::Debug;

/// Usage:
/// cargo run count the students (default)
/// cargo run -- seed insert the three sample students
/// cargo run -- create create one valid student
/// cargo run -- create-invalid empty name and grade 6
/// cargo run -- get <id> read one student
/// cargo run -- delete <id> delete one student
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config = Config::from_env();
    println!("Connecting {}", config.uri);

    let client = connection::connect(&config).await?;
    let service = StudentService::new(StudentRepo::new(&client, &config));

    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("count");
    let id = args.get(1).map(String::as_str).unwrap_or("");

    match command {
        "seed" => print_result(service.import(seed::sample_students()?).await),
        "create" => print_result(service.create(seed::sample_new_student()?).await),
        "create-invalid" => {
            let mut student = seed::sample_new_student()?;
            student.name = String::new();
            student.math_grades = Grades(vec![6]);
            print_result(service.create(student).await);
        }
        "get" => print_result(service.get(id).await),
        "delete" => print_result(service.delete(id).await),
        "count" => print_result(service.count().await),
        other => println!("unknown command: {other}"),
    }

    Ok(())

    // TODO:
    // NOTE:
    //  üzenet továbbítása socketre	to_writer vagy to_vec

    // csomagméret mérése / tömörítés	to_vec
    // transzparens módosítás (pl. mező átírása) serde_json::Value / bson::Document
    // debug logolás to_string
    // zlibcompression mongodb tárolási tömörítés
    // külön mappába mehet majd /Programs/
    //
    // snifferhez:
    // érdemes előbb **libpcap** vagy **pcap**, majd ezt megnézni low-level-el mint
    // például **eBPF** vagy akár legalacsonyabb szinten **ftrace**(LINUX), **ETW**(WINDOWS)
}

fn print_result<T: Debug>(result: AppResult<T>) {
    match result {
        Ok(reply) => println!("{} {:#?}", reply.status, reply.body),
        Err(error) => println!("{} {}", error.status(), error),
    }
}
