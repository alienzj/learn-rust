use bio::alphabets;
use bio::data_structures::suffix_array::suffix_array;
use bio::data_structures::bwt::{bwt, less, Occ};
use bio::data_structures::fmindex::{FMIndex, FMIndexable};

let text = b"ACGGATGCTGGATCGGATCGCGCTAGCTA$";
let pattern = b"ACGG";

// Create an FM-Index for a given text.
let alphabet = alphabets::dna::iupac_alphabet();
let sa = suffix_array(text);
let bwt = bwt(text, &sa);
let less = less(&bwt, &alphabet);
let occ = Occ::new(&bwt, 3, &alphabet);
let fmindex = FMIndex::new(&bwt, &less, &occ);

let interval = fmindex.backward_search(pattern.iter());
let positions = interval.occ(&sa);
