extern crate reqwest;
use serde_json::Value;
use std::{path::PathBuf};
use std::fs;
use crate::cli::CliArguments;
use crate::mediafile::MediaFile;
use crate::scan::scan;
use serde_json::json;
use id3::{Tag, TagLike, Version};

/// Récupère des métadonnées d'un certain média sur le web afin d'élargir les informations à propos de cette dernière.
/// Le site utilisé est <https://musicbrainz.org>, une base de donnée open source répertoriant une grande quantité de métadonnée.
/// Il y a une syntaxe de recherche que le programme utilise pour envoyer des requêtes.
pub fn scrap(filtre: CliArguments) -> std::io::Result<()>{
    // Début de la vérification et lecture du json
    let chemin = filtre.path().as_ref().unwrap();
    if !chemin.join(chemin.join("save.json")).exists(){
        scan(chemin.as_path()).expect("Scan impossible");
    }
    let res_scan = fs::read_to_string(chemin.join("save.json")).unwrap();
    let search_vec: Vec<MediaFile<PathBuf, String, u32>> = serde_json::from_str(&res_scan).unwrap();
    fs::remove_file(chemin.join("save.json")).expect("Fichier impossible a supprimer");
    // Fin de la vérification et lecture du json

    for elem in search_vec{
        let url = format!("https://musicbrainz.org/ws/2/recording/?query={} AND artists:{}&fmt=json", elem.title, elem.artist);
        let req = reqwest::get(url.as_str()).unwrap().text();
        let a = match req {
            Ok(x) => x,
            Err(_e) => "Scrapping impossible".to_string(),

        };
        let res: Value = serde_json::from_str(&a).unwrap();
        
        let tag = Tag::read_from_path(&elem.path).ok();
        if tag != None{
            println!("Scrapping et ecriture...");
            let mut tafef = tag.unwrap();

            let album = &res["recordings"][0]["releases"][0]["title"];
            let year = &res["recordings"][0]["releases"][0]["release-events"][0]["date"];
            if album == &json!(null) && year != &json!(null){
                //println!("markeur3");
                tafef.set_text("TYER", &year.to_string()[1..5]);
            }
            else if album != &json!(null) && year == &json!(null){
                //println!("markeur2");
                tafef.set_text("TALB", album.to_string());
            }
            else if album != &json!(null) && year != &json!(null){
                //println!("markeur1");
                tafef.set_text("TYER", &year.to_string()[1..5]);
                tafef.set_text("TALB", album.to_string());

            }
            tafef.write_to_path(&elem.path, Version::Id3v23).expect("probleme");
    }}
    Ok(())
}