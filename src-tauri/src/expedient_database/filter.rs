/// Text to match: A; B; C; A B; B A C
///
/// | Filter | Matches          |
/// | ------ | ---------------- |
/// | a B    | A; B; A B; B A C |
/// | A_B    | A B              |
/// | A b +C | B A C            |
/// | +A B   | A B; B A C       |

pub struct Filter {
    keywords: Vec<String>,
    mandatory_keywords: Vec<String>,
}

impl Filter {
    pub fn new<S: AsRef<str>>(filter: S) -> Self {
        let lowercase = filter.as_ref().to_lowercase();
        let mut iter = lowercase.splitn(2, "+");

        let keywords = iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|str| str.replace("_", " "))
            .map(|str| str.into())
            .collect();

        let mandatory_keywords = if let Some(mandatory_filter) = iter.next() {
            mandatory_filter
                .split_whitespace()
                .map(|str| str.replace("_", " "))
                .map(|str| str.into())
                .collect()
        } else {
            vec![]
        };

        Self {
            keywords,
            mandatory_keywords,
        }
    }

    pub fn test<S: AsRef<str>>(&self, text: S) -> bool {
        let subject = text.as_ref().to_lowercase().replace("_", " ");

        self.mandatory_keywords
            .iter()
            .all(|word| subject.contains(word))
            && (self.keywords.len() == 0
                || self
                    .keywords
                    .iter()
                    .find(|word| subject.contains(*word))
                    .is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter() {
        let filter = Filter::new("a B");
        assert!(filter.test("A"));
        assert!(filter.test("B"));
        assert!(!filter.test("C"));
        assert!(filter.test("A B"));
        assert!(filter.test("B A C"));

        let filter = Filter::new("A_B");
        assert!(!filter.test("A"));
        assert!(!filter.test("B"));
        assert!(!filter.test("C"));
        assert!(filter.test("A B"));
        assert!(!filter.test("B A C"));

        let filter = Filter::new("A b +C");
        assert!(!filter.test("A"));
        assert!(!filter.test("B"));
        assert!(!filter.test("C"));
        assert!(!filter.test("A B"));
        assert!(filter.test("B A C"));

        let filter = Filter::new("+A B");
        assert!(!filter.test("A"));
        assert!(!filter.test("B"));
        assert!(!filter.test("C"));
        assert!(filter.test("A B"));
        assert!(filter.test("B A C"));
    }
}
