use crate::model::{Channel, Movie, Program};

pub trait Filter<T> {
    fn matches(&self, item: &T) -> bool;
}

pub trait FilterGroup<T> {
    fn new() -> Self
    where
        Self: Sized;
    fn add<F: 'static + Filter<T>>(&mut self, filter: F);
    fn matches(&self, iterm: &T) -> bool;
}

#[derive(Clone)]
pub enum ChannelAttribute {
    Name(String),
}

impl Filter<Channel> for ChannelAttribute {
    fn matches(&self, channel: &Channel) -> bool {
        match self {
            ChannelAttribute::Name(name) => name.to_string() == channel.get_name(),
        }
    }
}

#[derive(Clone)]
pub enum MovieAttribute {
    Title(String),
    Genre(String),
    Division(String),
}

impl Filter<Movie> for MovieAttribute {
    fn matches(&self, movie: &Movie) -> bool {
        match self {
            MovieAttribute::Title(title) => title.to_string() == movie.get_title(),
            MovieAttribute::Genre(genre) => Some(genre.to_string()) == movie.get_genre(),
            MovieAttribute::Division(division) => {
                Some(division.to_string()) == movie.get_division()
            }
        }
    }
}

pub enum FilterType {
    Channel(ChannelAttribute),
    Movie(MovieAttribute),
}

pub struct Filters<T> {
    filters: Vec<Box<dyn Filter<T>>>,
}

impl<T> FilterGroup<T> for Filters<T> {
    fn new() -> Self {
        Filters { filters: vec![] }
    }

    fn add<F: 'static + Filter<T>>(&mut self, filter: F) {
        self.filters.push(Box::new(filter));
    }

    fn matches(&self, item: &T) -> bool {
        self.filters.iter().any(|f| f.matches(item))
    }
}

pub struct ProgramFilter {
    channel_filters: Filters<Channel>,
    movie_filters: Filters<Movie>,
}

impl ProgramFilter {
    pub fn new() -> Self {
        ProgramFilter {
            channel_filters: Filters::new(),
            movie_filters: Filters::new(),
        }
    }

    pub fn add(&mut self, filter: FilterType) {
        match filter {
            FilterType::Channel(c) => self.add_channel_filter(c),
            FilterType::Movie(m) => self.add_movie_filter(m),
        }
    }

    pub fn add_channel_filter<F: 'static + Filter<Channel>>(&mut self, filter: F) {
        self.channel_filters.add(filter)
    }

    pub fn add_movie_filter<F: 'static + Filter<Movie>>(&mut self, filter: F) {
        self.movie_filters.add(filter)
    }

    pub fn matches(&self, (channel, movie): (&Channel, &Movie)) -> bool {
        self.channel_filters.matches(channel) || self.movie_filters.matches(movie)
    }

    pub fn filter(&self, program: &Program) -> Program {
        program
            .iter()
            .filter(|(c, m)| !self.matches((c, m)))
            .cloned()
            .collect()
    }
}
