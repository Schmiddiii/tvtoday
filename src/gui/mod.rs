mod movie_list;
mod movie_list_item;
mod movie_page;
mod win;

pub use win::Win;

use movie_list::MovieList;
use movie_list_item::MovieListItem;
use movie_page::{MoviePage, MoviePageMsg};
use win::WinMsg;
