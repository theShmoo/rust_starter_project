use super::super::lib::built_info;

pub fn hello() {
    println!(
    "{name} (v{version})\n{description}\n\nbuild by {author}.",
    version = built_info::PKG_VERSION,
    name = built_info::PKG_NAME,
    description = built_info::PKG_DESCRIPTION,
    author = built_info::PKG_AUTHORS
);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {

        hello();
    }
}