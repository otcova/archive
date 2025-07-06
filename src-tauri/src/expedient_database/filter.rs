/// Text to match: A; B; C; A B; B A C
///
/// | Filter | Matches          |
/// | ------ | ---------------- |
/// | a B    | A B; B A C       |
/// | A_B    | A B              |
/// | A +b C | A B; B A C       |
/// | +A B   | A; B; A B; B A C |

pub struct Filter {
    keywords: Vec<String>,
}

impl Filter {
    pub fn new<S: AsRef<str>>(filter: S) -> Self {
        let lowercase = filter.as_ref().to_lowercase();

        let keywords = lowercase
            .split_whitespace()
            .map(|str| str.replace("_", " ").into())
            .collect();

        Self { keywords }
    }

    /// Returns an index representing how much it matches the filter.
    /// 0 means that it does not match
    /// >0 means that it matches
    pub fn test<S: AsRef<str>>(&self, text: S) -> u32 {
        let subject = text.as_ref().to_lowercase().replace("_", " ");

        if !self
            .keywords
            .iter()
            .all(|keyword| subject.contains(keyword))
        {
            return 0;
        }

        let mut score = 1;

        let subject_keywords = subject.split_whitespace().take(16);

        for (subject, filter) in subject_keywords.zip(&self.keywords) {
            if subject.starts_with(filter) {
                score += 1;
                if subject == filter {
                    score += 2;
                }
            }
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter() {
        let filter = Filter::new("a B");
        assert!(0 == filter.test("A"));
        assert!(0 == filter.test("B"));
        assert!(0 == filter.test("C"));
        assert!(1 < filter.test("A B"));
        assert!(0 < filter.test("B A C"));

        let filter = Filter::new("A_B");
        assert!(0 == filter.test("A"));
        assert!(0 == filter.test("B"));
        assert!(0 == filter.test("C"));
        assert!(1 < filter.test("A B"));
        assert!(0 < filter.test("B A C"));
    }
}
