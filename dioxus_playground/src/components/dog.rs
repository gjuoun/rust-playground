use anyhow::{Context, Result};
use dioxus::{logger::tracing, prelude::*};
use serde::Deserialize;

//region server function
// The database is only available to server code
#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "hotdog.db" file
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        // Create the "dogs" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).unwrap();

        // Return the connection
        conn
    };
}
#[server]
async fn save_dog2(image: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

// Expose a `save_dog` endpoint on our server that takes an "image" parameter
#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    // Open the `dogs.txt` file in append-only mode, creating it if it doesn't exist;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    // And then write a newline to it with the image url
    file.write_fmt(format_args!("{image}\n"));
    tracing::info!("Server: Saving dog image to file");
    Ok(())
}

//endregion server function

#[derive(Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogApp() -> Element {
    // State for the dog image URL using use_resource
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    // Using use_hook for timestamp
    let time_now = use_hook(|| {
        // return current timestamp
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    });

    // Create a reusable async function
    async fn fetch_and_save(img_src: &mut Resource<String>, skip_save: bool) {
        let current = img_src.cloned().unwrap();
        img_src.restart();
        if !skip_save {
            _ = save_dog2(current).await;
        }
    }

    // Use it in both handlers
    let skip = move |_| async move {
        tracing::info!("Skip button clicked");
        fetch_and_save(&mut img_src, true).await;
    };

    let save = move |_| async move {
        tracing::info!("Save button clicked");
        fetch_and_save(&mut img_src, false).await;
    };

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default(), id: "dog" }
        }
        div { "Current time: {time_now}" }
        button { id: "fetch", onclick: move |_| img_src.restart(), "fetch me!!!" }
        div { id: "buttons",
            button { id: "skip", onclick: skip, "skip" }
            button { id: "save", onclick: save, "save!" }
        }
    }
}
