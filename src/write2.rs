use crate::cli::CliArguments;
use crate::search::search;
use markdown_gen::markdown::{Markdown, AsMarkdown, List};
use std::fs::File;
use m3u::Entry;
/// Formatte le résultat d'une recherche sous la forme de fichier utilisable pour d'autre situation.
/// Permet de formatter sous forme de playlist lisible par une grande partie des lecteurs multimédias
pub fn write2m3u(filt: CliArguments) -> std::io::Result<()>{
    let res = search(filt.clone());
    let mut playlist: Vec<Entry> = vec![];
    for elem in &res{
        playlist.push(m3u::path_entry(&elem.path));
    }
    let mut file = std::fs::File::create(filt.path.unwrap().join("playlist.m3u")).unwrap();
    let mut writer = m3u::Writer::new(&mut file);
    for entry in &playlist {
        writer.write_entry(entry).unwrap();

    }
    Ok(())
}

/// Formatte le résultat d'une recherche sous la forme de fichier utilisable pour d'autre situation.
/// Permet de le formatter sous un fichier markdown
pub fn write2md(filt: CliArguments) -> std::io::Result<()>{
    let res = search(filt.clone());
    let file = File::create(&filt.path.unwrap().join("_Markdown_.md")).unwrap();
    let mut md = Markdown::new(file);
    //let mut file = LineWriter::new(fopening);

    md.write("RESULTAT".heading(1)).unwrap();
    for (sizeee, elem) in res.iter().enumerate(){
        md.write(format!("Fichier n°: {:?}\n", sizeee).heading(2)).unwrap();
        md.write(List::new(false).title(&elem.title[..]).item(&elem.artist[..]).item(&elem.album[..]).item(elem.year.to_string().as_str())).unwrap();
        md.write("\n").unwrap();
        }
    
    Ok(())

}
