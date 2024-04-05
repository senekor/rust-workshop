mod file_system;
mod http_client;

pub trait DistributionCenter {
    fn store(&self, receiver: String, content: String, express: bool);
    fn retrieve(&self, receiver: String) -> Option<String>;
}

pub enum DistributionStrategy {
    Local,
    Cloud,
}

pub fn new_distribution_center(strategy: DistributionStrategy) -> Box<dyn DistributionCenter> {
    match strategy {
        DistributionStrategy::Local => Box::new(file_system::FileSystemStorage),
        DistributionStrategy::Cloud => Box::new(http_client::HttpClient),
    }
}
