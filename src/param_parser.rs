use std::marker::PhantomData;
use std::str::FromStr;
use std::any::Any;


#[derive(Default)]
pub struct ParamParserTyped<T: 'static + FromStr> {
    phantom: PhantomData<T>
}

pub trait ParamParser {
    fn parse(&self, value: &str) -> Option<Box<Any>>;
}


impl<T: 'static + FromStr> ParamParserTyped<T> {
    pub fn new() -> ParamParserTyped<T> {
        ParamParserTyped::<T> {
            phantom: PhantomData::<T>::default(),
        }
    }
}

impl<T: 'static + FromStr> ParamParser for ParamParserTyped<T> {
    fn parse(&self, value: &str) -> Option<Box<Any>> {
        match value.parse::<T>() {
            Ok(value) => Some(Box::new(value)),
            Err(_) => None,
        }
    }
}
