#[derive(Debug, PartialEq)]
enum Type {
    DNA,
    RNA,
}

#[derive(Debug, PartialEq)]
enum Nucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
    Uracil,
    Error,
}

fn parse(input: &str, t: Type) -> Result<Vec<Nucleotide>, usize> {
    input
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            'A' => Ok(Nucleotide::Adenine),
            'T' => {
                if t == Type::DNA {
                    Ok(Nucleotide::Thymine)
                } else {
                    Err(i)
                }
            }
            'U' => {
                if t == Type::RNA {
                    Ok(Nucleotide::Uracil)
                } else {
                    Err(i)
                }
            }
            'C' => Ok(Nucleotide::Cytosine),
            'G' => Ok(Nucleotide::Guanine),
            _ => Err(i),
        })
        .collect()
}

#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleotides: Vec<Nucleotide>,
}

impl DNA {
    pub fn new(strand: &str) -> Result<Self, usize> {
        Ok(Self {
            nucleotides: parse(strand, Type::DNA)?,
        })
    }

    pub fn into_rna(self) -> RNA {
        let mut result = Vec::with_capacity(self.nucleotides.len());

        for n in self.nucleotides {
            result.push(match n {
                Nucleotide::Cytosine => Nucleotide::Guanine,
                Nucleotide::Guanine => Nucleotide::Cytosine,
                Nucleotide::Adenine => Nucleotide::Uracil,
                Nucleotide::Thymine => Nucleotide::Adenine,
                _ => Nucleotide::Error,
            });
        }

        RNA {
            nucleotides: result,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleotides: Vec<Nucleotide>,
}

impl RNA {
    pub fn new(strand: &str) -> Result<Self, usize> {
        Ok(Self {
            nucleotides: parse(strand, Type::RNA)?,
        })
    }
}
