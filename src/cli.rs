use std::vec;
use clap::Parser;

/// Représente les arguments en paramètres de ligne de commande
/// Seules l'argument command et path sont obligatoire
#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArguments {
    /// Commande à exécuter
    #[arg(default_missing_value(None))]
    pub command: Option<String>,
    /// Chemin d'accés
    pub path: Option<std::path::PathBuf>,
    ///Ajout de tag aux fichiers issus de la recherches
    #[clap(long)]
    pub tagging: bool,
    ///Les arguments ci-dessus sont tous présents pour optimiser ses recherches
    #[arg(short, default_missing_value(None))]
    title: Option<String>,

    #[arg(short, default_missing_value(None))]
    artist: Option<String>,

    #[clap(short, default_missing_value(None))]
    oualbum: Option<String>,
    
    #[arg(short, default_missing_value(None))]
    year: Option<String>,

}

impl CliArguments {
    /// Créer une instance du parser
    pub fn new() -> CliArguments {
        CliArguments::parse()
    }
    /// Créer une instance à partir d'arguments -> pour les tests 
    pub fn new_test(cmd: Option<String>, p: Option<std::path::PathBuf>, tag: bool, ar: Option<String>, t: Option<String>, al: Option<String>, y: Option<String>) -> CliArguments {
        CliArguments{command :cmd, 
                    path : p, 
                    tagging: tag, 
                    artist : ar, 
                    title : t, 
                    oualbum : al, 
                    year : y}
    }
    /// Renvoie le chemin d'accés au format PathBuf
    pub fn path(&self) -> &Option<std::path::PathBuf> {
        &self.path
    }
    /// Renvoie un Result. Celui-ci contient un tuple composé 
    /// du nom et de la valeur de chaque élément de la structure
    pub fn get_filtres(&self) -> std::io::Result<Vec<(&str, Vec<String>)>>{
        let mut vec_filtre: Vec<(&str, Vec<String>)> = vec![];
        let array: [(&str, &Option<String>); 4] = [("title", &self.title), 
                                                    ("artist", &self.artist), 
                                                    ("album", &self.oualbum), 
                                                    ("year", &self.year)];
        for elem in array{
            match elem.1{
                Some(x) => {let split = x.split("AND");
                                        let sous_vec_filtre: Vec<String> = split.map(|i| i
                                                                            .trim()
                                                                            .to_string())
                                                                            .take_while(|i| !i.is_empty())
                                                                            .collect();
                                        vec_filtre.push((elem.0, sous_vec_filtre));
                                        },
                None => {},
            }
        }
        Ok(vec_filtre)
        
    }
    
}

