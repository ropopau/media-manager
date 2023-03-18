use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MediaFile<U, T, Q> { // U: PathBuf, T: Option<String>, Q: Option<u32>
    pub path: U,
    pub artist: T,
    pub title: T,
    pub album: T,
    pub year: Q,
    pub medtype: String

}

#[derive(Debug, PartialEq, Eq)]

pub enum GetResult<T, Q> {
    Num(Q),
    Stri(T),
    None
}

impl<U, T, Q> MediaFile<U, T, Q> {
    pub fn new(vpath: U, vartist: T, vtitle: T, valbum: T, vyear: Q, vmedtype: String) -> MediaFile<U, T, Q> {
        
        MediaFile {
            path: vpath,
            artist: vartist,
            title: vtitle,
            album: valbum,
            year: vyear,
            medtype: vmedtype
        }
    }
    

    pub fn get_valeur(&self, fieldname: &str) -> GetResult<&T, &Q>
    where Q : PartialEq<u32>, T :PartialEq<String>{
        match fieldname{
            "artist" => GetResult::Stri(&self.artist),
            "title" => GetResult::Stri(&self.title),
            "album" => GetResult::Stri(&self.album),
            "year" => GetResult::Num(&self.year),
            &_ => GetResult::None,
        }
        
    }

}
