use std::collections::HashMap;
use std::rc::Rc;

use crate::contig::{AssignedMoleculeType, SequenceRole};

use super::contig::Contig;

pub struct GenomicAssembly {
    name: String,
    // TODO - add other genomic assembly members
    gen_bank_accession: String,
    contigs: Vec<Rc<Contig>>,
    contig_by_name: HashMap<String, usize>,
}

fn get_unknown_contig() -> Contig {
    Contig::new(0,
                "".to_string(),
                SequenceRole::AssembledMolecule,
                "".to_string(),
                AssignedMoleculeType::Chromosome,
                0,
                "".to_string(),
                "".to_string(),
                "".to_string())
}

impl GenomicAssembly {
    pub fn new(name: String,
               gen_bank_accession: String,
               contigs: Vec<Contig>) -> GenomicAssembly {
        let (contigs, contig_by_name) = GenomicAssembly::check(contigs);

        GenomicAssembly {
            name,
            gen_bank_accession,
            contigs,
            contig_by_name,
        }
    }

    fn check(contigs: Vec<Contig>) -> (Vec<Rc<Contig>>, HashMap<String, usize>) {
        if contigs.is_empty() {
            panic!("Contigs must not be empty")
        }

        // no contig can have ID = 0
        if let Some(contig) = contigs.iter().find(|ctg| ctg.id() == 0) {
            panic!("Contig cannot have ID 0: {:?}", contig)
        }

        // add the unknown contig and wrap in Rc
        let mut contigs: Vec<_> = vec![get_unknown_contig()].into_iter()
            .chain(contigs.into_iter())
            .map(|contig| Rc::new(contig))
            .collect();

        // contig IDs must be consecutive
        contigs.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        let last_contig = contigs.last().unwrap();
        let last_contig_id = last_contig.id();
        if last_contig_id != contigs.len() - 1 {
            for contig in &contigs {
                eprintln!("{:?}", contig)
            }

            panic!("Contig IDs must be consecutive integers (Expected to see `{:?}` as the last contig ID, but the ID of {:?} was `{:?}`)",
                   contigs.len() - 1, last_contig.name(), last_contig_id)
        }


        let contig_by_name: HashMap<String, usize> = contigs.iter()
            .enumerate()
            .map(|(idx, contig)| (contig.name().to_string(), idx))
            .collect();

        (contigs, contig_by_name)
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn gen_bank_accession(&self) -> &str {
        self.gen_bank_accession.as_str()
    }

    pub fn contig_by_id(&self, id: usize) -> Option<&Rc<Contig>> {
       self.contigs.get(id)
    }

    pub fn contig_by_name(&self, name: &str) -> Option<&Rc<Contig>> {
        match self.contig_by_name.get(name) {
            Some(idx) => self.contigs.get(*idx),
            None => None
        }
    }
}

// ---------------------------- ITERATING THROUGH CONTIGS ------------------------------------------

pub struct GenomicAssemblyIterator<'a> {
    assembly: &'a GenomicAssembly,
    id: usize,
}

// immutable iteration
impl<'a> IntoIterator for &'a GenomicAssembly {
    type Item = &'a Rc<Contig>;
    type IntoIter = GenomicAssemblyIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GenomicAssemblyIterator {
            assembly: &self,
            id: 0,
        }
    }
}

impl<'a> Iterator for GenomicAssemblyIterator<'a> {
    type Item = &'a Rc<Contig>;

    fn next(&mut self) -> Option<Self::Item> {
        let this = self.assembly.contigs.get(self.id);
        self.id += 1;
        this
    }
}

// ------------------------------------------ TESTS ------------------------------------------------

#[cfg(test)]
mod assembly_tests {
    use super::*;
    use super::super::test_data::get_contigs;

    #[test]
    fn test_new() {
        let assembly = get_assembly();

        assert_eq!(assembly.name(), "GRCh38.p12");
        assert_eq!(assembly.gen_bank_accession(), "GCA_000001405.28");
        let one = assembly.contig_by_name("1");
        assert!(one.is_some());

        let one = one.unwrap();
        assert_eq!(one.id(), 1);
        assert_eq!(one.name(), "1");
        assert_eq!(one.gen_bank_accession(), "CM000663.1");
        assert_eq!(one.length(), 249_250_621);

        let mt = assembly.contig_by_name("MT");
        assert!(mt.is_some());

        let mt = mt.unwrap();
        assert_eq!(mt.id(), 5);
        assert_eq!(mt.name(), "MT");
        assert_eq!(mt.gen_bank_accession(), "J01415.2");
        assert_eq!(mt.length(), 16_569);
    }

    #[test]
    fn contig_by_id() {
        let assembly = get_assembly();

        let zero = assembly.contig_by_id(0);
        assert!(zero.is_some());
        let zero = zero.unwrap();
        assert!(zero.is_unknown());
        assert_eq!(zero.name(), "");

        let two = assembly.contig_by_id(2);
        assert!(two.is_some());
        let two = two.unwrap();
        assert!(!two.is_unknown());
        assert_eq!(two.name(), "2");

        let q = assembly.contig_by_id(99);
        assert!(q.is_none());
    }

    #[test]
    fn contig_by_name() {
        let assembly = get_assembly();

        let y = assembly.contig_by_name("Y");
        assert!(y.is_some());
        let y = y.unwrap();
        assert_eq!(y.name(), "Y");

        let two = assembly.contig_by_name("Q");
        assert!(two.is_none());
    }

    #[test]
    fn iterate() {
        let assembly = get_assembly();

        for contig in &assembly {
            println!("Contig: {:?}", contig)
        }
        // TODO make this a test
    }

    fn get_assembly() -> GenomicAssembly {
        let name = "GRCh38.p12".to_string();
        let gen_bank_accession = "GCA_000001405.28".to_string();
        GenomicAssembly::new(name, gen_bank_accession, get_contigs())
    }
}