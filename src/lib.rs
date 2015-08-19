use std::path::PathBuf;
use std::path::Path;

#[derive(Clone)]
pub struct Repo {
    workdir: PathBuf,
}

impl Repo {
    pub fn at(path: &Path) -> Repo {
        Repo { workdir: PathBuf::from(path) }
    }

    pub fn workdir(&self) -> &Path {
        &self.workdir
    }

    //pub fn for_contained_file(path: &Path) -> Repo {
    //    Repo { workdir: PathBuf::from(path) }
    //}
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use Repo;

    #[test]
    fn construct_known_workdir() {
        assert_eq!(Repo::at(Path::new("/git/path")).workdir(), Path::new("/git/path"))
    }
}
