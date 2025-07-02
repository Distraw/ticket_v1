pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    const USIZE_64BIT_IN_BYTES: usize = 8;

    #[test]
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), USIZE_64BIT_IN_BYTES);
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&mut u64>(), USIZE_64BIT_IN_BYTES);
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), USIZE_64BIT_IN_BYTES);
    }
}
