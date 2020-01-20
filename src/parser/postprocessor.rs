use super::Postprocessor;
use crate::parser::Token;
use crate::Cell;

#[derive(Debug)]
pub(crate) struct AggregateSameProcessor;

impl AggregateSameProcessor {
    pub fn create() -> Self {
        AggregateSameProcessor {

        }
    }
}

impl<T> Postprocessor<T> for AggregateSameProcessor
    where T: Cell
{
    fn process(&self, mut code: Vec<Token<T>>) -> Vec<Token<T>> {
        let mut result = Vec::new();
        let mut iter = code.drain(0..);
        let last_item = iter.next();
        if last_item.is_none() {
            return result;
        }
        let mut last_item = last_item.unwrap();

        loop {
            let item = iter.next();
            if item.is_none() {
                break;
            }
            let item = item.unwrap();

            match item {
                Token::LoopBlock(content) => {
                    result.push(last_item);
                    last_item = Token::LoopBlock(self.process(content));
                },
                item => {
                    if last_item.can_join(&item) {
                        last_item = last_item.join(item);
                    } else {
                        result.push(last_item);
                        last_item = item;
                    }
                },
            }
        }

        result.push(last_item);
        result
    }
}