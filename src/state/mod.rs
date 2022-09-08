pub mod loading;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum State {
    Loading,
    InGame,
}