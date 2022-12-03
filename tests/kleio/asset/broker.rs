use std::{fs::{self, File}, io::Write, path::PathBuf, vec, iter::FlatMap};

use olympus_kleio::asset::{KAssetBroker, KAssetSourceFolder, KAssetSource};

static TEST_FOLDER: &str = "../target/tests/kleio/asset/";


/********
* TESTS *
********/
/// KAssetBroker::new()
/// 
/// Create a new KAssetBroker.
#[test]
fn kasset_broker_create() {
    KAssetBroker::new();
}


/// KAssetBroker::add_source()
/// 
/// Create a new KAssetBroker and add a source to it.
#[test]
fn kasset_broker_add_source() {
    let mut kab = KAssetBroker::new();

    // Test folder name
    let folder_name: &str = &(TEST_FOLDER.to_owned() + "kasset_broker_add_1_source/");

    // Create test files
    create_test_files(folder_name, 1, 10);

    // Create KAssetSourceFolder
    let kaf = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned()));

    // Add KAssetSourceFolder to broker
    match_source_to_broker(kab.add_source(&kaf), false);
    // Clean test
    fs::remove_dir_all(PathBuf::from(folder_name)).expect("Test couldn't be cleaned!");
}

/// KAssetBroker::add_source()
/// 
/// Create a new KAssetBroker and try adding same source to it.
/// 
/// Adding the same source should trigger an error.
#[test]
fn kasset_broker_add_source_again() {
    let mut kab = KAssetBroker::new();

    // Test folder name
    let folder_name: &str = &(TEST_FOLDER.to_owned() + "kasset_broker_add_same_source/");

    // Create test files
    create_test_files(folder_name, 1, 10);

    // Create KAssetSourceFolder
    let kaf = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned()));

    // Add KAssetSourceFolder to broker
    match_source_to_broker(kab.add_source(&kaf), false);

    // Add KAssetSourceFolder to broker (should fail)
    match_source_to_broker(kab.add_source(&kaf), true);
    
    // Clean test
    fs::remove_dir_all(PathBuf::from(folder_name)).expect("Test couldn't be cleaned!");
}


/// KAssetBroker::add_source()
/// 
/// Create a new KAssetBroker, add 10 source to it and try adding 10 sources again.
#[test]
fn kasset_broker_add_source_10() {
    let mut kab = KAssetBroker::new();

    // Test folder name
    let folder_name: &str = &(TEST_FOLDER.to_owned() + "kasset_broker_add_10_source/");

    // Create test files
    create_test_files(folder_name, 10, 10);

    // Create 10 KAssetSources
    let kaf0 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder0/"));
    let kaf1 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder1/"));
    let kaf2 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder2/"));
    let kaf3 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder3/"));
    let kaf4 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder4/"));
    let kaf5 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder5/"));
    let kaf6 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder6/"));
    let kaf7 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder7/"));
    let kaf8 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder8/"));
    let kaf9 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder9/"));

    // Adding 10 sources (and readding them)
    for n in 0..2 {
        match_source_to_broker(kab.add_source(&kaf0), n == 1);
        match_source_to_broker(kab.add_source(&kaf1), n == 1);
        match_source_to_broker(kab.add_source(&kaf2), n == 1);
        match_source_to_broker(kab.add_source(&kaf3), n == 1);
        match_source_to_broker(kab.add_source(&kaf4), n == 1);
        match_source_to_broker(kab.add_source(&kaf5), n == 1);
        match_source_to_broker(kab.add_source(&kaf6), n == 1);
        match_source_to_broker(kab.add_source(&kaf7), n == 1);
        match_source_to_broker(kab.add_source(&kaf8), n == 1);
        match_source_to_broker(kab.add_source(&kaf9), n == 1);
    }
   

    // Clean test
    fs::remove_dir_all(PathBuf::from(folder_name)).expect("Test couldn't be cleaned!");
}


/// KAssetBroker::has_source()
/// 
/// Verify if broker has source.
#[test]
fn kasset_broker_has_source(){

    let mut kab = KAssetBroker::new();

    // Test folder name
    let folder_name: &str = &(TEST_FOLDER.to_owned() + "kasset_broker_has_source/");

    // Create test files
    create_test_files(folder_name, 10, 10);

    // Create 3 KAssetSources
    let kaf0 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder0/"));
    let kaf1 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder1/"));
    let kaf2 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder2/"));

    match_source_to_broker(kab.add_source(&kaf0), false);
    match_source_to_broker(kab.add_source(&kaf2), false);

    assert!(kab.has_source(&kaf0), "KAssetBroker should contain source #0");
    assert!(!kab.has_source(&kaf1), "KAssetBroker shouldn't contain source #1");
    assert!(kab.has_source(&kaf2), "KAssetBroker should contain source #2");

    // Clean test
    fs::remove_dir_all(PathBuf::from(folder_name)).expect("Test couldn't be cleaned!");

}



/// KAssetBroker::remove_source()
/// 
/// Remove a source from the Broker
#[test]
fn kasset_broker_remove_source() {
    
    let mut kab = KAssetBroker::new();

    // Test folder name
    let folder_name: &str = &(TEST_FOLDER.to_owned() + "kasset_broker_remove_source/");

    // Create test files
    create_test_files(folder_name, 10, 10);

    // Create 3 KAssetSources
    let kaf0 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder0/"));
    let kaf1 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder1/"));
    let kaf2 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder2/"));

    match_source_to_broker(kab.add_source(&kaf0), false);
    match_source_to_broker(kab.add_source(&kaf2), false);
    match_source_to_broker(kab.add_source(&kaf1), false);

    assert!(kab.get_sources().len() == 3, "Broker should contains 3 sources!");
    print_broker_sources_metadatas(&kab);

    match kab.remove_source(&kaf0){
        Ok(_) => {},
        Err(_) => assert!(false, "Error while removing source #0!"),
    }

    assert!(kab.get_sources().len() == 2, "Broker should contains 2 sources!");
    print_broker_sources_metadatas(&kab);

    match kab.remove_source(&kaf1){
        Ok(_) => {},
        Err(_) => assert!(false, "Error while removing source #1!"),
    }

    assert!(kab.get_sources().len() == 1, "Broker should contains 1 sources!");
    print_broker_sources_metadatas(&kab);

    match kab.remove_source(&kaf2){
        Ok(_) => {},
        Err(_) => assert!(false, "Error while removing source #2!"),
    }

    assert!(kab.get_sources().len() == 0, "Broker should contains 0 sources!");
    print_broker_sources_metadatas(&kab);

    // Clean test
    fs::remove_dir_all(PathBuf::from(folder_name)).expect("Test couldn't be cleaned!");


}


/// KAssetBroker::remove_source()
/// 
/// Remove an inexistant source from the Broker.
/// 
/// Broker should return an error upon remove_source.
#[test]
fn kasset_broker_remove_source_inexistant() {

    let mut kab = KAssetBroker::new();

     // Test folder name
     let folder_name: &str = &(TEST_FOLDER.to_owned() + "kasset_broker_remove_source_inexistant/");

     // Create test files
    create_test_files(folder_name, 10, 10);
 
     // Create KAssetSources
     let kaf = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder0/"));

    // Try to remove source even if not in broker
    match kab.remove_source(&kaf){
        Ok(_) => assert!(false, "Error! Source shouldn't be in broker!"),
        Err(_) => {},
    }
}


/// KAssetBroker::set_source_priority()
/// 
/// Set a source priority.
#[test]
fn kasset_broker_set_source_priority() {


    let mut kab = KAssetBroker::new();

    // Test folder name
    let folder_name: &str = &(TEST_FOLDER.to_owned() + "kasset_broker_add_10_source/");

    // Create test files
    create_test_files(folder_name, 10, 10);

    // Create 10 KAssetSources
    let kaf0 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder0/"));
    let kaf1 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder1/"));
    let kaf2 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder2/"));
    let kaf3 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder3/"));
    let kaf4 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder4/"));
    let kaf5 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder5/"));
    let kaf6 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder6/"));
    let kaf7 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder7/"));
    let kaf8 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder8/"));
    let kaf9 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder9/"));

    // Add sources to broker
    match_source_to_broker(kab.add_source(&kaf0), false);
    match_source_to_broker(kab.add_source(&kaf1), false);
    match_source_to_broker(kab.add_source(&kaf2), false);
    match_source_to_broker(kab.add_source(&kaf3), false);
    match_source_to_broker(kab.add_source(&kaf4), false);
    match_source_to_broker(kab.add_source(&kaf5), false);
    match_source_to_broker(kab.add_source(&kaf6), false);
    match_source_to_broker(kab.add_source(&kaf7), false);
    match_source_to_broker(kab.add_source(&kaf8), false);
    match_source_to_broker(kab.add_source(&kaf9), false);

    // Print broker sources
    print_broker_sources_metadatas(&kab);

    // Change a source priority
    match kab.set_source_priority(&kaf2, 8){
        Ok(_) => {},
        Err(_) => assert!(false, "Error happens when setting source priority!"),
    }

    // TODO : Switch multiple priority

     // Print broker sources
     print_broker_sources_metadatas(&kab);


     compare_priorities_order(&kab, vec![1,2,3]);


}

/// KAssetBroker::set_source_priority()
/// 
/// Set an inexistant source priority. Should generate an error.
#[test]
fn kasset_broker_set_source_priority_inexistant() {
    todo!();
}

/// KAssetBroker::set_source_priority()
/// 
/// Set a source priority > broker length. Should generate an error.
#[test]
fn kasset_broker_set_source_priority_oob() {
    todo!();
}

/// KAssetBroker::get_sources()
/// 
/// Get an immutable ref to the vector of sources
#[test]
fn kasset_broker_get_sources() {
    todo!();
}

/// KAssetBroker::get_asset()
/// 
/// Get an asset from broker with 0 source
#[test]
fn kasset_broker_get_asset_no_source() {
    todo!();
}

/// KAssetBroker::get_asset()
/// 
/// Get an asset from broker with 1 source
#[test]
fn kasset_broker_get_asset_1_source() {
    todo!();
}


/// KAssetBroker::get_asset()
/// 
/// Get an asset from broker with 10 sources
#[test]
fn kasset_broker_get_asset_10_source() {
    todo!();
}

/// KAssetBroker::get_asset()
/// 
/// Get an inexistant asset from broker
#[test]
fn kasset_broker_get_asset_inexistant() {
    todo!();
}


/// KAssetBroker Stress test
///
/// * Add / remove multiple sources.
/// * Fetch assets
/// * Set priorities
/// 
/// 1 million time
#[test]
fn kasset_broker_stress() {
    todo!();
}


/*************
 * FUNCTIONS *
 ************/
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

/// Quickly create test files 
fn create_test_files(folder_name : &str, sf_count: usize, file_count:usize){

    // Create folder and sub folders for test
    for i in 0..sf_count {
        create_folder(&(folder_name.to_owned() + "subfolder" + i.to_string().as_str()));
    }
    
    // Create file with content in multiple subfolders
    for i in 0..sf_count {
        for j in 0..file_count {
            let file_name = &(folder_name.to_owned() + "subfolder" + i.to_string().as_str() + "/file" + j.to_string().as_str() + ".txt");
            create_file_with_content(&file_name, &("Hello".to_owned() + i.to_string().as_str() + ", world" + j.to_string().as_str() + "!"));
        }
    }
}

/// Match result and assert error according to expectation
fn match_source_to_broker<'a>(res : Result<usize, &str>, expect_fail : bool){

    match res{
        Ok(_) => assert!(!expect_fail, "Adding the same source should fail!"),
        Err(_) => assert!(expect_fail, "Add 1 source failed!"),
    }


}

/// Print KAssetBroker sources metadata
fn print_broker_sources_metadatas(kab : &KAssetBroker){

    println!("\n*** START BROKER SOURCES ***");


    for src in kab.get_sources().iter() {

        println!("{}", src.get_metadata());
    } 
    
    println!("*** END BROKER SOURCES ***\n");
}

// Compare source priority and return true if priorities order are correct
fn compare_priorities_order(kab : &KAssetBroker, priority : Vec<usize>) -> bool{

    let mut is_correct : bool = true;

    for src in kab.get_sources().iter() {

        let metadata = src.get_metadata();
        let mut split = metadata.split("\",\"");

        let path  = split.next().unwrap();

        let index = path[path.len() - 5, path.len()];


        println!("{:?}", split.next().unwrap());

    }

    is_correct
}