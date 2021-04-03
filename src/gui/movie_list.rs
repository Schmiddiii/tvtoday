use crate::gui::{MovieListItem, WinMsg};
use crate::model::{Program, Provider};
use crate::Error;

use std::thread;

use gtk::prelude::*;
use gtk::{
    Adjustment, Box, Button, ListBox, ListBoxRow, Orientation, ScrolledWindow, SelectionMode,
    Viewport,
};
use libhandy::{HeaderBar, HeaderBarExt};
use relm::{connect, Component, ContainerWidget, Relm, StreamHandle, Update, Widget};
use relm_derive::Msg;
use tokio::runtime::Runtime;

#[derive(Msg)]
pub enum MovieListMsg<T: 'static + Provider> {
    Reload,
    ReloadFinished((T, Result<Program, Error>)),
    RowActivated(ListBoxRow),
}

pub struct MovieListModel<T: 'static + Provider> {
    program: Program,
    provider: T,

    movies: Vec<Component<MovieListItem>>,

    stream_win: StreamHandle<WinMsg<T>>,
    relm: Relm<MovieList<T>>,
}

pub struct MovieList<T: 'static + Provider> {
    model: MovieListModel<T>,
    widgets: MovieListWidgets,
}

struct MovieListWidgets {
    root: Box,
    listbox: ListBox,
}

impl<T: 'static + Provider> Update for MovieList<T> {
    type Model = MovieListModel<T>;
    type ModelParam = (StreamHandle<WinMsg<T>>, T);
    type Msg = MovieListMsg<T>;

    fn model(relm: &Relm<Self>, (stream_win, provider): Self::ModelParam) -> MovieListModel<T> {
        relm.stream().emit(MovieListMsg::Reload);
        MovieListModel {
            program: Program::new(),
            provider,

            movies: vec![],

            stream_win,
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: MovieListMsg<T>) {
        match event {
            MovieListMsg::Reload => {
                let stream = self.model.relm.stream().clone();

                let (_channel, sender) = relm::Channel::new(move |result| {
                    stream.emit(MovieListMsg::ReloadFinished(result))
                });

                let mut provider = self.model.provider.clone();

                thread::spawn(move || {
                    let rt = Runtime::new().expect("Could not create runtime");
                    let program = rt.block_on(provider.get_program());
                    sender.send((provider, program)).unwrap()
                });
            }
            MovieListMsg::ReloadFinished((provider, program_res)) => {
                if let Ok(program) = program_res {
                    self.model.program = program;
                    self.reset_movies();
                } else {
                    self.model.program = Program::new();
                    self.reset_movies();
                }
                self.model.provider = provider.clone();
                self.model.stream_win.emit(WinMsg::UpdateProvider(provider));
            }
            MovieListMsg::RowActivated(row) => {
                let index = self
                    .widgets
                    .listbox
                    .get_children()
                    .iter()
                    .position(|x| x.clone() == row)
                    .unwrap();

                let movie = &self.model.program[index];
                self.model
                    .stream_win
                    .emit(WinMsg::SelectedMovie(movie.clone()));
            }
        }
    }
}

impl<T: 'static + Provider> Widget for MovieList<T> {
    type Root = Box;

    fn root(&self) -> Self::Root {
        self.widgets.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let root = Box::new(Orientation::Vertical, 0);
        root.set_hexpand(true);

        let header_bar = HeaderBar::new();
        header_bar.set_title(Some("Movies"));

        let button_reload = Button::new();
        button_reload.set_label("Reload");
        connect!(
            relm,
            button_reload,
            connect_clicked(_),
            MovieListMsg::Reload
        );

        header_bar.add(&button_reload);

        root.add(&header_bar);

        let scrolled_window = ScrolledWindow::new::<Adjustment, Adjustment>(None, None);
        scrolled_window.set_hexpand(true);
        scrolled_window.set_vexpand(true);

        let viewport = Viewport::new::<Adjustment, Adjustment>(None, None);

        scrolled_window.add(&viewport);

        let listbox = ListBox::new();
        listbox.set_selection_mode(SelectionMode::None);

        viewport.add(&listbox);

        root.add(&scrolled_window);

        connect!(
            relm,
            listbox,
            connect_row_activated(_, row),
            MovieListMsg::RowActivated(row.clone())
        );

        root.show_all();

        let widgets = MovieListWidgets { root, listbox };
        Self { model, widgets }
    }
}

impl<T: Provider> MovieList<T> {
    fn reset_movies(&mut self) {
        let listbox = &mut self.widgets.listbox;

        let listbox_clone = listbox.clone();
        listbox.foreach(|c| listbox_clone.remove(c));

        for data in self.model.program.iter() {
            let component = listbox.add_widget::<MovieListItem>(data.clone());
            self.model.movies.push(component);
        }
    }
}
