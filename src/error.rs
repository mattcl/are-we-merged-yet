use clap;
use git2;

error_chain! {
    foreign_links {
        IoError(::std::io::Error);
        GitError(git2::Error);
    }

    errors {
        RepoEmpty {
            description("repo is empty")
            display("repo is empty")
        }
        InvalidReference {
            description("git reference is invalid")
            display("git reference is invalid")
        }
        MissingSubcommand(command: String) {
            description("missing subcommand")
            display("missing subcommand: {}", command)
        }
    }
}

pub trait UnwrapOrExit<T>
    where Self: Sized
{
    fn unwrap_or_else<F>(self, f: F) -> T where F: FnOnce() -> T;

    fn unwrap_or_exit(self, message: &str) -> T {
        let err = clap::Error::with_description(message, clap::ErrorKind::InvalidValue);
        self.unwrap_or_else(|| err.exit())
    }
}

impl<T> UnwrapOrExit<T> for Option<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
        where F: FnOnce() -> T
    {
        self.unwrap_or_else(f)
    }
}

impl<T> UnwrapOrExit<T> for Result<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
        where F: FnOnce() -> T
    {
        self.unwrap_or_else(|_| f())
    }

    fn unwrap_or_exit(self, message: &str) -> T {
        self.unwrap_or_else(|e| {
            let err = clap::Error::with_description(&format!("{}: {}", message, e),
                                                    clap::ErrorKind::InvalidValue);
            err.exit()
        })
    }
}
