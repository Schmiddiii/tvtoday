use crate::model::{Channel, Movie};

use gtk::prelude::*;
use pango::{AttrList, Attribute};
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

#[derive(Msg)]
pub enum MovieListItemMsg {}

pub struct MovieListItemModel {
    data: (Channel, Movie),
}

#[widget]
impl Widget for MovieListItem {
    fn model(_relm: &Relm<Self>, data: (Channel, Movie)) -> MovieListItemModel {
        MovieListItemModel { data }
    }

    fn update(&mut self, _event: MovieListItemMsg) {}

    fn init_view(&mut self) {
        let attr_list = AttrList::new();
        attr_list.insert(Attribute::new_size(12 * pango::SCALE).unwrap());

        self.widgets.label_channel.set_attributes(Some(&attr_list));
        self.widgets.label_movie.set_attributes(Some(&attr_list));

        let pixbuf_opt = self.model.data.0.get_icon_as_pixbuf();

        if let Some(pixbuf) = pixbuf_opt {
            self.widgets.icon_channel.set_from_pixbuf(Some(&pixbuf));

            self.widgets.label_channel.set_visible(false);
            self.widgets.icon_channel.set_visible(true);
        } else {
            self.widgets.icon_channel.set_visible(false);
            self.widgets.label_channel.set_visible(true);
        }
    }

    view! {
        gtk::ListBoxRow {
            #[name="box_content"]
            gtk::Box {
                spacing: 10,
                #[name="icon_channel"]
                gtk::Image {
                },
                #[name="label_channel"]
                gtk::Label {
                    label: &self.model.data.0.get_name(),

                },
                #[name="label_movie"]
                gtk::Label {
                    label: &self.model.data.1.get_title()
                },
            },
        }
    }
}
