use std::{io::{Read, ErrorKind}, path::PathBuf, fmt::Error};

use super::KAssetSource;

/// Middle men between [`0..n`] [KAssetSource] to supplies assets according to source priority.
/// 
/// // TODO: Example and doc. Explain priority system (0 > n)
/// 
pub struct KAssetBroker<'a> {

    // Vector of sources. Position 0 is highest priority.
    sources: Vec<&'a dyn KAssetSource>,
}


impl<'a> KAssetBroker<'a> {

    /// Create a new instance of KAssetBroker.
    pub fn new() -> KAssetBroker<'a> {
        // Create sources vector
        let sources : Vec<&'a dyn KAssetSource> = Vec::new();

        // Return new data broker
        KAssetBroker { sources }
    }

    /// Add a [KAssetSource] reference to the broker.
    /// 
    /// Added source are always last in priority.
    /// 
    /// # Argument(s)
    /// * `source` - [KAssetSource] reference to fetch assets.
    /// 
    /// # Return
    /// Ok<usize> with the priority of source added, Err<&str> otherwise.
    pub fn add_source(&mut self, source : &'a dyn KAssetSource) -> Result<usize, &str>{

        if self.has_source(source) == false {
            let priority = self.sources.len();
            // Add source to broker
            self.sources.push(source);

            Ok(priority)
        } else {
            Err("Source already exists in broker!")
        }
    }

    /// Remove a [KAssetSource] reference to the broker.
    /// 
    /// # Argument(s)
    /// * `source` - [KAssetSource] reference to fetch assets.
    /// 
    /// # Return
    /// Ok<usize> with the priority of source removed, Err<&str> otherwise.
    pub fn remove_source(&mut self, source : &'a dyn KAssetSource) -> Result<usize, &str>{

        let priority = self.get_source_priority(source);

        match priority {
            Some(priority) => {
                self.sources.remove(priority);
                Ok(priority)
            },
            None => Err("Source not found in broker!"),
        }
    }

    /// Set a [KAssetSource] priority. Will change other sources priorities.
    /// 
    /// # Argument(s)
    /// * `source` - [KAssetSource] reference to fetch assets.
    /// * `priority` - Set source priority. 0 > n.
    /// 
    /// # Return
    /// Ok<usize> with the new priority of source, Err<&str> otherwise.
    /// 
    /// # Error(s)
    /// * `priority` > broker sources length
    /// * `source` not found in broker sources
    pub fn set_source_priority(&mut self, source : &'a dyn KAssetSource, priority : usize)-> Result<usize, &str>{

        // Get current position / priority of the source
        let position = self.get_source_priority(source);

        match position {
            Some(mut position) => {

                if priority < self.sources.len() {

                    // Will replace the position at the correct place
                    while priority > position {
                        let temp = self.sources[position];
                        self.sources[position] = self.sources[position + 1];
                        self.sources[position + 1] = temp;
                        position += 1;
                            
                    }
                    while priority < position {
                        let temp = self.sources[position];
                        self.sources[position] = self.sources[position - 1];
                        self.sources[position - 1] = temp;                            
                        position -= 1;
                    }

                    Ok(priority)

                } else {
                    Err("New priority is higher than broker sources length!")
                }

            },
            None => Err("Source not found in broker!"),
        }

    }

    /// Get broker sources vector reference.
    /// 
    /// # Return
    /// Unmutable broker sources vector reference.
    pub fn get_sources(&self) -> &Vec<&'a dyn KAssetSource> {
        &self.sources
    }

    /// Fetch an asset in sources according to priority and path.
    /// 
    /// # Argument(s)
    /// * `path` - Relative path to the asset.
    /// 
    /// # Return
    /// `Ok(Box(Read))` if asset found, [std::io::Error] otherwise.
    pub fn get_asset(&self, path: PathBuf) ->  Result<Box<dyn Read>, std::io::Error>{

        // Use for 0.. as priority
        for n in 0..self.sources.len() {
            let src = self.sources[n];
            
            // If sources has asset, return it
            if src.has_asset(path.clone()) {
                return src.get_asset(path);
            } 
        }

        Err(std::io::Error::new(ErrorKind::NotFound, "Asset not found!"))
    }

    /// Get the source priority.
    /// 
    /// # Argument(s)
    /// * `source` - [KAssetSource] reference to fetch assets.
    /// 
    /// # Return
    /// Some(usize) with the priority/position if the source, None if not found
    pub fn get_source_priority(&self, source : &'a dyn KAssetSource) -> Option<usize>{

        for n in 0..self.sources.len() {
            if std::ptr::eq(self.sources[n], source) {
                return Some(n) 
            }
        } 

        None
    }

    /// Verify if broker contain source
    /// 
    /// # Argument(s)
    /// * `source` - [KAssetSource] reference to fetch assets.
    /// 
    /// # Return
    /// True if broker contain source, false otherwise
    pub fn has_source(&self, source : &'a dyn KAssetSource) -> bool{
        match  self.get_source_priority(source) {
            Some(_) => true,
            None => false,
        } 
    }

}
