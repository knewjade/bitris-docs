use mdbook::{
    book::Book,
    BookItem::Chapter,
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
};

pub trait Mapper {
    fn name(&self) -> &str;

    fn exec(&self, content: &String) -> String;
}

pub struct MappingProcessor {
    mapper: Box<dyn Mapper>,
}

impl MappingProcessor {
    pub(crate) fn new(mapper: Box<dyn Mapper>) -> Self {
        Self { mapper }
    }
}

impl Preprocessor for MappingProcessor {
    fn name(&self) -> &str {
        self.mapper.name()
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let Chapter(chapter) = item {
                chapter.content = self.mapper.exec(&chapter.content);
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "test"
    }
}
