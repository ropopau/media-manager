use std::path::Path;
use medman::mediafile::MediaFile;
use medman::search::search;
use medman::cli::CliArguments;

#[test]
fn deux_artiste(){
    let m = search(CliArguments::new_test(Some("search".to_string()), Some(Path::new("tests\\test_datas\\filtrage_un").to_path_buf()), false,  Some("DamsoANDOrelsan".to_string()), None, None, None));
    let arrayr = vec![MediaFile::new(Path::new("tests\\test_datas\\filtrage_un\\4 Julien.mp3").to_path_buf(), "Damso".to_string(), "Julien".to_string(), "Lithopédion".to_string(), 2018, "AUD".to_string()), MediaFile::new(Path::new("tests\\test_datas\\filtrage_un\\Orelsan - Dis-moi.mp3").to_path_buf(), "Orelsan".to_string(), "Dis-moi".to_string(), "La fête est finie - EPILOGUE".to_string(), 2018, "AUD".to_string()), MediaFile::new(Path::new("tests/test_datas/filtrage_un\\Orelsan - Discipline.mp3").to_path_buf(), "Orelsan".to_string(), "Discipline".to_string(), "La fête est finie - EPILOGUE".to_string(), 2018, "AUD".to_string())];
    assert_eq!(m, arrayr);
}
#[test]
fn un_titre(){
    let m = search(CliArguments::new_test(Some("search".to_string()), Some(Path::new("tests\\test_datas\\filtrage_un").to_path_buf()),false, None, Some("Julien".to_string()), None, None ));
    let arrayr = vec![MediaFile::new(Path::new("tests\\test_datas\\filtrage_un\\4 Julien.mp3").to_path_buf(), "Damso".to_string(), "Julien".to_string(), "Lithopédion".to_string(), 2018 , "AUD".to_string())];
    assert_eq!(m, arrayr);
}
#[test]
fn deux_artistes_et_un_titre(){
    let m = search(CliArguments::new_test(Some("search".to_string()), Some(Path::new("tests\\test_datas\\filtrage_un").to_path_buf()), false, Some("DamsoANDOrelsan".to_string()), Some("Julien".to_string()), None, None ));
    let arrayr = vec![MediaFile::new(Path::new("tests\\test_datas\\filtrage_un\\4 Julien.mp3").to_path_buf(), "Damso".to_string(), "Julien".to_string(), "Lithopédion".to_string(), 2018, "AUD".to_string())];
    assert_eq!(m, arrayr)


}
