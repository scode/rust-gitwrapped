extern crate tempdir;

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

    // Create a Repo for manipulating the git repository that contains
    // the given file. Unless the given path is the repository root
    // itself, parents are successfully probed until what looks like a
    // valid git repository is found.
    pub fn containing_file(path: &Path) -> Result<Repo, GitError> {
        Ok(Repo { workdir: try!(find_repo_root(path)) })
    }
}

fn find_repo_root(path: &Path) -> Result<PathBuf, io::Error> {
    // TODO(scode): Fix false negative, incorrect positive.
    //
    // With the API:s in stable rust at the time of this writing, we
    // don't seem to be able to tell the difference between I/O errors
    // and a file not existing. As a result, this function will
    // potentially give incorrect results if, while probing for a .git
    // directory, we encounter an I/O error.
    //
    // Depending on whether the offending .git is contained within
    // another directory, this may lead to either a false negative, or
    // an incorrect positive.

    let mut pb = path.to_path_buf();

    loop {
        // Return with a failure once we reach a path that doesn't
        // exist. This triggers in one of two cases. The first case is
        // if we are passed a directory that does not exist. The
        // second case is the presence of concurrent file system
        // modification under our feet.
        try!(fs::metadata(pb.as_path()));

        let mut git_dir = pb.clone();
        git_dir.push(".git");

        let git_dir_meta = fs::metadata(git_dir.as_path());
        match git_dir_meta {
            Ok(m) => {
                if m.is_dir() {
                    return Ok(pb);
                } else {
                    pb.pop();
                }
            },
            Err(_) => { pb.pop(); }
        }

        // If we've reached the root, and the root wasn't in fact a
        // git repository, we couldn't find one.
        if pb.parent().is_none() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "no .git directotry was found in any parent"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;
    use tempdir::TempDir;
    use Repo;

    #[test]
    fn at() {
        assert_eq!(Repo::at(Path::new("/git/path")).workdir(), Path::new("/git/path"))
    }

    #[test]
    fn containing_file() {
        let tmpdir = TempDir::new("gitwrapped-test").unwrap();
        let tmppath = tmpdir.path();

        // Make a path buf representing a path relative to tmppath,
        // with relative path components passed as a vector.
        let p = |path: Vec<&str>| -> PathBuf {
            let mut ret = tmppath.to_path_buf();
            for path_comp in path {
                ret.push(path_comp);
            }
            ret
        };

        assert!(fs::create_dir_all(p(vec!(".git"))).is_ok());
        assert!(fs::create_dir_all(p(vec!("sub1", "sub2", "sub3"))).is_ok());

        let repo_root_buf = p(vec!());
        let repo_root = repo_root_buf.as_path();

        // The repo root itself.
        assert_eq!(Repo::containing_file(p(vec!()).as_path()).unwrap().workdir(), repo_root);

        // An existing sub directory.
        assert_eq!(Repo::containing_file(p(vec!("sub1")).as_path()).unwrap().workdir(), repo_root);

        // An existing sub-sub directory.
        assert_eq!(Repo::containing_file(p(vec!("sub1", "sub2")).as_path()).unwrap().workdir(), repo_root);

        // A non-existing directory (should fail).
        assert!(Repo::containing_file(p(vec!("sub1", "sub2", "non-existent")).as_path()).is_err());

        // XXX(scode): Missing test: Root directory (would fail if /.git exists).
        // XXX(scode): Missing test: Concurrent modification to cause non-existence while traversing up parents.
    }
}
