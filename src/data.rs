use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum StreamingProviders {
    Netflix,
    DisneyPlus,
    Hulu,
    PrimeVideo,
    YouTubePremium,
    DiscoveryPlus,
    ParamountPlus,
    Peacock,
    HBOMax
}

#[derive(Serialize, Deserialize)]
pub struct Actor {
    name: String,
    birthdate: String,
    deathdate: Option<String>,
    movies_acted_in: Vec<Movie>,
    is_deceased: bool,
}

impl Actor {
    pub fn new(
        name: String,
        birthdate: String,
        deathdate: Option<String>,
        movies_acted_in: Vec<Movie>,
        is_deceased: bool,
    ) -> Self {
        Self {
            name,
            birthdate,
            deathdate,
            movies_acted_in,
            is_deceased,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Movie {
    title: String,
    release_date: String,
    rating: f32,
    actor_list: Vec<Actor>,
    length: String,
    movie_poster: String, // will most likely point towards a URL or path
    where_to_watch: Vec<StreamingProviders>,
    similar_titles: Vec<Movie>
}

impl Movie {
    pub fn new(
        title: String,
        release_date: String,
        rating: f32,
        actor_list: Vec<Actor>,
        length: String,
        movie_poster: String, // will most likely point towards a URL or path
        where_to_watch: Vec<StreamingProviders>,
        similar_titles: Vec<Movie>
    ) -> Self {
        Self {
            title,
            release_date,
            rating,
            actor_list,
            length,
            movie_poster, // will most likely point towards a URL or path
            where_to_watch,
            similar_titles
        }
    }
}
