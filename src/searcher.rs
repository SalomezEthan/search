pub struct Line<'a> {
    pub number : usize,
    pub content : &'a str
}

/// Trouve les lignes contenant la chaine demandée.
pub fn search<'a>(query : &'a str, buffer : &'a str) -> impl Iterator<Item = Line<'a>> {
    buffer  
        .lines()
        .enumerate()
        .filter(move |(_, line)| line.contains(query))
        .map(|(index, line)| Line { number : real_line_number(index), content : line })
}

fn real_line_number(index : usize) -> usize {
    index + 1
}

#[cfg(test)]
mod test {
    use crate::searcher::search;

    const BUFFER : &str = "Je suis un buffer.\n J'ai actuellement 2 lignes !";

    #[test]
    fn buffer_has_2_lines() {
        let lines : Vec<&str> = BUFFER
            .lines()
            .collect();

        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn search_return_line_2() {
        let query = "lignes";
        let mut iter = search(query, BUFFER);
        let line = iter.next();
        
        assert!(line.is_some());
        assert_eq!(line.unwrap().number, 2);
    }
}
