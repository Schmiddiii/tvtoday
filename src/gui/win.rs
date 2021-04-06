use crate::gui::{MovieList, MovieListMsg, MoviePage, MoviePageMsg};
use crate::model::{Channel, FilterType, Movie, Provider};

use gtk::prelude::*;
use gtk::{Box, Inhibit};
use libhandy::prelude::*;
use libhandy::{Leaflet, Window};
use relm::{connect, Component, Relm, StreamHandle, Update, Widget};
use relm_derive::Msg;

#[derive(Msg)]
pub enum WinMsg<T: 'static + Provider> {
    SelectedMovie((Channel, Movie)),
    UpdateProvider(T),
    AddFilter(FilterType),
    Quit,
}

pub struct WinModel<T: 'static + Provider> {
    provider: T,

    stream_win: StreamHandle<WinMsg<T>>,
}

pub struct Win<T: 'static + Provider> {
    model: WinModel<T>,
    widgets: WinWidgets,
    components: WinComponents<T>,
}

struct WinWidgets {
    root: Window,
    leaflet: Leaflet,
    page_movie: Box,
}

struct WinComponents<T: 'static + Provider> {
    page_list: Component<MovieList<T>>,
    page_movie: Component<MoviePage<T>>,
}

impl<T: 'static + Provider> Update for Win<T> {
    type Model = WinModel<T>;
    type ModelParam = ();
    type Msg = WinMsg<T>;

    fn model(relm: &Relm<Self>, _: Self::ModelParam) -> Self::Model {
        WinModel {
            provider: T::new(),
            stream_win: relm.stream().clone(),
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            WinMsg::SelectedMovie((channel, movie)) => {
                self.components
                    .page_movie
                    .emit(MoviePageMsg::Set((channel, movie)));
                self.widgets
                    .leaflet
                    .set_visible_child(&self.widgets.page_movie);
            }
            WinMsg::UpdateProvider(provider) => {
                self.model.provider = provider.clone();

                self.components
                    .page_movie
                    .emit(MoviePageMsg::SetProvider(provider));
            }
            WinMsg::AddFilter(filter) => self
                .components
                .page_list
                .emit(MovieListMsg::AddFilter(filter)),
            WinMsg::Quit => gtk::main_quit(),
        }
    }
}

impl<T: 'static + Provider> Widget for Win<T> {
    type Root = Window;
    fn root(&self) -> Self::Root {
        self.widgets.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let root = Window::new();

        let leaflet = Leaflet::new();
        leaflet.set_can_swipe_back(true);

        let page_list = relm::create_component::<MovieList<T>>((
            model.stream_win.clone(),
            model.provider.clone(),
        ));
        let page_movie = relm::create_component::<MoviePage<T>>(model.stream_win.clone());

        page_list.widget().set_size_request(360, -1);
        page_movie.widget().set_size_request(360, -1);

        leaflet.add(page_list.widget());
        leaflet.add(page_movie.widget());

        root.add(&leaflet);

        connect!(
            relm,
            root,
            connect_delete_event(_, _),
            return (WinMsg::Quit, Inhibit(false))
        );

        root.show_all();

        let widgets = WinWidgets {
            root,
            leaflet,
            page_movie: page_movie.widget().clone(),
        };

        let components = WinComponents {
            page_list: page_list,
            page_movie,
        };

        Win {
            model,
            widgets,
            components,
        }
    }
}
