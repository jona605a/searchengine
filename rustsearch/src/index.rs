pub mod index6;
pub mod index7;




pub struct Index<T,G> {
    database: T,
    extra_variables: Option<G>,
}

#[derive(Debug)]
pub struct Index7ExtraVariables {
    article_titles: Vec<String>,
}



#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
}
