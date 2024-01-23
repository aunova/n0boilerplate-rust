use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Hello, world!");
    Ok(())
}

// #[cfg(test)] omitted because we want test linting
mod tests {
    #![allow(clippy::used_underscore_binding)]

    #[allow(clippy::wildcard_imports)]
    use rstest::*;

    #[fixture]
    #[once]
    fn setup() {
        #[allow(clippy::unwrap_used)]
        color_eyre::install().unwrap();
        // Add any other setup here

    }

    #[rstest]
    fn demo_failing_test(_setup: ()) {
        panic!("This demos a failing test");
    }
}
