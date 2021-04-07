use std::iter::FromIterator;
use std::ops::Index;

use gdk_pixbuf::{Colorspace, Pixbuf};
use glib::Bytes;
use image::RgbaImage;

/// The television program consisiting of many channels and their movie
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    content: Vec<(Channel, Movie)>,
}

/// A channel must have a name and a optional icon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Channel {
    name: String,
    icon: Option<RgbaImage>,
}

/// A movie must have a title, a optional year, genre, division and description.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Movie {
    title: String,
    year: Option<u32>,
    genre: Option<String>,
    division: Option<String>,
    description: Option<String>,
}

/// Build movies.
pub struct MovieBuilder {
    movie: Movie,
}

impl Program {
    /// Create a new, empty program.
    pub fn new() -> Self {
        Program { content: vec![] }
    }

    /// Add a channel and its movie to the program.
    pub fn add(&mut self, channel: Channel, movie: Movie) {
        self.content.push((channel, movie));
    }

    /// Turn the program into a iterator over the channels and their movies.
    pub fn iter(&self) -> std::slice::Iter<'_, (Channel, Movie)> {
        self.content.iter()
    }
}

impl Index<usize> for Program {
    type Output = (Channel, Movie);

    /// A program can be indexed.
    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}

impl FromIterator<(Channel, Movie)> for Program {
    /// Convert a iterator over `(Channel, Movie)` into a program.
    fn from_iter<I: IntoIterator<Item = (Channel, Movie)>>(iter: I) -> Self {
        let content = iter.into_iter().collect();
        Program { content }
    }
}

impl Channel {
    /// Create a new `Channel` with the given name and no icon.
    pub fn new(name: &str) -> Self {
        Channel {
            name: name.to_string(),
            icon: None,
        }
    }

    /// Get the name of the `Channel`.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Set the ivon of the `Channel`.
    pub fn set_icon(&mut self, icon: Option<RgbaImage>) {
        self.icon = icon;
    }

    /// Get the icon of the `Channel` as a `gdk_pixbuf::Pixbuf`.
    pub fn get_icon_as_pixbuf(&self) -> Option<Pixbuf> {
        if let Some(icon) = &self.icon {
            let bytes = icon.clone().into_raw();
            Some(Pixbuf::from_bytes(
                &Bytes::from(&bytes),
                Colorspace::Rgb,
                true,
                8,
                icon.width() as i32,
                icon.height() as i32,
                4 * icon.width() as i32,
            ))
        } else {
            None
        }
    }
}

impl Movie {
    /// Create a new `Movie` with the given title. All other attributes are not set.
    pub fn new(title: &str) -> Self {
        Movie {
            title: title.to_string(),
            year: None,
            genre: None,
            division: None,
            description: None,
        }
    }

    /// Get the title.
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    /// Get the optional year.
    pub fn get_year(&self) -> Option<u32> {
        self.year
    }

    /// Set the optional year.
    pub fn set_year(&mut self, year: Option<u32>) {
        self.year = year
    }

    /// Get the optional genre.
    pub fn get_genre(&self) -> Option<String> {
        self.genre.clone()
    }

    /// Set the optional genre.
    pub fn set_genre(&mut self, genre: Option<String>) {
        self.genre = genre
    }

    /// Get the optional division.
    pub fn get_division(&self) -> Option<String> {
        self.division.clone()
    }

    /// Set the optional division.
    pub fn set_division(&mut self, disision: Option<String>) {
        self.division = disision
    }

    /// Get the optional description.
    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }

    /// Set the optional description.
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description
    }
}

impl MovieBuilder {
    /// Create a `MovieBuilder` building a `Movie` with the given title.
    pub fn new(title: &str) -> Self {
        MovieBuilder {
            movie: Movie::new(title),
        }
    }

    /// Set the year of the `Movie`.
    pub fn with_year(&mut self, year: u32) -> &mut Self {
        self.movie.set_year(Some(year));
        self
    }

    /// Set the genre of the `Movie`.
    pub fn with_genre(&mut self, genre: &str) -> &mut Self {
        self.movie.set_genre(Some(genre.to_string()));
        self
    }

    /// Set the division of the `Movie`.
    pub fn with_division(&mut self, division: &str) -> &mut Self {
        self.movie.set_division(Some(division.to_string()));
        self
    }

    /// Build the `Movie`.
    pub fn build(self) -> Movie {
        self.movie
    }
}
