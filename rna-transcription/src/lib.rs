#[derive(Debug, PartialEq, Eq)]
pub struct Dna{
    nucleotides: Vec<Nucleotide>
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotides: Vec<Nucleotide>
}

#[derive(Debug, PartialEq, Eq)]
enum Nucleotide {
    G,
    C,
    A,
    T,
    U
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut nucleotides: Vec<Nucleotide> = vec![];
        
        for (i, n) in dna.chars().enumerate() {
            if let Ok(res) = Nucleotide::into_dna(&n) {
                nucleotides.push(res)
            } else {
                return Err(i)
            }
        };

        Ok(Dna {
            nucleotides
        })
    }

    pub fn into_rna(self) -> Rna {
        let nucleotides = self
            .nucleotides
            .into_iter()
            .map(|n| n.encode())
            .collect();

        Rna {
            nucleotides
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut nucleotides: Vec<Nucleotide> = vec![];
        
        for (i, n) in rna.chars().enumerate() {
            if let Ok(res) = Nucleotide::into_rna(&n) {
                nucleotides.push(res)
            } else {
                return Err(i)
            }
        };

        Ok(Rna{
            nucleotides
        })
    }
}

impl Nucleotide {
    fn into_dna(n: &char) -> Result<Self, ()> {
        match n {
            'G' => Ok(Self::G),
            'C' => Ok(Self::C),
            'T' => Ok(Self::T),
            'A' => Ok(Self::A),
            _ => Err(())
        }
    }

    fn into_rna(n: &char) -> Result<Self, ()> {
        match n {
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'A' => Ok(Self::A),
            'U' => Ok(Self::U),
            _ => Err(())
        }
    }

    fn encode(&self) -> Self {
        match self {
            Self::G => Self::C,
            Self::C => Self::G,
            Self::T => Self::A,
            Self::A => Self::U,
            _ => unimplemented!()
        }
    }
}