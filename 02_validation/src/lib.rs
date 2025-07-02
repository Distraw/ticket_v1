use std::str::FromStr;
use strum_macros::EnumString;

#[derive(EnumString)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    fn new(title: String, description: String, status: String) -> Self {
        Ticket::validate_input(&title, &description);
        Self {
            title: title,
            description: description,
            status: Status::from_str(&status).expect("Only ToDo, InProgress and Done are allowed"),
        }
    }

    fn validate_input(title: &String, description: &String) {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }

        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }

        if description.len() <= 0 {
            panic!("Description cannot be empty");
        }

        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "ToDo".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "ToDo".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "ToDo".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "ToDo".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "InProgress".into());
    }
}
