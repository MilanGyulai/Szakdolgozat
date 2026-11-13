mod config;
mod connection;
mod error;
mod model;
mod query;
mod repo;
mod student_seed;
mod service;
mod validation;

use crate::{
    config::Config, error::AppResult, model::Grades, repo::StudentRepo, service::StudentService,
};

use std::{fmt::Debug};
use std::io::{self, Write};
use crate::model::{StudentPatch, Subject};
use crate::query::{SortField, SortOrder, StudentFilter};

/// Usage:
/// cargo run count the students (default)
/// cargo run -- seed insert the three sample students
/// cargo run -- create creates one valid student
/// cargo run -- create-invalid empty name and grade 6
/// cargo run -- get <id> read one student
/// cargo run -- delete <id> delete one student
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config = Config::from_env();
    println!("Connecting {}", config.uri);

    let client = connection::connect(&config).await?;
    let service = StudentService::new(StudentRepo::new(&client, &config).await);

    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("count");
    let id = args.get(1).map(String::as_str).unwrap_or("");


    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let args: Vec<&str> = input.split_whitespace().collect();

        let command = args.first().copied().unwrap_or("");
        let id = args.get(1).copied().unwrap_or("");

        if command == "exit" {
            println!("Exiting application...");
            break;
        }
        match command {
            "seed" => print_result(
                "Students imported",
                service.import(student_seed::sample_students()?).await,
            ),
            "create" => print_result(
                "Student created",
                service.create(student_seed::sample_new_student()?).await,
            ),
            "create-invalid" => {
                let mut student = student_seed::sample_new_student()?;
                student.name = String::new();
                student.math_grades = Grades(vec![6]);
                print_result(
                    "Invalid students created (not stored)",
                    service.create(student).await,
                );
            }
            "get" => print_result("Student by id got", service.get(id).await),
            "delete" => print_result("Deleted student by id", service.delete(id).await),
            "count" => print_result("Number of students found in DB:", service.count().await),
            "list" => {
                let mut filter = StudentFilter::default()
                    .sort(SortField::Gpa, SortOrder::Descending)
                    .per_page(2);
                if let Some(class) = args.get(1) {
                    filter = filter.class(*class);
                }
                if let Some(page) = args.get(2).and_then(|text| text.parse().ok()) {
                    filter = filter.page(page);
                }
                print_result("Students listed", service.list(&filter).await);
            }
            "rename" => {
                let name = args.get(2).map(|name| name.to_string());
                let patch = StudentPatch { name, ..Default::default() };
                print_result("Student renamed", service.update(id, patch).await);
            }
            "deactivate" => print_result("Student deactivated", service.set_active(id, false).await),
            "grade" => match args.get(2).map(|text| text.parse::<Subject>()) {
                Some(Ok(subject)) => {
                    // Text that is not a number becomes 0, which validation rejects with 422.
                    let grade = args.get(3).and_then(|text| text.parse().ok()).unwrap_or(0);
                    print_result("Grade added", service.add_grade(id, subject, grade).await);
                }
                Some(Err(error)) => println!("{} {}", error.status(), error),
                None => println!("usage: grade <id> <math|biology> <grade>"),
            },
            "" => continue,
            other => println!("unknown command: {other}"),
        }
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
    //
    // TODO:
    // osztálydiagram
    // dokumentáció
    // működési dokumentáció
    // mappa nevek
    //
}

fn print_result<T: Debug>(message: &str, result: AppResult<T>) {
    match result {
        Ok(reply) => println!("{} {} {:#?}", reply.status, message, reply.body),
        Err(error) => println!("{} {}", error.status(), error),
    }
}
