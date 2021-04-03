use crate::model::{Movie, Program};
use crate::Error;

use async_trait::async_trait;

#[async_trait]
pub trait Provider: Send {
    /// Create a new provider.
    fn new() -> Self
    where
        Self: Sized;

    /// Clone the current provider. #[derive(Clone)] can not be used as `Self` needs to be `Sized`.
    fn clone(&self) -> Self
    where
        Self: Sized;

    /// Get the current program. This does not need to fill out all information about the movie.
    async fn get_program(&mut self) -> Result<Program, Error>;

    /// Get more information regarding the movie. This will be called when clicking on a movie in the list.
    /// If any error occures when providing more information, the given movie must be returned.
    async fn get_more_information(&self, movie: &Movie) -> Movie;
}
