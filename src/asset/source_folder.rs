use core::panic;
use std::{path::{PathBuf}, fs::{File}, io::Read, time::{SystemTime}};
use crate::asset::KAssetSource;

/// ##### [KAssetSource] implementation using a file system folder.
/// 
/// KAssetSourceFolder uses a given folder as a base path to retrieve assets.
/// 
/// ```no_run
/// // Import crate module
/// use std::path::PathBuf;
/// use olympus::kleio::asset::{KAssetSource , KAssetSourceFolder};
/// 
/// // Create KAssetSourceFolder using a folder as base
/// let source : KAssetSourceFolder = KAssetSourceFolder::new(PathBuf::from("/base_folder"));
/// 
/// // Get assets from KAssetSourceFolder with path relative to base
/// let asset1 = source.get_asset(PathBuf::from("audio/audio1.ogg"));
/// let asset2 = source.get_asset(PathBuf::from("models/model.obj"));
/// ```
pub struct KAssetSourceFolder {
    // Path of the source folder
    folder_path : PathBuf,

    // Metadata of KAssetSourceFolder (folderpath, created, modified)
    metadata : String
}

impl KAssetSourceFolder {
    /// Create a new KAssetSourceFolder from a folder path.
    /// 
    /// Meta data will be folder_path and 
    /// 
    /// # Argument(s)
    /// * `folder_path` - Path of the folder to read data from.
    /// 
    /// # Panic
    /// Will panic if folder not found, if not a folder or insufficient permissions.
    pub fn new(folder_path : PathBuf) -> KAssetSourceFolder {


        if !folder_path.exists() {
            // Panic if folder doesn't exists
            panic!("Folder {:?} not found!", folder_path.as_os_str());
        }

        // Panic if path isn't a folder
        if !folder_path.is_dir(){
            panic!("{:?} is not a folder!", folder_path.as_os_str());
        }

        KAssetSourceFolder {
            metadata : Self::create_metadata(&folder_path),
            folder_path,
            
        }
    }

    /// Create KAssetSourceFolder metadata which contains path, created and modified in JSON format.
    /// 
    /// If an error occur while retrieving metadata, `(metadata _error)` value will fill json variable.
    fn create_metadata(folder_path : &PathBuf)->String {

        let mut metadata : String = "{ \"path\":\"".to_owned() +  &folder_path.clone().into_os_string().into_string().unwrap().to_owned() + &"\",".to_owned();

        // Get folder metadata
        match folder_path.metadata(){
            // If OK, retieve date created and modified
            Ok(md) => {

                // Date created, UNIX format
                match md.created(){
                    Ok(st) => {
                        match st.duration_since(SystemTime::UNIX_EPOCH) {
                            Ok(n) => metadata.push_str(&("\"created\":\"".to_owned() + &n.as_millis().to_string().to_owned() + &"\",".to_owned())),
                            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                        }
                        
                    },
                    Err(_) => {
                        metadata.push_str("\"created\":\"(metadata _error)\",");
                    },
                }

                // Date modified, UNIX format
                match md.modified(){
                    Ok(st) => {
                        match st.duration_since(SystemTime::UNIX_EPOCH) {
                            Ok(n) => metadata.push_str(&("\"modified\":\"".to_owned() + &n.as_millis().to_string().to_owned() + &"\",".to_owned())),
                            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                        }
                        
                    },
                    Err(_) => metadata.push_str("\"modified\":\"(metadata _error)\","),
                }

            },
            // If error, still continue BUT created and modified will have (metadata _error) value
            Err(err) => {
                metadata.push_str("\"created\":\"(metadata _error)\",");
                metadata.push_str("\"modified\":\"(metadata _error)\",");
            },
        }

        metadata

    }

}

impl KAssetSource for KAssetSourceFolder {

    fn get_metadata(&self) -> String {
        self.metadata.clone()
    }
    
    fn has_asset(&self, path: PathBuf) -> bool {
        // Get asset full path
        let mut full_path: PathBuf = self.folder_path.clone();
        full_path.push(path);

        // Return if file exists
        full_path.exists()
    }

    fn get_asset(&self, path: PathBuf) ->  Result<Box<dyn Read>, std::io::Error> {
        // Get asset full path
        let mut full_path: PathBuf = self.folder_path.clone();
        full_path.push(path);

        // Return file opened
        match File::open(full_path){
            // File opened correctly, return handle
            Ok(file) => Ok(Box::new(file)),

            // File error, return error
            Err(err) => Err(err),
        }
    }

} 
