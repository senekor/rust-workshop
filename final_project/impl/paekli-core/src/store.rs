mod file_system;
mod http_client;
mod sql_database;

pub trait DistributionCenter {
    fn store(&self, recipient: String, content: String, express: bool);
    fn retrieve(&self, recipient: String) -> Option<String>;
}

pub enum DistributionStrategy {
    Fs,
    Sql,
    Http,
}

pub fn new_distribution_center(strategy: DistributionStrategy) -> Box<dyn DistributionCenter> {
    match strategy {
        DistributionStrategy::Fs => Box::new(file_system::FileSystemStorage),
        DistributionStrategy::Sql => Box::new(sql_database::SqlDatabase::new()),
        DistributionStrategy::Http => Box::new(http_client::HttpClient),
    }
}
