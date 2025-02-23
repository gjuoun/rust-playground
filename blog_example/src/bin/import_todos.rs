use blog_example::config::env_config::EnvConfig;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    #[serde(rename = "userId")]
    user_id: i32,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // Initialize environment config
    EnvConfig::init()?;
    let config = EnvConfig::get_instance();

    // Read the JSON file
    let json_content = fs::read_to_string("src/todos.json")?;
    let todos: Vec<Todo> = serde_json::from_str(&json_content)?;

    // Connect to the database
    let pool = PgPool::connect(&config.database_url).await?;

    println!("Found {} todos to import", todos.len());

    // Process in chunks of 25
    for chunk in todos.chunks(25) {
        let todos_data: (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = chunk
            .iter()
            .map(|todo| (todo.id, todo.title.clone(), todo.user_id, todo.body.clone()))
            .fold((vec![], vec![], vec![], vec![]), |mut acc, (a, b, c, d)| {
                acc.0.push(a);
                acc.1.push(b);
                acc.2.push(c);
                acc.3.push(d);
                acc
            });

        let (ids, titles, user_ids, bodies) = todos_data;

        sqlx::query!(
            r#"
            INSERT INTO todos (id, title, "userId", body)
            SELECT * FROM UNNEST($1::int[], $2::text[], $3::int[], $4::text[])
            ON CONFLICT (id) DO UPDATE
            SET title = EXCLUDED.title,
                "userId" = EXCLUDED."userId",
                body = EXCLUDED.body
            "#,
            &ids,
            &titles,
            &user_ids,
            &bodies
        )
        .execute(&pool)
        .await?;

        println!("Imported chunk of {} todos", chunk.len());
    }

    println!("Import completed successfully!");
    Ok(())
}
