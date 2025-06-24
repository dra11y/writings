use crate::{Author, WritingsType};

pub trait WritingsTrait<T: WritingsTrait<T>>:
    std::fmt::Debug + Sized + Clone + PartialEq + Eq
{
    fn ty(&self) -> WritingsType;
    fn ref_id(&self) -> String;
    fn title(&self) -> String;
    fn subtitle(&self) -> Option<String>;
    fn author(&self) -> Author;
    fn number(&self) -> Option<u32>;
    fn paragraph(&self) -> u32;
    fn text(&self) -> String;
}
