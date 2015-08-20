use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub enum GitError {
    IoError(io::Error)
}

impl From<io::Error> for GitError {
    fn from(err: io::Error) -> GitError {
        GitError::IoError(err)
    }
}

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

    pub fn containing_file(path: &Path) -> Result<Repo, GitError> {
        Ok(Repo { workdir: try!(find_repo_root(path)) })
    }
}

fn find_repo_root(path: &Path) -> Result<PathBuf, io::Error> {
    let mut pb = path.to_path_buf();

    loop {
        let mut git_dir = pb.clone();
        git_dir.push(".git");

        let md = try!(fs::metadata(git_dir.as_path()));
        if md.is_dir() {
            return Ok(pb);
        } else {
            pb.pop();
        }
    }
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
