use blog_example::config::env_config::EnvConfig;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
struct UserWithCity {
    name: String,
    city: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserWithLocation {
    name: String,
    lat: String,
    lng: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompanyInfo {
    user_name: String,
    company_name: String,
    catch_phrase: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    EnvConfig::init()?;
    let config = EnvConfig::get_instance();
    let pool = PgPool::connect(&config.database_url).await?;

    // Example 1: Query users and their cities
    println!("\n--- Users and their cities ---");
    let users_cities: Vec<UserWithCity> = sqlx::query_as!(
        UserWithCity,
        r#"
        SELECT 
            name,
            address->>'city' as "city!"
        FROM users
        ORDER BY name
        "#
    )
    .fetch_all(&pool)
    .await?;

    for user in users_cities {
        println!("{} lives in {}", user.name, user.city);
    }

    // Example 2: Find users by city
    println!("\n--- Users in a specific city ---");
    let city = "Gwenborough";
    let users_in_city: Vec<String> = sqlx::query_scalar!(
        r#"
        SELECT name
        FROM users
        WHERE address->>'city' = $1
        "#,
        city
    )
    .fetch_all(&pool)
    .await?;

    println!("Users in {}: {:?}", city, users_in_city);

    // Example 3: Get user locations (nested JSONB access)
    println!("\n--- User locations ---");
    let user_locations: Vec<UserWithLocation> = sqlx::query_as!(
        UserWithLocation,
        r#"
        SELECT 
            name,
            address->'geo'->>'lat' as "lat!",
            address->'geo'->>'lng' as "lng!"
        FROM users
        "#
    )
    .fetch_all(&pool)
    .await?;

    for location in user_locations {
        println!(
            "{} is at ({}, {})",
            location.name, location.lat, location.lng
        );
    }

    // Example 4: Complex JSONB query with company info
    println!("\n--- Company information ---");
    let company_info: Vec<CompanyInfo> = sqlx::query_as!(
        CompanyInfo,
        r#"
        SELECT 
            name as "user_name!",
            company->>'name' as "company_name!",
            company->>'catchPhrase' as "catch_phrase!"
        FROM users
        WHERE company->>'name' IS NOT NULL
        "#
    )
    .fetch_all(&pool)
    .await?;

    for info in company_info {
        println!(
            "{} works at {} - '{}'",
            info.user_name, info.company_name, info.catch_phrase
        );
    }

    // Example 5: Search in nested JSONB arrays (if your data had arrays)
    println!("\n--- Users with specific street pattern ---");
    let street_pattern = "%Light%";
    let matching_users: Vec<String> = sqlx::query_scalar!(
        r#"
        SELECT name
        FROM users
        WHERE address->>'street' LIKE $1
        "#,
        street_pattern
    )
    .fetch_all(&pool)
    .await?;

    println!(
        "Users on streets matching '{}': {:?}",
        street_pattern, matching_users
    );

    Ok(())
}
