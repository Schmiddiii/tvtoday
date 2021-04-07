use glib::object::IsA;
use gtk::prelude::*;
use gtk::{Stack, Widget};
use relm::{Relm, Update};
use relm_derive::Msg;

/// Messages for the sliding stack.
#[derive(Msg)]
pub enum SlidingStackMsg {
    /// Switch the pages.
    Switch,
    /// Show the second page.
    ShowSecondPage,
}

/// The model for the slidin stack containing the two widgets.
pub struct SlidingStackModel<T1: IsA<Widget>, T2: IsA<Widget>> {
    widget1: T1,
    widget2: T2,
}

/// The sliding stack is a widget that always shows one of the given widgets, using a sliding animation to switch.
pub struct SlidingStack<T1: IsA<Widget>, T2: IsA<Widget>> {
    model: SlidingStackModel<T1, T2>,
    stack: Stack,
}

impl<T1: IsA<Widget>, T2: IsA<Widget>> Update for SlidingStack<T1, T2> {
    type Model = SlidingStackModel<T1, T2>;
    type ModelParam = (T1, T2);
    type Msg = SlidingStackMsg;

    fn model(_: &Relm<Self>, (widget1, widget2): Self::ModelParam) -> Self::Model {
        SlidingStackModel { widget1, widget2 }
    }

    fn update(&mut self, event: SlidingStackMsg) {
        match event {
            SlidingStackMsg::Switch => match self.stack.get_visible_child_name() {
                Some(name) => {
                    if name == "widget1" {
                        self.stack.set_visible_child(&self.model.widget2);
                    } else {
                        self.stack.set_visible_child(&self.model.widget1);
                    }
                }
                None => {
                    self.stack.set_visible_child(&self.model.widget1);
                }
            },
            SlidingStackMsg::ShowSecondPage => self.stack.set_visible_child(&self.model.widget2),
        }
    }
}

impl<T1: IsA<Widget>, T2: IsA<Widget>> relm::Widget for SlidingStack<T1, T2> {
    type Root = Stack;

    fn root(&self) -> Self::Root {
        self.stack.clone()
    }

    fn view(_: &Relm<Self>, model: Self::Model) -> Self {
        let stack = Stack::new();

        stack.set_transition_type(gtk::StackTransitionType::SlideUpDown);

        stack.add_named(&model.widget1, "widget1");
        stack.add_named(&model.widget2, "widget2");

        SlidingStack { model, stack }
    }
}
