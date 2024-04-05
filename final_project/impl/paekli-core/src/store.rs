mod file_system;

pub trait DistributionCenter {
    fn store(&self, receiver: String, content: String, express: bool);
    fn retrieve(&self, receiver: String) -> Option<String>;
}

pub fn new_distribution_center() -> Box<dyn DistributionCenter> {
    Box::new(file_system::FileSystemStorage)
}
