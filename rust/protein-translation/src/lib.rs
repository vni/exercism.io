pub struct CodonsInfo<'a>(Vec<(&'a str, &'a str)>);

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        for pair in self.0.iter() {
            if pair.0 == codon {
                return Some(pair.1);
            }
        }
        None
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        if rna.is_empty() {
            return None;
        }

        let mut res = Vec::<&str>::new();
        for c in rna.as_bytes().chunks(3) {
            if c.len() != 3 {
                return None;
            }

            let name = self.name_for(std::str::from_utf8(c).unwrap())?;
            if name == "stop codon" {
                break;
            }

            res.push(name);
        }

        Some(res)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo(pairs)
}
