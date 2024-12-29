use std::sync::LazyLock;

use pumpkin_data::chunk::Biome;

use crate::{
    coordinates::BlockCoordinates,
    generation::{
        biome_search_tree::{BiomeEntries, SearchTree},
        noise_router::multi_noise_sampler::MultiNoiseSampler,
    },
};

pub static BIOME_ENTRIES: LazyLock<BiomeEntries<Biome>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../../assets/multi_noise.json"))
        .expect("Could not parse synced_registries.json registry.")
});

pub trait BiomeSupplier {
    fn biome(&self, at: BlockCoordinates) -> Biome;
}

#[derive(Clone)]
pub struct DebugBiomeSupplier;

impl BiomeSupplier for DebugBiomeSupplier {
    fn biome(&self, _at: BlockCoordinates) -> Biome {
        Biome::Plains
    }
}

pub struct MultiNoiseBiomeSupplier<'a> {
    noise: MultiNoiseSampler<'a>,
}

impl BiomeSupplier for MultiNoiseBiomeSupplier<'_> {
    fn biome(&self, at: BlockCoordinates) -> Biome {
        BIOME_ENTRIES.find_biome(&self.noise.sample(at.x, at.y.0 as i32, at.z))
    }
}
