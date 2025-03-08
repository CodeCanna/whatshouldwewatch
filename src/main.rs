use reqwest::{Client, Method, Request, RequestBuilder, Url};
use std::env;
use json::{self};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Movie {
    title: String,
    release_date: String,
    rating: f32,
    actor_list: ActorList, // TODO Create this datatype
    length: String,
    movie_poster: String, // will most likely point towards a URL or path
    where_to_watch: ProviderList, //TODO Creat this datatype
}

#[tokio::main]
async fn main() {
    let auth = RequestBuilder::from_parts(
        Client::new(),
        Request::new(
            Method::GET,
            Url::parse("https://api.themoviedb.org/3/search/movie?query=bean&include_adult=false&language=en-US&page=1")
            .unwrap(),
        ),
    ).header("Authorization", "Bearer")
    .header("accept", "application/json")
    .send()
    .await.unwrap();

    let json_object = json::from(auth.text().await.unwrap()).pretty(4);

    println!("{}", &json_object);
}
