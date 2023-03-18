use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use serde_json;
use std::fs::File;
use std::fs;
use id3::{Tag, TagLike};
use crate::mediafile::MediaFile;
use mp4ameta::Tag as TagV;
const SUPPORTED_EXTENSIONS: [&str; 2] = ["mp3", "mp4"];

fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}


/// Scan un répertoire et créer un fichier json composé des infos de chaques fichier média du répertoire
/// à la racine de celui-ci.
pub fn scan(path: &Path) -> std::io::Result<String> {
    let mut music_files: Vec<MediaFile<PathBuf, String, u32>> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker {
        match entry{
            Ok(x) => {if is_supported(&x) {
                if &x.path().extension().unwrap().to_str().unwrap().to_owned() == "mp3"{
                    let tagf = Tag::read_from_path(x.path()).ok();
                    if tagf != None{
                        let tag = tagf.unwrap();
                        let tag_art = tag.artist().unwrap_or("None");
                        let tag_title = tag.title().unwrap_or("None");
                        let tag_year =  tag.year().unwrap_or(0000) as u32;
                        let tag_album = tag.album().unwrap_or("None");
                        music_files.push(MediaFile::new(x.path().to_path_buf(), 
                                                        tag_art.to_string(), 
                                                        tag_title.to_string(), 
                                                        tag_album.to_string(), 
                                                        tag_year,
                                                        "AUD".to_string()));       
                 }}else{
                    let tagf = TagV::read_from_path(x.path()).ok();
                    if tagf != None{
                        let tag = tagf.unwrap();
                        let tag_art = tag.artist().unwrap_or("None");
                        let tag_title = tag.title().unwrap_or("None");
                        let tag_year =  tag.year().unwrap_or("None");
                        music_files.push(MediaFile::new(x.path().to_path_buf(), 
                                                        tag_art.to_string(), 
                                                        tag_title.to_string(), 
                                                        "NA".to_string(), 
                                                        tag_year.to_string().parse::<u32>().unwrap_or(0),
                                                        "VID".to_string()));


                    }
                 }
                }},
            Err(_e) => {}};
    }

    let serialized = serde_json::to_string(&music_files)?;
    File::create( path.join("save.json")).expect("Chemin incorrecte");
    fs::write(path.join("save.json"), &serialized)?;

    Ok(serialized)

}