use medman::cli::CliArguments;
use std::path::Path;
use std::{fs};
use medman::write2::write2m3u;

#[test]
fn verif_playlist(){
    let _m = write2m3u(CliArguments::new_test(Some("write2m3u".to_string()), Some(Path::new("tests\\test_datas\\tout_type").to_path_buf()), false,  None, None, None, None ));
    let txt = fs::read_to_string("tests\\test_datas\\tout_type\\playlist.m3u").unwrap();
    let resulttext = "tests\\test_datas\\tout_type\\3 Baltringue.mp3\ntests\\test_datas\\tout_type\\Orelsan - Basique.mp3\n";
    assert_eq!(txt, resulttext);
}

#[test]
fn verif_markdown(){
    
}