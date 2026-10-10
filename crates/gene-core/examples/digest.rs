//! Layout-independent physics digest used to verify that refactors keep results identical.
use gene_core::sim::{Config, Sim};
fn main() {
    let prog = gene_core::isa::assemble(gene_core::FOUNDER5_ASM).unwrap();
    let mut sim = Sim::with_program(Config { seed: 5, width: 128, height: 128, agents: 64 }, &prog).unwrap();
    for _ in 0..6000 { sim.step(); }
    let w = &sim.st.world;
    let mut h = blake3::Hasher::new();
    for t in 0..w.n() as u32 {
        for s in 0..10 { h.update(&w.loose_total(t, s).to_le_bytes()); }
        h.update(&w.heat_u_at(t).to_le_bytes());
        h.update(&w.heat_c_at(t).to_le_bytes());
        h.update(&w.surface_c_at(t).to_le_bytes());
        h.update(&w.rx_credit_at(t).iter().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>());
    }
    for a in &sim.st.agents {
        h.update(&a.uid.to_le_bytes()); h.update(&a.x.to_le_bytes()); h.update(&a.y.to_le_bytes());
        h.update(&a.energy.to_le_bytes()); h.update(&a.photo_credit.to_le_bytes());
        for v in a.inv { h.update(&v.to_le_bytes()); }
        for v in a.mem { h.update(&v.to_le_bytes()); }
        for v in a.vm.regs { h.update(&v.to_le_bytes()); }
    }
    println!("pop {} births {} digest {}", sim.st.agents.len(), sim.obs.total_births, h.finalize().to_hex());
}
