use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use std::error::Error;

#[derive(Parser)]
#[command(name = "Job Search")]
#[command(version = "1.0")]
#[command(author = "Ваше имя <ваш_email@example.com>")]
#[command(about = "Ищет вакансии по заголовку")]
struct Cli {
    /// Заголовок вакансии для поиска
    title_query: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Получаем значение title_query из аргументов командной строки
    let title_query = &cli.title_query;

    // let title_query = "java"; // Замените на заголовок вакансии для поиска
    let url = format!("https://api.hh.ru/vacancies?text={}", title_query);

    // Создаем заголовки
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("api-test-agent"));

    // Отправляем GET-запрос с заголовками
    let client = reqwest::Client::new();
    let response = client.get(&url).headers(headers).send().await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;

        let mut counter = 0;

        if let Some(vacancies) = json.get("items") {
            if let Some(vacancy_array) = vacancies.as_array() {
                'outer: for vacancy in vacancy_array {
                    if let Some(title) = vacancy.get("name") {
                        let tl = title.to_string().to_lowercase();

                        for kw in title_query.split_whitespace() {
                            if !tl.contains(kw) {
                                continue 'outer;
                            }
                        }

                        counter += 1;
                        println!("===================");
                        println!("Вакансия: {}", title);
                        println!("===================");
                        println!("{}", serde_json::to_string_pretty(&vacancy)?);
                        println!("\n\n\n");
                    }
                }
            }
            println!("===================");
            println!("Найдено вакансий: {}", counter);
            println!("===================");
        } else {
            println!("Нет вакансий");
        }
    } else {
        eprintln!("Error: {}", response.status());
        let error_text = response.text().await?;
        eprintln!("Response body: {}", error_text);
    }

    Ok(())
}
