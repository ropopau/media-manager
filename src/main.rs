use medman::cli::CliArguments;
use medman::scan::scan;
use medman::search::search;
use medman::write2::{write2md, write2m3u};
use medman::scrap::scrap;
use std::path::PathBuf;
use medman::mediafile::MediaFile;
use medman::interface::interface;

/// Gère l'éxécution du programme:
/// Si une commande et un répertoire est précisé, alors on passe en mode ligne de commande
/// Sinon, on passe en mode intéractif
fn main() {
    let args = CliArguments::new();
    if args.command == None || args.path == None {
        loop{
            interface().expect("Impossible d'appeler l'interface");
    }}
    else{
        let command = args.command.as_ref().unwrap();
        match command.as_str() {
            "scan" => {let resultat = scan(args.path()
                                                                    .as_ref()
                                                                    .unwrap()
                                                                    .as_path());
                        let deser: Vec<MediaFile<PathBuf, String, u32>> = serde_json::from_str(resultat.expect("Pas de fichier")
                                                                                                        .as_str())
                                                                                                        .unwrap();
                        println!("{:?}", deser);},

            "search" => {let resutltat = search(args);
                        println!("{:?}", resutltat)},
            "write2md" => {write2md(args).expect("Erreur lors de la création du fichier markdown");},
            "write2m3u" => {write2m3u(args).expect("Erreur lors de la playlist");},
            "scrap" => {scrap(args).expect("Erreur lors du scrapping");},
            &_ => (),
            }
                        

    }}
