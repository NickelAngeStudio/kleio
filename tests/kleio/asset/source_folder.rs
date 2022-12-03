use std::{path::{PathBuf}, fs::{self, File, remove_dir_all}, io::Write, cmp::{self, Ordering}};
use olympus_kleio::asset::{KAssetSourceFolder, KAssetSource};

static TEST_FOLDER: &str = "../target/tests/kleio/asset/";

/// Create a folder with parents folders
/// 
/// Will fail an assert if folders not created
fn create_folder(folder_path : &str) {

    match fs::create_dir_all(folder_path){
        Ok(_) => {},
        Err(_) => assert!(false, "Error when creating folder {}!", folder_path),
    }

}

/// Create a file and it's content.
/// 
/// Will fail an assert if file not created.
fn create_file_with_content(file_path : &str, file_content : &str){

    let file = File::create(&file_path);
    
    match file {
        Ok(mut file) => { 
            match file.write_all(file_content.as_bytes()) {
                Ok(_) => {},
                Err(_) => assert!(false, "Error when writing file {}!", file_path),
            }
        },
        Err(_) => assert!(false, "Error when creating file {}!", file_path),
    }
}

/// Compare 2 buffer
/// 
/// Source : https://codereview.stackexchange.com/questions/233872/writing-slice-compare-in-a-more-compact-way
fn compare(a: &[u8], b: &[u8]) -> cmp::Ordering {
    for (ai, bi) in a.iter().zip(b.iter()) {
        match ai.cmp(&bi) {
            Ordering::Equal => continue,
            ord => return ord
        }
    }

    /* if every single element was equal, compare length */
    a.len().cmp(&b.len())
}

#[test]
#[should_panic]
/// Trying to create [KAssetSourceFolder] using a folder that doesn't exists.
/// 
/// # Panics
/// This test function must panic because the folder doesn't exists.
fn kasset_source_folder_create_not_found() {
    KAssetSourceFolder::new(PathBuf::from("/kasf_not_found"));
}

#[test]
#[should_panic]
/// Trying to create [KAssetSourceFolder] using a file instead of a folder.
/// 
/// The test will create a directory "tests/kleio/asset" in target and a file for testing.
/// This directory can be deleted once test are finished.
/// 
/// # Panics
/// This test function must panic because path isn't a folder.
fn kasset_source_folder_create_not_folder() {
    // Test folder name
    let folder_name: &str = &(TEST_FOLDER.to_owned() + "kasf_create_not_folder/");

    // Create folder for test
    create_folder(folder_name);

    // Create file with content
    create_file_with_content(&(folder_name.to_owned() + "file.txt"), "Hello, world!");

    // Try to create KAssetSourceFolder from file.
    KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "file.txt"));
}


#[test]
/// Create [KAssetSourceFolder] and test KAssetSource::has_asset().
/// 
/// Will test both existents and inexistents files.
fn kasset_source_folder_has_file() {
    // Test folder name
    let folder_name: &str = &(TEST_FOLDER.to_owned() + "kasf_has_file/");

    // Create folder and sub folders for test
    for i in 1..10 {
        create_folder(&(folder_name.to_owned() + "subfolder" + i.to_string().as_str()));
    }
    

    // Create file with content in multiple subfolders
    for i in 1..10 {
        for j in 1..10 {
            let file_name = &(folder_name.to_owned() + "subfolder" + i.to_string().as_str() + "/file" + j.to_string().as_str() + ".txt");
            create_file_with_content(&file_name, &("Hello".to_owned() + i.to_string().as_str() + ", world" + j.to_string().as_str() + "!"));
        }
    }
    
    // Create KAssetSourceFolder
    let kasf = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned()));

    // Verify that source has ALL files except those that aren't created.
    for i in 0..15 {
        for j in 0..15 {
            let file_name = "subfolder".to_owned() + i.to_string().as_str() + "/file" + j.to_string().as_str() + ".txt";

            // For those in range, file should exists
            if i >= 1 && i <= 9 && j >=1 && j <= 9 {
                assert!(kasf.has_asset(PathBuf::from(&file_name)), "KAssetSourceFolder should have file {}", &file_name);
            } else {
                // For those out of range, file shouldn't exists.
                assert!(!kasf.has_asset(PathBuf::from(&file_name)), "KAssetSourceFolder shouldn't have file {}", &file_name);
            }

        }
    }

    // Clean test
    fs::remove_dir_all(PathBuf::from(folder_name)).expect("Test couldn't be cleaned!");
}



#[test]
/// Create [KAssetSourceFolder] and test read from asset given by KAssetSource::get_asset().
/// 
/// Will test both existents and inexistents files (will make sure that std::io::error is given).
fn kasset_source_folder_read_file() {
    // Test folder name
    let folder_name: &str = &(TEST_FOLDER.to_owned() + "kasf_read_file/");

    // Create folder and sub folders for test
    for i in 1..10 {
        create_folder(&(folder_name.to_owned() + "subfolder" + i.to_string().as_str()));
    }
    

    // Create file with content in multiple subfolders
    for i in 1..10 {
        for j in 1..10 {
            let file_name = &(folder_name.to_owned() + "subfolder" + i.to_string().as_str() + "/file" + j.to_string().as_str() + ".txt");
            create_file_with_content(&file_name, &("Hello".to_owned() + i.to_string().as_str() + ", world" + j.to_string().as_str() + "!"));
        }
    }
    
    // Create KAssetSourceFolder
    let kasf = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned()));

    // Read buffer
    let mut buffer : [u8; 50] = [0; 50];

    

    // Verify that source has ALL files except those that aren't created.
    for i in 0..15 {
        for j in 0..15 {
            
            let file_name = "subfolder".to_owned() + i.to_string().as_str() + "/file" + j.to_string().as_str() + ".txt";

            let result = kasf.get_asset(PathBuf::from(&file_name));

            // For those in range, file should exists, read it
            if i >= 1 && i <= 9 && j >=1 && j <= 9 {
                match result  {
                    Ok(mut file) => {
                        // Read from file and compare
                        let read = file.read(&mut buffer);
                        let content = &("Hello".to_owned() + i.to_string().as_str() + ", world" + j.to_string().as_str() + "!");

                        

                        match read {
                            Ok(size) => {
                                match compare(&buffer[0..size], &content.as_bytes()[0..size]){
                                    Ordering::Less => assert!(false, "Content is different that expected!"),
                                    Ordering::Equal => {},
                                    Ordering::Greater => assert!(false, "Content is different that expected!"),
                                }
                            },
                            Err(_) => assert!(false, "Couldn't read file!"),
                        }


                    },
                    Err(_) => assert!(false, "KAssetSourceFolder should have file {}", &file_name),
                }
            } else {
                // For those out of range, file shouldn't exists.
                match result {
                    // Assert error because file shouldn't exists
                    Ok(_) => assert!(false, "KAssetSourceFolder shouldn't have file {}", &file_name),

                    // Do nothing which is expected
                    Err(_) => {},
                }
            }

        }
    }

    // Clean test
    fs::remove_dir_all(PathBuf::from(folder_name)).expect("Test couldn't be cleaned!");
}