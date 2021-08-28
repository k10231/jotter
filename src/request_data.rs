use isahc::{prelude::*, Request};

pub fn get_data() -> Result<String, isahc::Error> {
    let mut response =
        Request::get("https://hurrxycxigvviayjhlxr.supabase.co/rest/v1/just-stuffs?select=stuff")
            .header("apikey", "...")
            .header("Authorization", "...")
            .body(r#""#,)?
            .send()?;
    let data = response.text().unwrap().replace("[", "").replace("]", "");
    Ok(data)
}
