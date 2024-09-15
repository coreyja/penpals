use std::fmt::Display;

#[derive(Clone)]
struct Paragraph(String);

#[derive(Clone)]
struct Paragraphs(Vec<Paragraph>);

impl Display for Paragraphs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let strings: Vec<String> = self.0.iter().map(|paragraph| paragraph.0.clone()).collect();

        write!(f, "{}", strings.join("\n\n"))
    }
}

impl Paragraph {
    fn strip_quotes(&self) -> Paragraph {
        let lines = self.0.split("\n");
        let stripped_lines: Vec<_> = lines
            .filter(|line| !line.starts_with(">") || line.starts_with(">>"))
            .collect();

        Paragraph(stripped_lines.join("\n"))
    }

    fn quote(&self) -> Paragraph {
        let lines = self.0.split("\n");
        let quoted_lines: Vec<_> = lines.map(|line| format!("> {}", line)).collect();

        Paragraph(quoted_lines.join("\n"))
    }

    fn is_empty(&self) -> bool {
        self.0.trim().is_empty()
    }
}

impl Paragraphs {
    fn strip_quotes(&self) -> Paragraphs {
        Paragraphs(
            self.0
                .iter()
                .map(|paragraph| paragraph.strip_quotes())
                .collect(),
        )
    }

    fn quote(&self) -> Paragraphs {
        Paragraphs(self.0.iter().map(|paragraph| paragraph.quote()).collect())
    }

    fn remove_empty(&self) -> Paragraphs {
        Paragraphs(
            self.0
                .iter()
                .filter(|paragraph| !paragraph.is_empty())
                .cloned()
                .collect(),
        )
    }

    fn new(text: &str) -> Paragraphs {
        Paragraphs(
            text.split("\n\n")
                .map(|s| Paragraph(s.trim().to_string()))
                .collect(),
        )
    }
}

pub fn format(text: &str) -> String {
    Paragraphs::new(text)
        .strip_quotes()
        .remove_empty()
        .quote()
        .to_string()
}
