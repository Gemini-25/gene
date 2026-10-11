//! Size of a saved state's sparse parts: tiles with dropped records, pages, parcels, agents.
//! usage: state_stats SAVE.bin
fn main() {
    let sim = gene_core::Sim::load(&std::fs::read(std::env::args().nth(1).unwrap()).unwrap()).unwrap();
    let st = &sim.st;
    let pages: u64 = st.dropped.values().flatten().map(|m| m.present_pages() as u64).sum();
    let media: usize = st.dropped.values().map(|v| v.len()).sum();
    let occ = st.agents.len();
    println!(
        "tick {} agents {} | tiles with records {} ({:.1}% of {}) media {} pages {} | parcels {} | residue tiles {}",
        st.tick,
        occ,
        st.dropped.len(),
        100.0 * st.dropped.len() as f64 / st.world.n() as f64,
        st.world.n(),
        media,
        pages,
        st.world.parcels.len(),
        st.world.residue.len()
    );
}
