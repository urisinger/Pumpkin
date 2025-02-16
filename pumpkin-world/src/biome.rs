use std::{borrow::BorrowMut, cell::RefCell, sync::LazyLock};

use pumpkin_data::chunk::Biome;

use crate::{
    coordinates::BlockCoordinates,
    generation::{
        biome_search_tree::{BiomeEntries, SearchTree, TreeLeafNode},
        noise_router::multi_noise_sampler::MultiNoiseSampler,
    },
};

pub static BIOME_ENTRIES: LazyLock<SearchTree<Biome>> = LazyLock::new(|| {
    SearchTree::create(
        serde_json::from_str::<BiomeEntries>(include_str!("../../assets/multi_noise.json"))
            .expect("Could not parse synced_registries.json registry.")
            .nodes,
    )
    .expect("entries cannot be empty")
});

thread_local! {
    static LAST_RESULT_NODE: RefCell<Option<TreeLeafNode<Biome>>> = RefCell::new(None);
}

pub trait BiomeSupplier {
    fn biome(&mut self, at: BlockCoordinates) -> Biome;
}

#[derive(Clone)]
pub struct DebugBiomeSupplier;

impl BiomeSupplier for DebugBiomeSupplier {
    fn biome(&mut self, _at: BlockCoordinates) -> Biome {
        Biome::Plains
    }
}

pub struct MultiNoiseBiomeSupplier<'a> {
    noise: MultiNoiseSampler<'a>,
}

impl BiomeSupplier for MultiNoiseBiomeSupplier<'_> {
    fn biome(&mut self, at: BlockCoordinates) -> Biome {
        let point = self.noise.sample(at.x, at.y.0 as i32, at.z);
        LAST_RESULT_NODE
            .with_borrow_mut(|last_result| BIOME_ENTRIES.get(&point, last_result).expect("a"))
    }
}
