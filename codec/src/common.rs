/// Defines a series of methods to interact with a list of codec descriptors.
pub trait CodecList {
    /// The type of the structure used to describe a codec.
    type D: ?Sized;

    /// Creates a new codec list.
    fn new() -> Self;

    // TODO more lookup functions
    /// Search by name whether a codec descriptor is in the codec list and
    /// returns it.
    ///
    /// If the requested codec descriptor is not in the list,
    /// `None` is returned.
    fn by_name(&self, name: &str) -> Option<&'static Self::D>;

    /// Appends a codec to the list.
    fn append(&mut self, desc: &'static Self::D);

    /// Adds a list of codec descriptors.
    fn add_list(&mut self, descs: &[&'static Self::D]) {
        for &desc in descs {
            self.append(desc);
        }
    }
}

/// Defines a series of methods to interact with a list of codec descriptors.
pub trait CodecList2 {
    /// The type of the structure used to describe a codec.
    type D: ?Sized;

    /// Creates a new codec list.
    fn new() -> Self;

    // TODO more lookup functions
    /*/// Search by name whether a codec descriptor is in the codec list and
    /// returns it.
    ///
    /// If the requested codec descriptor is not in the list,
    /// `None` is returned.
    fn by_name(&self, name: &str) -> Option<&Box<&'static Self::D>>;*/

    /// Appends a codec to the list.
    fn append(&mut self, desc: &'static Self::D);

    /*/// Adds a list of codec descriptors.
    fn add_list(&mut self, descs: &[&'static Self::D]) {
        for &desc in descs {
            self.append(desc);
        }
    }*/
}
