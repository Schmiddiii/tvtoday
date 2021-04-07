use crate::model::{Channel, Movie, Program};

use std::convert::TryFrom;
use std::iter::FromIterator;
use std::marker::PhantomData;

pub trait Filter<T> {
    fn matches(&self, item: &T) -> bool;
}

pub trait FilterGroup<T, F: Filter<T>> {
    fn new() -> Self
    where
        Self: Sized;
    fn add(&mut self, filter: F);
    fn matches(&self, iterm: &T) -> bool;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChannelAttribute {
    Name(String),
}

impl From<ChannelAttribute> for [String; 2] {
    fn from(item: ChannelAttribute) -> [String; 2] {
        match item {
            ChannelAttribute::Name(name) => ["name".to_string(), name],
        }
    }
}

impl TryFrom<[String; 2]> for ChannelAttribute {
    type Error = ();

    fn try_from(item: [String; 2]) -> Result<ChannelAttribute, ()> {
        match &item[0][..] {
            "name" => Ok(ChannelAttribute::Name(item[1].clone())),
            _ => Err(()),
        }
    }
}

impl Filter<Channel> for ChannelAttribute {
    fn matches(&self, channel: &Channel) -> bool {
        match self {
            ChannelAttribute::Name(name) => name.to_string() == channel.get_name(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MovieAttribute {
    Title(String),
    Genre(String),
    Division(String),
}

impl TryFrom<[String; 2]> for MovieAttribute {
    type Error = ();

    fn try_from(item: [String; 2]) -> Result<MovieAttribute, ()> {
        match &item[0][..] {
            "title" => Ok(MovieAttribute::Title(item[1].clone())),
            "genre" => Ok(MovieAttribute::Genre(item[1].clone())),
            "division" => Ok(MovieAttribute::Division(item[1].clone())),
            _ => Err(()),
        }
    }
}

impl From<MovieAttribute> for [String; 2] {
    fn from(item: MovieAttribute) -> [String; 2] {
        match item {
            MovieAttribute::Title(title) => ["title".to_string(), title],
            MovieAttribute::Genre(genre) => ["genre".to_string(), genre],
            MovieAttribute::Division(division) => ["division".to_string(), division],
        }
    }
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

#[derive(PartialEq, Eq, Debug)]
pub struct Filters<T, F: Filter<T>> {
    filters: Vec<F>,
    phantom: PhantomData<T>,
}

impl<T: Clone, F: Filter<T> + Clone> Clone for Filters<T, F> {
    fn clone(&self) -> Self {
        Filters {
            filters: self.filters.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T, F: Filter<T> + TryFrom<[String; 2], Error = ()>> TryFrom<Vec<[String; 2]>>
    for Filters<T, F>
{
    type Error = ();

    fn try_from(item: Vec<[String; 2]>) -> Result<Filters<T, F>, ()> {
        let results: Vec<Result<F, ()>> = item.into_iter().map(|i| F::try_from(i)).collect();

        if results.iter().any(|r| r.is_err()) {
            Err(())
        } else {
            Ok(Filters {
                filters: results.into_iter().map(|r| r.unwrap()).collect(),
                phantom: PhantomData,
            })
        }
    }
}

impl<T, F: Filter<T>> FromIterator<F> for Filters<T, F> {
    fn from_iter<I: IntoIterator<Item = F>>(iter: I) -> Self {
        let vector = iter.into_iter().collect();
        Filters {
            filters: vector,
            phantom: PhantomData,
        }
    }
}

impl<T, F: Filter<T> + Clone + Into<[String; 2]>> From<Filters<T, F>> for Vec<[String; 2]> {
    fn from(item: Filters<T, F>) -> Vec<[String; 2]> {
        item.filters.iter().map(|f| f.clone().into()).collect()
    }
}

impl<T, F: Filter<T>> FilterGroup<T, F> for Filters<T, F> {
    fn new() -> Self {
        Filters {
            filters: vec![],
            phantom: PhantomData,
        }
    }

    fn add(&mut self, filter: F) {
        self.filters.push(filter);
    }

    fn matches(&self, item: &T) -> bool {
        self.filters.iter().any(|f| f.matches(item))
    }
}

pub enum FilterType {
    Channel(ChannelAttribute),
    Movie(MovieAttribute),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProgramFilter {
    channel_filters: Filters<Channel, ChannelAttribute>,
    movie_filters: Filters<Movie, MovieAttribute>,
}

impl TryFrom<Vec<[String; 2]>> for ProgramFilter {
    type Error = ();

    fn try_from(item: Vec<[String; 2]>) -> Result<ProgramFilter, ()> {
        let filters: Vec<(Result<ChannelAttribute, ()>, Result<MovieAttribute, ()>)> = item
            .into_iter()
            .map(|i| {
                (
                    ChannelAttribute::try_from(i.clone()),
                    MovieAttribute::try_from(i),
                )
            })
            .collect();

        if filters
            .iter()
            .any(|(c, m)| c.is_err() && m.is_err() || c.is_ok() && m.is_ok())
        {
            return Err(());
        } else {
            let channel_filters = filters
                .iter()
                .cloned()
                .map(|(c, _m)| c)
                .filter(|c| c.is_ok())
                .map(|c| c.unwrap())
                .collect();
            let movie_filters = filters
                .iter()
                .cloned()
                .map(|(_c, m)| m)
                .filter(|m| m.is_ok())
                .map(|m| m.unwrap())
                .collect();

            return Ok(ProgramFilter {
                channel_filters,
                movie_filters,
            });
        }
    }
}

impl From<ProgramFilter> for Vec<[String; 2]> {
    fn from(item: ProgramFilter) -> Vec<[String; 2]> {
        let mut channels: Vec<[String; 2]> = item.channel_filters.clone().into();
        let mut movies: Vec<[String; 2]> = item.movie_filters.clone().into();

        channels.append(&mut movies);

        channels
    }
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

    pub fn add_channel_filter(&mut self, filter: ChannelAttribute) {
        self.channel_filters.add(filter)
    }

    pub fn add_movie_filter(&mut self, filter: MovieAttribute) {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_str_array() {
        assert_eq!(
            ["title".to_string(), "Hello".to_string()],
            <[String; 2]>::from(MovieAttribute::Title("Hello".to_string()))
        );

        assert_eq!(
            ["genre".to_string(), "Bye".to_string()],
            <[String; 2]>::from(MovieAttribute::Genre("Bye".to_string()))
        );

        assert_eq!(
            ["division".to_string(), "World".to_string()],
            <[String; 2]>::from(MovieAttribute::Division("World".to_string()))
        );

        assert_eq!(
            ["name".to_string(), "World2".to_string()],
            <[String; 2]>::from(ChannelAttribute::Name("World2".to_string()))
        );
    }

    #[test]
    fn test_to_vec_str_array() {
        let mut filter = Filters::new();
        filter.add(MovieAttribute::Title("Hello".to_string()));
        filter.add(MovieAttribute::Genre("Bye".to_string()));
        filter.add(MovieAttribute::Division("World".to_string()));

        assert_eq!(
            vec![
                ["title".to_string(), "Hello".to_string()],
                ["genre".to_string(), "Bye".to_string()],
                ["division".to_string(), "World".to_string()],
            ],
            <Vec<[String; 2]>>::from(filter)
        );
    }
    #[test]
    fn test_program_to_vec_str_array() {
        let mut filter = ProgramFilter::new();
        filter.add_movie_filter(MovieAttribute::Title("Hello".to_string()));
        filter.add_movie_filter(MovieAttribute::Genre("Bye".to_string()));
        filter.add_movie_filter(MovieAttribute::Division("World".to_string()));
        filter.add_channel_filter(ChannelAttribute::Name("World2".to_string()));

        assert_eq!(
            vec![
                ["name".to_string(), "World2".to_string()],
                ["title".to_string(), "Hello".to_string()],
                ["genre".to_string(), "Bye".to_string()],
                ["division".to_string(), "World".to_string()],
            ],
            <Vec<[String; 2]>>::from(filter)
        );
    }

    #[test]
    fn test_from_str_array() {
        assert_eq!(
            MovieAttribute::try_from(["title".to_string(), "Hello".to_string()]),
            Ok(MovieAttribute::Title("Hello".to_string()))
        );

        assert_eq!(
            MovieAttribute::try_from(["genre".to_string(), "Bye".to_string()]),
            Ok(MovieAttribute::Genre("Bye".to_string()))
        );

        assert_eq!(
            MovieAttribute::try_from(["division".to_string(), "World".to_string()]),
            Ok(MovieAttribute::Division("World".to_string()))
        );

        assert_eq!(
            ChannelAttribute::try_from(["name".to_string(), "World2".to_string()]),
            Ok(ChannelAttribute::Name("World2".to_string()))
        );

        assert_eq!(
            ChannelAttribute::try_from(["nan".to_string(), "World2".to_string()]),
            Err(())
        );
    }

    #[test]
    fn test_program_from_vec_str_array() {
        let mut program_filter = ProgramFilter::new();
        program_filter.add_channel_filter(ChannelAttribute::Name("World2".to_string()));
        program_filter.add_movie_filter(MovieAttribute::Title("Hello".to_string()));
        program_filter.add_movie_filter(MovieAttribute::Genre("Bye".to_string()));
        program_filter.add_movie_filter(MovieAttribute::Division("World".to_string()));

        assert_eq!(
            vec![
                ["name".to_string(), "World2".to_string()],
                ["title".to_string(), "Hello".to_string()],
                ["genre".to_string(), "Bye".to_string()],
                ["division".to_string(), "World".to_string()],
            ]
            .try_into(),
            Ok(program_filter)
        );
    }
}
