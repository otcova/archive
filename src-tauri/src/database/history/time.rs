use chrono;

#[cfg(test)]
mod tests {
    #[test]
    fn done() {
        println!("{:?}", chrono::offset::Local::now());
        println!("{:?}", chrono::offset::Utc::now());
		assert!(false);
    }
}
