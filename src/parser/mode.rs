#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum ParserMode {
    Debug,
    Release,
}

impl ParserMode {

    pub(crate) fn keep_comments(&self) -> bool {
        match self {
            ParserMode::Debug => {false},
            ParserMode::Release => {true},
        }
    }

    pub(crate) fn aggregate_instructions(&self) -> bool {
        match self {
            ParserMode::Debug => {false},
            ParserMode::Release => {true},
        }
    }

}