pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), size_of::<usize>() * 3);
    }

    #[test]
    fn ticket_size() {
        assert_eq!(size_of::<Ticket>(), size_of::<String>() * 3);
    }
}
