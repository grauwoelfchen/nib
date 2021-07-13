use serde::{Serialize, Serializer};

enum HeadingTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Unknown,
}

impl Serialize for HeadingTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: String = self.into();
        serializer.serialize_str(&s)
    }
}

impl From<&str> for HeadingTag {
    fn from(s: &str) -> Self {
        match s.to_ascii_lowercase().as_ref() {
            "h1" => Self::H1,
            "h2" => Self::H2,
            "h3" => Self::H3,
            "h4" => Self::H4,
            "h5" => Self::H5,
            "h6" => Self::H6,
            _ => Self::Unknown,
        }
    }
}

impl From<&HeadingTag> for String {
    fn from(h: &HeadingTag) -> String {
        match *h {
            HeadingTag::H1 => "h1",
            HeadingTag::H2 => "h2",
            HeadingTag::H3 => "h3",
            HeadingTag::H4 => "h4",
            HeadingTag::H5 => "h5",
            HeadingTag::H6 => "h6",
            _ => "",
        }
        .to_string()
    }
}

#[derive(Serialize)]
pub struct Heading {
    tag: HeadingTag,
    text: String,
}

impl Heading {
    pub fn new(tag: &str, text: &str) -> Self {
        Self {
            tag: tag.into(),
            text: text.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_heading_tag_serialization() {
        assert_eq!(json!(HeadingTag::H1), "h1");
        assert_eq!(json!(HeadingTag::H2), "h2");
        assert_eq!(json!(HeadingTag::H3), "h3");
        assert_eq!(json!(HeadingTag::H4), "h4");
        assert_eq!(json!(HeadingTag::H5), "h5");
        assert_eq!(json!(HeadingTag::H6), "h6");
    }

    #[test]
    fn test_heading_tag_from() {
        let h1: HeadingTag = "h1".into();
        assert_eq!(json!(h1), "h1");
    }

    #[test]
    fn test_heading_serialization() {
        let h1 = Heading::new("h1", "foo");
        assert_eq!(json!(h1).to_string(), "{\"tag\":\"h1\",\"text\":\"foo\"}");

        let h2 = Heading::new("h2", "bar");
        assert_eq!(json!(h2).to_string(), "{\"tag\":\"h2\",\"text\":\"bar\"}");

        let h3 = Heading::new("h3", "baz");
        assert_eq!(json!(h3).to_string(), "{\"tag\":\"h3\",\"text\":\"baz\"}");

        let h4 = Heading::new("h4", "qux");
        assert_eq!(json!(h4).to_string(), "{\"tag\":\"h4\",\"text\":\"qux\"}");

        let h5 = Heading::new("h5", "quux");
        assert_eq!(json!(h5).to_string(), "{\"tag\":\"h5\",\"text\":\"quux\"}");

        let h6 = Heading::new("h6", "quuz");
        assert_eq!(json!(h6).to_string(), "{\"tag\":\"h6\",\"text\":\"quuz\"}");
    }
}
