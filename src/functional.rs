// For lightweight strategies that do not require encapsulated state, you do not need to construct
// multiple structs. Rust treats functions as first-class citizens, meaning you can pass closures or
// function pointers directly into your context.

struct TextProcessor<F>
where
    F: Fn(&str) -> String,
{
    formatter: F,
}

impl<F> TextProcessor<F>
where
    F: Fn(&str) -> String,
{
    fn format(&self, text: &str) -> String {
        (self.formatter)(text)
    }
}

fn main() {
    // Strategy A: Uppercase formatter via closure
    let upper_processor = TextProcessor {
        formatter: |text: &str| text.to_uppercase(),
    };

    // Strategy B: Slugify formatter via closure
    let slug_processor = TextProcessor {
        formatter: |text: &str| text.replace(" ", "-").to_lowercase(),
    };

    println!("{}", upper_processor.format("hello rust")); // Output: HELLO RUST
    println!("{}", slug_processor.format("hello rust")); // Output: hello-rust
}
