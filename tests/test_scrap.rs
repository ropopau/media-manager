use medman::scrap::scrap;
use std::fs::copy;
use medman::cli::CliArguments;
use std::path::Path;
use std::fs;

#[test]
fn scrapping(){
    copy("tests\\test_datas\\scrap\\4 Julien.mp3", "tests\\test_datas\\scrap\\4 Julien.temp").expect("copie impossible");
    copy("tests\\test_datas\\scrap\\Orelsan - Discipline.mp3", "tests\\test_datas\\scrap\\Orelsan - Discipline.temp").expect("copie impossible");
    scrap(CliArguments::new_test(Some("scrap".to_string()), Some(Path::new("tests\\test_datas\\scrap").to_path_buf()), false,  None, None, None, None )).expect("Scrapping echoué");

    fs::remove_file("tests\\test_datas\\scrap\\4 Julien.mp3").expect("Fichier impossible a supprimeree");
    fs::remove_file("tests\\test_datas\\scrap\\Orelsan - Discipline.mp3").expect("Fichier impossible a supprimerff");
    fs::rename("tests\\test_datas\\scrap\\4 Julien.temp", "tests\\test_datas\\scrap\\4 Julien.mp3").expect("nom impossible");
    fs::rename("tests\\test_datas\\scrap\\Orelsan - Discipline.temp", "tests\\test_datas\\scrap\\Orelsan - Discipline.mp3").expect("nom impossible");
}