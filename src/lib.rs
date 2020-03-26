pub mod oak {
    pub mod handle {
        include!(concat!(env!("OUT_DIR"), "/oak.handle.rs"));
    }
}

pub mod handle_extract {
    include!(concat!(env!("OUT_DIR"), "/handle_extract.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
