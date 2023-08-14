extern crate flatbuffers;

#[allow(dead_code, unused_imports)]
#[path = "schema_generated.rs"]
mod schema_generated;
pub use schema_generated::{
    root_as_dictionary, Definition, DefinitionArgs, Dictionary, DictionaryArgs, Entry, EntryArgs,
    Example, ExampleArgs, Tag, Usage, UsageArgs,
};

fn main() {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    let example_japanese = builder.create_string("１０ページの５行目をみなさい。");
    let example_translation = builder.create_string("Let's begin with the fifth line on page 10.");
    let definition_string = builder.create_string("to see; to look; to watch; to view; to observe​");

    let example = Example::create(
        &mut builder,
        &ExampleArgs {
            value: Some(example_japanese),
            translation: Some(example_translation),
        },
    );

    let examples = builder.create_vector(&[example]);

    let definition = Definition::create(
        &mut builder,
        &DefinitionArgs {
            value: Some(definition_string),
            see: None,
            examples: Some(examples),
        },
    );

    let definitions = builder.create_vector(&[definition]);
    let pitches = builder.create_vector(&[0 as i16, 1 as i16]);
    let tags = builder.create_vector(&[Tag::v1, Tag::vi]);

    let usage = Usage::create(
        &mut builder,
        &UsageArgs {
            tags: Some(tags),
            pitches: Some(pitches),
            definitions: Some(definitions),
        },
    );

    let usages = builder.create_vector(&[usage]);

    let term = builder.create_string("見る");
    let reading = builder.create_string("みる");
    let frequencies = builder.create_vector(&[73 as i64, 21 as i64, 51 as i64]);

    let entry = Entry::create(
        &mut builder,
        &EntryArgs {
            term: Some(term),
            reading: Some(reading),
            see: None,
            alt: None,
            frequencies: Some(frequencies),
            usages: Some(usages),
        },
    );

    let entries = builder.create_vector(&[entry]);
    let dictionary_name = builder.create_string("Test Dictionary");

    let dictionary = Dictionary::create(
        &mut builder,
        &DictionaryArgs {
            name: Some(dictionary_name),
            entries: Some(entries),
        },
    );

    builder.finish(dictionary, None);
}
