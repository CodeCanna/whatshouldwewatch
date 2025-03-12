use whatshouldwewatch::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // let auth = RequestBuilder::from_parts(
    //     Client::new(),
    //     Request::new(
    //         Method::GET,
    //         Url::parse("https://api.themoviedb.org/3/search/movie?query=bean&include_adult=false&language=en-US&page=1")
    //         .unwrap(),
    //     ),
    // ).header("Authorization", "Bearer")
    // .header("accept", "application/json")
    // .send()
    // .await.unwrap();
    //
    // let json_object = json::from(auth.text().await.unwrap()).pretty(4);
    //
    // println!("{}", &json_object);
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
