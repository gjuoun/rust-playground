use blog_example::config::env_config::EnvConfig;
use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Debug, Serialize, Deserialize)]
struct Company {
    name: String,
    #[serde(rename = "catchPhrase")]
    catch_phrase: String,
    bs: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // Initialize environment config
    EnvConfig::init()?;
    let config = EnvConfig::get_instance();

    // Fetch users from the API
    let users: Vec<User> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/users")
        .send()
        .await?
        .json()
        .await?;

    // Connect to the database
    let pool = PgPool::connect(&config.database_url).await?;

    println!("Found {} users to import", users.len());

    // Process in chunks of 10
    for chunk in users.chunks(10) {
        let users_data: (
            Vec<i32>,               // id
            Vec<String>,            // name
            Vec<String>,            // username
            Vec<String>,            // email
            Vec<serde_json::Value>, // address as JSONB
            Vec<String>,            // phone
            Vec<String>,            // website
            Vec<serde_json::Value>, // company as JSONB
        ) = chunk.iter().fold(
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            |mut acc, user| {
                acc.0.push(user.id);
                acc.1.push(user.name.clone());
                acc.2.push(user.username.clone());
                acc.3.push(user.email.clone());
                acc.4.push(serde_json::to_value(&user.address).unwrap());
                acc.5.push(user.phone.clone());
                acc.6.push(user.website.clone());
                acc.7.push(serde_json::to_value(&user.company).unwrap());
                acc
            },
        );

        sqlx::query!(
            r#"
            INSERT INTO users (
                id, name, username, email, 
                address, phone, website, company,
                created_at, updated_at
            )
            SELECT 
                u.*,
                CURRENT_TIMESTAMP as created_at,
                CURRENT_TIMESTAMP as updated_at
            FROM UNNEST(
                $1::int[], $2::text[], $3::text[], $4::text[],
                $5::jsonb[], $6::text[], $7::text[], $8::jsonb[]
            ) as u(id, name, username, email, address, phone, website, company)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                username = EXCLUDED.username,
                email = EXCLUDED.email,
                address = EXCLUDED.address,
                phone = EXCLUDED.phone,
                website = EXCLUDED.website,
                company = EXCLUDED.company,
                updated_at = CURRENT_TIMESTAMP
            "#,
            &users_data.0, // id
            &users_data.1, // name
            &users_data.2, // username
            &users_data.3, // email
            &users_data.4, // address as JSONB
            &users_data.5, // phone
            &users_data.6, // website
            &users_data.7, // company as JSONB
        )
        .execute(&pool)
        .await?;

        println!("Imported chunk of {} users", chunk.len());
    }

    println!("Import completed successfully!");
    Ok(())
}
