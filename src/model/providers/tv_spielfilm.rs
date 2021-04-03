use crate::model::{Channel, Movie, MovieBuilder, Program, Provider};
use crate::Error;

use std::collections::HashMap;

use async_trait::async_trait;
use image::imageops;
use scraper::{Html, Selector};
use webp::Decoder;

const URL: &str = "https://www.tvspielfilm.de/tv-programm/sendungen/abends.html";
const ICONS_URL: &str =
    "https://a2.tvspielfilm.de/images/tv/sender/mini/sprite_web_optimized_1616508904.webp";

const ICON_SIZE: u32 = 44;

/// Represents the order of the icons on the icon image.
const ICON_IMAGE_LIST: &[&str] = &[
    "Das Erste",
    "ZDF",
    "RTL",
    "SAT.1",
    "ProSieben",
    "kabel eins",
    "RTL II",
    "VOX",
    "TELE 5",
    "3sat",
    "ARTE",
    "ZDFneo",
    "ONE",
    "ServusTV Deutschland",
    "NITRO",
    "DMAX",
    "sixx",
    "SAT.1 Gold",
    "ProSieben MAXX",
    "COMEDY CENTRAL",
    "RTLplus",
    "WDR",
    "NDR",
    "BR",
    "SWR/SR",
    "HR",
    "MDR",
    "RBB",
    "tv.berlin",
];

pub struct TvSpielfilm {
    /// Maps each movie to a URL with more information (e.g. description).
    more_information_urls: HashMap<Movie, String>,
}

#[async_trait]
impl Provider for TvSpielfilm {
    fn new() -> Self {
        TvSpielfilm {
            more_information_urls: HashMap::new(),
        }
    }

    fn clone(&self) -> Self {
        TvSpielfilm {
            more_information_urls: self.more_information_urls.clone(),
        }
    }

    async fn get_program(&mut self) -> Result<Program, Error> {
        // Get the contents of the website and the image of icons.
        let html = reqwest::get(URL).await?.text().await?;

        let image_icons: &[u8] = &reqwest::get(ICONS_URL).await?.bytes().await?;

        let document = Html::parse_document(&html);

        // The selectors to get the movie and channel data.
        let selector_list_rows = Selector::parse("body #wrapper #main .content-area #content .tvlistings .content-holder .tab-content .info-table tbody .hover").expect("failed to parse selector for list row");
        let selector_channel_name =
            Selector::parse(".programm-col1 a").expect("failed to parse selector for channel name");
        let selector_movie_title = Selector::parse(".col-3 span a strong")
            .expect("failed to parse selector for movie title");
        let selector_movie_genre =
            Selector::parse(".col-4 span").expect("failed to parse selector for movie genre");
        let selector_movie_division =
            Selector::parse(".col-5 span").expect("failed to parse selector for movie division");
        let selector_movie_year =
            Selector::parse(".col-3 span a").expect("failed to parse selector for movie year");
        let selector_movie_information = selector_movie_year.clone();

        // Decode image of icons.
        let image = Decoder::new(image_icons)
            .decode()
            .expect("could not decode icons image")
            .as_image();

        let mut image_rgba8 = image.into_rgba8();

        // Create the program.
        let mut program = Program::new();
        for row in document.select(&selector_list_rows) {
            // The channel name.
            let channel_str_opt = row.select(&selector_channel_name).next();
            if channel_str_opt.is_none() {
                return Err(Error::ParsingWebsite);
            }
            let mut channel_str = channel_str_opt.unwrap().value().attr("title").unwrap();

            // Remove trailing " Program".
            if channel_str.ends_with(" Programm") {
                channel_str = &channel_str[0..channel_str.len() - 9];
            }

            // The title of the movie.
            let title_str_opt = row.select(&selector_movie_title).next();
            if title_str_opt.is_none() {
                return Err(Error::ParsingWebsite);
            }
            let title_str = title_str_opt.unwrap().inner_html();

            // Create the channel and the movie.
            let mut channel = Channel::new(&channel_str);
            let mut movie_builder = MovieBuilder::new(&title_str);

            // Get the icon for the channel if available.
            let index_in_image = ICON_IMAGE_LIST.iter().position(|c| c == &channel_str);
            if let Some(index) = index_in_image {
                let channel_icon = imageops::crop(
                    &mut image_rgba8,
                    0,
                    index as u32 * ICON_SIZE,
                    ICON_SIZE,
                    ICON_SIZE,
                )
                .to_image();
                channel.set_icon(Some(channel_icon));
            }

            // Get the genre of the movie.
            let genre_str_opt = row.select(&selector_movie_genre).next();
            if let Some(genre_str) = genre_str_opt {
                movie_builder.with_genre(genre_str.inner_html().trim());
            }

            // Get the division of the movie.
            let division_str_opt = row.select(&selector_movie_division).next();
            if let Some(division_str) = division_str_opt {
                movie_builder.with_division(
                    division_str
                        .inner_html()
                        .trim()
                        .split(" ")
                        .collect::<Vec<&str>>()
                        .first()
                        .unwrap(),
                );
            }

            // Get the year of the movie.
            let year_str_opt = row.select(&selector_movie_year).next();
            if let Some(year_str) = year_str_opt {
                if let Ok(year) = year_str
                    .value()
                    .attr("title")
                    .unwrap_or("")
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse()
                {
                    movie_builder.with_year(year);
                }
            }

            let movie = movie_builder.build();
            // The more information page.
            let information_str_opt = row.select(&selector_movie_information).next();
            if let Some(information_str) = information_str_opt {
                if let Some(href) = information_str.value().attr("href") {
                    self.more_information_urls
                        .insert(movie.clone(), href.to_string());
                }
            }

            program.add(channel, movie);
        }

        Ok(program)
    }

    async fn get_more_information(&self, movie: &Movie) -> Movie {
        if let Some(more_information_url) = self.more_information_urls.get(movie) {
            // Get the contents of the website.
            let html_result1 = reqwest::get(more_information_url).await;
            if html_result1.is_err() {
                return movie.clone();
            }

            let html_result2 = html_result1.unwrap().text().await;
            if html_result2.is_err() {
                return movie.clone();
            }
            let html = html_result2.unwrap();

            let document = Html::parse_document(&html);

            let selector_description =
                Selector::parse("#content div div article section.broadcast-detail__description p")
                    .expect("failed to parse selector for movie description");

            // Get the description.
            let description = document
                .select(&selector_description)
                .map(|e| e.inner_html() + "\n\n")
                .collect();

            // Create a cloned movie ant manipulate it.
            let mut movie_clone = movie.clone();

            movie_clone.set_description(Some(description));

            movie_clone
        } else {
            movie.clone()
        }
    }
}
