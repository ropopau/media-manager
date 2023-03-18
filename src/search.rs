use std::{path::PathBuf};
use crate::{cli::CliArguments};
use crate::scan::scan;
use crate::mediafile::MediaFile;
use crate::mediafile::GetResult;
use std::{fs};
use id3::{Tag, TagLike, Version};

/// Recherche dans un répertoire tout les fichiers multimédias répondant à des filtres précisés.
/// Suivant le choix de l'utilisation, un tag commun est donnée à tout les éléments résultants de la recherche.
pub fn search(filtre: CliArguments) -> Vec<MediaFile<PathBuf, String, u32>> {
    // Début de la vérification et lecture du json
    let chemin = filtre.path().as_ref().unwrap();
    if !chemin.join("save.json").exists(){
        scan(chemin.as_path()).expect("Scan impossible");
    }
    let res_scan = fs::read_to_string(chemin.join("save.json")).unwrap();
    let res: Vec<MediaFile<PathBuf, String, u32>> = serde_json::from_str(&res_scan).unwrap();
    //fs::remove_file(chemin.join("save.json")).expect("Fichier impossible a supprimer");
    // Fin de la vérification et lecture du json

    let filtres_choisis = filtre.get_filtres().unwrap();
    let mut vecteur_res: Vec<MediaFile<PathBuf, String, u32>> = res.clone();
    for filt in filtres_choisis{
        let nomf = filt.0;
        let vecf = filt.1;
        let mut acpr = "NA".to_string();
        for music_file in res.clone(){
            let a = music_file.get_valeur(nomf);
            match a{
                GetResult::Num(x) => {acpr = x.to_string();},
                GetResult::Stri(x) => {acpr = x.to_string();},
                GetResult::None => {},
            
            }
            if !vecf.contains(&acpr){
                vecteur_res.retain(|x| x != &music_file);
            }}
        
        
    }
    if filtre.tagging{
        println!("Tagging en cours...");
        tag(vecteur_res.clone()).expect("Erreur lors du tagging");
    }
    vecteur_res
}


pub fn tag(vector:Vec<MediaFile<PathBuf, String, u32>> ) -> std::io::Result<Vec<Tag>>{
    //copy("Julien.mp3", "music.mp3");
    let mut listtag: Vec<Tag> = vec![];
    for elem in &vector{
        let path = &elem.path;
        let tag = Tag::read_from_path(path).ok();
        
        if tag != None{
            println!("Ecriture...");
            let mut tafef = tag.unwrap();
            //tafef.remove_disc();
            tafef.set_text("TPOS", "DSK_MEDMAN");
            tafef.write_to_path(path, Version::Id3v23).expect("probleme");
            listtag.push(tafef);
        }
    } 
    Ok(listtag)





}