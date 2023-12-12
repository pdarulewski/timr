use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Status {
    Todo,
    Doing,
    Done,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Todo" => Ok(Status::Todo),
            "Doing" => Ok(Status::Doing),
            "Done" => Ok(Status::Done),
            _ => Err(()),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Status::Todo => "Todo",
            Status::Doing => "Doing",
            Status::Done => "Done",
        };
        write!(f, "{}", status)
    }
}
