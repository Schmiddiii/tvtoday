use crate::gui::{SlidingStack, SlidingStackMsg, WinMsg};
use crate::model::{Channel, ChannelAttribute, FilterType, Movie, MovieAttribute, Provider};

use std::thread;

use gtk::prelude::*;
use gtk::{Adjustment, Box, Button, Label, Orientation, ScrolledWindow};
use libhandy::{HeaderBar, HeaderBarExt};
use relm::{connect, Component, Relm, StreamHandle, Update, Widget};
use relm_derive::Msg;
use tokio::runtime::Runtime;

pub enum FilterList {
    ChannelName,
    MovieTitle,
    MovieGenre,
    MovieDivision,
}

#[derive(Msg)]
pub enum MoviePageMsg<T: 'static + Provider> {
    Filter(FilterList),
    SwitchStack,
    SetProvider(T),
    Set((Channel, Movie)),
    SetMovie(Movie),
}

pub struct MoviePageModel<T: 'static + Provider> {
    channel: Channel,
    movie: Movie,

    provider: T,

    relm: Relm<MoviePage<T>>,
    win_stream: StreamHandle<WinMsg<T>>,
}

pub struct MoviePage<T: 'static + Provider> {
    model: MoviePageModel<T>,
    widgets: MoviePageWidgets,
    components: MoviePageComponents,
}

pub struct MoviePageWidgets {
    root: Box,
    header_bar: HeaderBar,
    label_channel_name: Label,
    label_movie_genre: Label,
    label_movie_division: Label,
    label_movie_year: Label,
    label_movie_description: Label,
}

pub struct MoviePageComponents {
    stack: Component<SlidingStack<Box, ScrolledWindow>>,
}

impl<T: 'static + Provider> Update for MoviePage<T> {
    type Model = MoviePageModel<T>;
    type ModelParam = StreamHandle<WinMsg<T>>;
    type Msg = MoviePageMsg<T>;

    fn model(relm: &Relm<MoviePage<T>>, win_stream: Self::ModelParam) -> Self::Model {
        MoviePageModel {
            channel: Channel::new(""),
            movie: Movie::new(""),

            provider: T::new(),

            relm: relm.clone(),
            win_stream,
        }
    }

    fn update(&mut self, event: MoviePageMsg<T>) {
        match event {
            MoviePageMsg::Filter(item) => {
                self.components.stack.emit(SlidingStackMsg::ShowSecondPage);
                let filter;
                match item {
                    FilterList::ChannelName => {
                        filter = FilterType::Channel(ChannelAttribute::Name(
                            self.model.channel.get_name(),
                        ))
                    }
                    FilterList::MovieTitle => {
                        filter =
                            FilterType::Movie(MovieAttribute::Title(self.model.movie.get_title()))
                    }
                    FilterList::MovieGenre => {
                        let genre_opt = self.model.movie.get_genre();
                        if let Some(genre) = genre_opt {
                            filter = FilterType::Movie(MovieAttribute::Genre(genre))
                        } else {
                            return;
                        }
                    }
                    FilterList::MovieDivision => {
                        let division_opt = self.model.movie.get_division();
                        if let Some(division) = division_opt {
                            filter = FilterType::Movie(MovieAttribute::Division(division))
                        } else {
                            return;
                        }
                    }
                }

                self.model.win_stream.emit(WinMsg::AddFilter(filter));
            }
            MoviePageMsg::SwitchStack => {
                self.components.stack.emit(SlidingStackMsg::Switch);
            }
            MoviePageMsg::Set((channel, movie)) => {
                self.model.channel = channel;
                self.model.movie = movie.clone();

                // Get more information.
                let stream = self.model.relm.stream().clone();

                let (_channel, sender) =
                    relm::Channel::new(move |movie| stream.emit(MoviePageMsg::SetMovie(movie)));

                let provider = self.model.provider.clone();

                thread::spawn(move || {
                    let rt = Runtime::new().expect("Could not create runtime");
                    let information_movie = rt.block_on(provider.get_more_information(&movie));
                    sender.send(information_movie).unwrap()
                });
                self.show_all();
            }
            MoviePageMsg::SetMovie(movie) => {
                self.model.movie = movie;
                self.show_all();
            }
            MoviePageMsg::SetProvider(provider) => {
                self.model.provider = provider;
            }
        }
    }
}

impl<T: 'static + Provider> Widget for MoviePage<T> {
    type Root = Box;

    fn root(&self) -> Self::Root {
        self.widgets.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let root = Box::new(Orientation::Vertical, 0);

        let header_bar = HeaderBar::new();

        let button_switch_stack = Button::new();
        button_switch_stack.set_image(Some(&gtk::Image::from_icon_name(
            Some("open-menu-symbolic"),
            gtk::IconSize::Menu,
        )));
        connect!(
            relm,
            button_switch_stack,
            connect_clicked(_),
            MoviePageMsg::SwitchStack
        );

        header_bar.pack_end(&button_switch_stack);

        let scrolled_window = ScrolledWindow::new::<Adjustment, Adjustment>(None, None);
        let scrolled_window_box = Box::new(Orientation::Vertical, 0);

        scrolled_window.add(&scrolled_window_box);

        let label_channel_name = Label::new(None);
        let label_movie_genre = Label::new(None);
        let label_movie_division = Label::new(None);
        let label_movie_year = Label::new(None);
        let label_movie_description = Label::new(None);

        label_movie_description.set_line_wrap(true);

        scrolled_window_box.add(&label_channel_name);
        scrolled_window_box.add(&label_movie_genre);
        scrolled_window_box.add(&label_movie_division);
        scrolled_window_box.add(&label_movie_year);
        scrolled_window_box.add(&label_movie_description);

        scrolled_window.set_hexpand(true);
        scrolled_window.set_vexpand(true);

        root.add(&header_bar);

        let menu_box = gtk::Box::new(Orientation::Vertical, 0);

        let button_channel_name = Button::new();
        button_channel_name.set_label("Block channel name");
        connect!(
            relm,
            button_channel_name,
            connect_clicked(_),
            MoviePageMsg::Filter(FilterList::ChannelName)
        );

        let button_movie_title = Button::new();
        button_movie_title.set_label("Block movie title");
        connect!(
            relm,
            button_movie_title,
            connect_clicked(_),
            MoviePageMsg::Filter(FilterList::MovieTitle)
        );

        let button_movie_genre = Button::new();
        button_movie_genre.set_label("Block movie genre");
        connect!(
            relm,
            button_movie_genre,
            connect_clicked(_),
            MoviePageMsg::Filter(FilterList::MovieGenre)
        );

        let button_movie_division = Button::new();
        button_movie_division.set_label("Block movie division");
        connect!(
            relm,
            button_movie_division,
            connect_clicked(_),
            MoviePageMsg::Filter(FilterList::MovieDivision)
        );

        menu_box.add(&button_channel_name);
        menu_box.add(&button_movie_title);
        menu_box.add(&button_movie_genre);
        menu_box.add(&button_movie_division);

        let stack = relm::create_component::<SlidingStack<Box, ScrolledWindow>>((
            menu_box,
            scrolled_window.clone(),
        ));
        stack.emit(SlidingStackMsg::ShowSecondPage);

        root.add(stack.widget());

        root.show_all();

        let widgets = MoviePageWidgets {
            root,
            header_bar,
            label_channel_name,
            label_movie_genre,
            label_movie_division,
            label_movie_year,
            label_movie_description,
        };

        let components = MoviePageComponents { stack };

        MoviePage {
            model,
            widgets,
            components,
        }
    }
}

impl<T: 'static + Provider> MoviePage<T> {
    fn show_all(&self) {
        self.widgets
            .header_bar
            .set_title(Some(&self.model.movie.get_title()));
        self.widgets
            .label_channel_name
            .set_text(&self.model.channel.get_name());
        self.widgets
            .label_movie_genre
            .set_text(&self.model.movie.get_genre().unwrap_or("".to_string()));
        self.widgets
            .label_movie_division
            .set_text(&self.model.movie.get_division().unwrap_or("".to_string()));
        self.widgets
            .label_movie_description
            .set_text(&self.model.movie.get_description().unwrap_or("".to_string()));
        self.widgets.label_movie_year.set_text(
            &self
                .model
                .movie
                .get_year()
                .map(|v| v.to_string())
                .unwrap_or("".to_string()),
        );
    }
}
