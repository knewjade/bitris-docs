use clap::{Arg, ArgMatches, Command};
use mdbook::{
    errors::Error,
    preprocess::{CmdPreprocessor, Preprocessor},
};
use std::io;

pub fn execute(preprocessor: &impl Preprocessor) -> Result<(), Error> {
    let name = preprocessor.name().to_string();
    let matches = make_app(name).get_matches();

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if let Some(sub_args) = matches.subcommand_matches("supports") {
        if handle_supports(preprocessor, sub_args) {
            return Ok(());
        } else {
            return Err(Error::msg("err"));
        }
    }

    handle_preprocessing(preprocessor)
}

fn make_app(name: String) -> Command {
    Command::new(name)
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn handle_supports(preprocessor: &impl Preprocessor, sub_args: &ArgMatches) -> bool {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    preprocessor.supports_renderer(renderer)
}

fn handle_preprocessing(preprocessor: &impl Preprocessor) -> Result<(), Error> {
    let (context, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if context.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, but we're being called from version {}",
            preprocessor.name(),
            mdbook::MDBOOK_VERSION,
            context.mdbook_version,
        );
    }

    dbg!(preprocessor.name());

    let processed_book = preprocessor.run(&context, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
