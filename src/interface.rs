use crate::scan::scan;
use crate::search::search;
use crate::scrap::scrap;
use crate::write2::{write2m3u, write2md};
use std::path::{Path, PathBuf};
use crate::mediafile::MediaFile;
use dialoguer::{Input, console, Select, theme::ColorfulTheme};
use console::Term;
use crate::cli::CliArguments;





fn option_none(vect: Vec<Option<String>>) -> Vec<Option<String>>{
    let mut newvec: Vec<Option<String>> = vec![];
    for elem in &vect{
        if elem.to_owned().unwrap() == ""{
            newvec.push(None);
        }
        else{
            newvec.push(elem.clone());
        }
    }
    newvec

}
fn def_args(path: String) -> CliArguments{
    Select::with_theme(&ColorfulTheme::default())
            .item("filtres".to_string().as_str())
            .default(0)
            .interact_on_opt(&Term::stderr()).expect("error");
    let mut vecf:Vec<Option<String>> = vec![];
    let inputartiste = Some(Input::<String>::new().with_prompt("Artiste")
        .allow_empty(true).interact_text().expect("error"));
    let inputtitre = Some(Input::<String>::new().with_prompt("Titre")
        .allow_empty(true).interact_text().expect("error"));
    let inputalbum = Some(Input::<String>::new().with_prompt("Album")
        .allow_empty(true).interact_text().expect("error"));
    let inputannee = Some(Input::<String>::new().with_prompt("Annee")
        .allow_empty(true).interact_text().expect("error"));
    let inputtag = Input::<bool>::new().with_prompt("Tagging?")
        .allow_empty(true).interact_text().expect("error");
    vecf.push(inputartiste);
    vecf.push(inputtitre);
    vecf.push(inputalbum);
    vecf.push(inputannee);
    
    let newvec = option_none(vecf);
    
    let m = CliArguments::new_test(Some("search".to_string()),
                                                            Some(Path::new(&path).to_path_buf()), 
                                                            inputtag,  
                                                            newvec[0].to_owned(), 
                                                            newvec[1].to_owned(), 
                                                            newvec[2].to_owned(), 
                                                            newvec[3].to_owned() );
    m
}


/// Interface intéractive avec le crate Dialoguer
/// 
/// 
pub fn interface() -> std::io::Result<()> {
    let items = vec!["scan", "search", "scrap", "write2md", "write2m3u"];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    let path = Input::<String>::new().with_prompt("Faites glisser un répertoire...")
        .interact_text()?;
    match selection {
        Some(index) => match items[index]{
            "scan" => {let a =scan(Path::new(&path));
                        let deser: Vec<MediaFile<PathBuf, String, u32>> = serde_json::from_str(a.expect("Pas de fichier")
                        .as_str())
                        .unwrap();
                        println!("{:?}", deser);},

            "search" => {let args = def_args(path);
                            let res = search(args);
                            println!("{:?}", res)},


            "scrap" => {let args = def_args(path);
                        scrap(args).expect("erreur");},
            "write2md" => {let args = def_args(path);
                            write2md(args).expect("erreur");},
            "write2m3u" => {let args = def_args(path);
                            write2m3u(args).expect("erreur");},
            &_ => {},





        },
        None => println!("User did not select anything")
    }

    Ok(())

}