use blog_example::config::env_config::EnvConfig;
use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Posts {
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

    // Fetch posts from the API
    let posts: Vec<Posts> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/posts")
        .send()
        .await?
        .json()
        .await?;

    // Connect to the database
    let pool = PgPool::connect(&config.database_url).await?;

    println!("Found {} posts to import", posts.len());

    // Process in chunks of 25
    for chunk in posts.chunks(25) {
        let posts_data: (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = chunk
            .iter()
            .map(|post| (post.id, post.title.clone(), post.user_id, post.body.clone()))
            .fold((vec![], vec![], vec![], vec![]), |mut acc, (a, b, c, d)| {
                acc.0.push(a);
                acc.1.push(b);
                acc.2.push(c);
                acc.3.push(d);
                acc
            });

        let (ids, titles, user_ids, bodies) = posts_data;

        sqlx::query!(
            r#"
            INSERT INTO posts (id, title, "userId", body)
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

        println!("Imported chunk of {} posts", chunk.len());
    }

    println!("Import completed successfully!");
    Ok(())
}
