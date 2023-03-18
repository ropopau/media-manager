use medman::search::{search, tag};
use medman::cli::CliArguments;
use std::path::Path;
use std::fs::copy;
use id3::TagLike;
use std::fs;

#[test]
fn tag_nonvide(){
    copy("tests\\test_datas\\tag_nonvide\\3 Baltringue.mp3", "tests\\test_datas\\tag_nonvide\\3 Baltringue.temp").expect("copie impossible");
    copy("tests\\test_datas\\tag_nonvide\\Orelsan - Basique.mp3", "tests\\test_datas\\tag_nonvide\\Orelsan - Basique.temp").expect("copie impossible");
    let m = search(CliArguments::new_test(Some("search".to_string()), Some(Path::new("tests\\test_datas\\tag_nonvide").to_path_buf()), true,  Some("Damso:Orelsan".to_string()), None, None, None ));
    let vectag = tag(m).expect("Tagging impossible");
    let mut var : bool = true;
    for elem in vectag{
        if !elem.get("TPOS").is_some(){
            var = false;
        }

    }
    {
    fs::remove_file("tests\\test_datas\\tag_nonvide\\3 Baltringue.mp3").expect("Fichier impossible a supprimeree");
    fs::remove_file("tests\\test_datas\\tag_nonvide\\Orelsan - Basique.mp3").expect("Fichier impossible a supprimerff");
    fs::rename("tests\\test_datas\\tag_nonvide\\3 Baltringue.temp", "tests\\test_datas\\tag_nonvide\\3 Baltringue.mp3").expect("nom impossible");
    fs::rename("tests\\test_datas\\tag_nonvide\\Orelsan - Basique.temp", "tests\\test_datas\\tag_nonvide\\Orelsan - Basique.mp3").expect("nom impossible");
    }
    assert_eq!(var, true);
    



}