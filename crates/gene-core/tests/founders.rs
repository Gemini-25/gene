//! The public ancestors assemble to the documented sizes and are valid genomes.
use gene_core::{genome, isa};

fn check(src: &str, words: usize, cells: usize) {
    let p = isa::assemble(src).unwrap();
    assert_eq!(p.words.len(), words);
    let c = p.to_cells();
    assert_eq!(c.len(), cells);
    assert_eq!(genome::parse(&c).unwrap(), p);
}

#[test]
fn founders_assemble_to_documented_sizes() {
    check(gene_core::FOUNDER_ASM, 369, 762);
    check(gene_core::FOUNDER5_ASM, 250, 524);
    check(gene_core::FOUNDER6_ASM, 248, 520);
    check(gene_core::FOUNDER7_ASM, 291, 606);
    check(gene_core::FOUNDER8_ASM, 349, 722);
    check(gene_core::FOUNDER9_ASM, 397, 818);
    check(gene_core::FOUNDER10_ASM, 340, 704);
    check(gene_core::FOUNDER8M_ASM, 356, 760);
    check(gene_core::FOUNDER9M_ASM, 415, 878);
    check(gene_core::FOUNDER10M_ASM, 358, 764);
}
