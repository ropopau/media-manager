use std::path::{Path, PathBuf};
use serde_json;
use std::fs;
use medman::mediafile::MediaFile;
use medman::scan::scan;

fn json_vers_vec(path: &Path) -> Vec<MediaFile<PathBuf, Option<String>, Option<u32>>>{
    let res_scan = fs::read_to_string(path.join("save.json")).unwrap();
    let res: Vec<MediaFile<PathBuf, Option<String>, Option<u32>>> = serde_json::from_str(&res_scan).unwrap();
    res
    }
#[test]
fn vide(){
    let resultat: Vec<MediaFile<PathBuf, Option<String>, Option<u32>>>  = vec![];
    let path_test = Path::new("tests\\test_datas\\vide");
    if !path_test.join("save.json").exists(){
        scan(path_test).expect("Scan Impossible");
    }
    let res = json_vers_vec(path_test);
    fs::remove_file(path_test.join("save.json")).expect("Fichier impossible a supprimer");
    assert_eq!(res, resultat)
}
#[test]
fn tout_type(){
    let resultat: Vec<MediaFile<PathBuf, Option<String>, Option<u32>>>  = vec![MediaFile { path: Path::new("tests\\test_datas\\tout_type\\3 Baltringue.mp3").to_path_buf(), artist: Some("Damso".to_string()), title: Some("Baltringue".to_string()), album: Some("Lithopédion".to_string()), year: Some(2018) , medtype: "AUD".to_string()}, MediaFile { path: Path::new("tests\\test_datas\\tout_type\\Orelsan - Basique.mp3").to_path_buf(), artist: Some("Orelsan".to_string()), title: Some("Basique".to_string()), album: Some("La fête est finie - EPILOGUE".to_string()), year: Some(2018), medtype: "AUD".to_string()}];
    let path_test = Path::new("tests\\test_datas\\tout_type");
    //let mut res_scan: Result<std::string::String, std::io::Error>= Ok("None".to_string());
    if !path_test.join("save.json").exists(){
        scan(path_test).expect("Scan Impossible");
    }
    let res = json_vers_vec(path_test);
    fs::remove_file(path_test.join("save.json")).expect("Fichier impossible a supprimer");
    assert_eq!(res, resultat)
}

#[test]
fn arbo(){
    let resultat: Vec<MediaFile<PathBuf, Option<String>, Option<u32>>>  = vec![MediaFile { path: Path::new("tests\\test_datas\\arborescence\\3 Baltringue.mp3").to_path_buf(), artist: Some("Damso".to_string()), title: Some("Baltringue".to_string()), album: Some("Lithopédion".to_string()), year: Some(2018) , medtype: "AUD".to_string()}, MediaFile { path: Path::new("tests\\test_datas\\arborescence\\a1\\a2\\a3\\5 Silence.mp3").to_path_buf(), artist: Some("Damso feat. Angèle".to_string()), title: Some("Silence".to_string()), album: Some("Lithopédion".to_string()), year: Some(2018) , medtype: "AUD".to_string()}, MediaFile { path: Path::new("tests\\test_datas\\arborescence\\a1\\Orelsan - Basique.mp3").to_path_buf(), artist: Some("Orelsan".to_string()), title: Some("Basique".to_string()), album: Some("La fête est finie - EPILOGUE".to_string()), year: Some(2018) , medtype: "AUD".to_string()}];
    let path_test = Path::new("tests\\test_datas\\arborescence");
    if !path_test.join("save.json").exists(){
        scan(path_test).expect("Scan impossible");
    }
    let res = json_vers_vec(path_test);
    fs::remove_file(path_test.join("save.json")).expect("Fichier impossible a supprimer");
    assert_eq!(res, resultat);
}

