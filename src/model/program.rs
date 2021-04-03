use std::ops::Index;

use gdk_pixbuf::{Colorspace, Pixbuf};
use glib::Bytes;
use image::RgbaImage;

#[derive(Debug, Clone)]
pub struct Program {
    content: Vec<(Channel, Movie)>,
}

#[derive(Debug, Clone)]
pub struct Channel {
    name: String,
    icon: Option<RgbaImage>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Movie {
    title: String,
    year: Option<u32>,
    genre: Option<String>,
    division: Option<String>,
    description: Option<String>,
}

pub struct MovieBuilder {
    movie: Movie,
}

impl Program {
    pub fn new() -> Self {
        Program { content: vec![] }
    }

    pub fn add(&mut self, channel: Channel, movie: Movie) {
        self.content.push((channel, movie));
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (Channel, Movie)> {
        self.content.iter()
    }
}

impl Index<usize> for Program {
    type Output = (Channel, Movie);

    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}

impl Channel {
    pub fn new(name: &str) -> Self {
        Channel {
            name: name.to_string(),
            icon: None,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_icon(&mut self, icon: Option<RgbaImage>) {
        self.icon = icon;
    }

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
    pub fn new(title: &str) -> Self {
        Movie {
            title: title.to_string(),
            year: None,
            genre: None,
            division: None,
            description: None,
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_year(&self) -> Option<u32> {
        self.year
    }

    pub fn set_year(&mut self, year: Option<u32>) {
        self.year = year
    }

    pub fn get_genre(&self) -> Option<String> {
        self.genre.clone()
    }

    pub fn set_genre(&mut self, genre: Option<String>) {
        self.genre = genre
    }

    pub fn get_division(&self) -> Option<String> {
        self.division.clone()
    }

    pub fn set_division(&mut self, disision: Option<String>) {
        self.division = disision
    }

    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description
    }
}

impl MovieBuilder {
    pub fn new(title: &str) -> Self {
        MovieBuilder {
            movie: Movie::new(title),
        }
    }

    pub fn with_year(&mut self, year: u32) -> &mut Self {
        self.movie.set_year(Some(year));
        self
    }

    pub fn with_genre(&mut self, genre: &str) -> &mut Self {
        self.movie.set_genre(Some(genre.to_string()));
        self
    }

    pub fn with_division(&mut self, division: &str) -> &mut Self {
        self.movie.set_division(Some(division.to_string()));
        self
    }

    pub fn build(self) -> Movie {
        self.movie
    }
}
