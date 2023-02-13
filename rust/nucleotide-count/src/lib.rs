use std::collections::HashMap;

fn valid_nuc(nuc: char) -> bool {
    matches!(nuc, 'A' | 'C' | 'G' | 'T')
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !valid_nuc(nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;
    for c in dna.chars() {
        if !valid_nuc(c) {
            return Err(c);
        }

        if c == nucleotide {
            count += 1;
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hm = HashMap::<char, usize>::new();
    hm.insert('A', 0);
    hm.insert('C', 0);
    hm.insert('G', 0);
    hm.insert('T', 0);
    for n in dna.chars() {
        match n {
            'A' | 'C' | 'G' | 'T' => {
                hm.entry(n).and_modify(|e| *e += 1);
            }
            _ => return Err(n),
        }
    }
    Ok(hm)
}
