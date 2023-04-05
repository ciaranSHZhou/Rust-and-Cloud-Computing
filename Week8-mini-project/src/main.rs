use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};
fn main() -> anyhow::Result<()> {
let model = TranslationModelBuilder::new()
        .with_source_languages(vec![Language::English])
        .with_target_languages(vec![Language::Spanish, Language::French, Language::Italian])
        .create_model()?;
    // Ask the user for a sentence to translate
    println!("Enter a sentence to translate:");
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text)?;
    let output = model.translate(&[input_text], None, Language::French)?;
    for sentence in output {
        println!("{}", sentence);
    }
    Ok(())
}