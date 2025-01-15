use std::sync::LazyLock;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::generation::{
    multi_noise_sampler::MultiNoiseSampler,
    noise::density::{NoisePos, UnblendedNoisePos},
    GeneratorInit, Seed,
};

pub static BIOME_ENTRIES: LazyLock<BiomeEntries> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../../assets/multi_noise.json"))
        .expect("Could not parse synced_registries.json registry.")
});

// TODO make this work with the protocol
// Send by the registry
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Biome {
    #[serde(rename = "minecraft:plains")]
    Plains,
    #[serde(rename = "minecraft:snowy_taiga")]
    SnowyTaiga,
    // TODO list all Biomes
}

pub trait BiomeSupplier {
    fn biome(&self, x: i32, y: i32, z: i32, noise: &MultiNoiseSampler) -> Biome;
}

#[derive(Clone)]
pub struct DebugBiomeSupplier;

impl BiomeSupplierImpl for DebugBiomeSupplier {
    fn biome(&self, _x: i32, _y: i32, _z: i32, _noise: &MultiNoiseSampler) -> Biome {
        Biome::Plains
    }
}

#[derive(Clone)]
pub struct MultiNoiseBiomeSupplier;

impl BiomeSupplierImpl for MultiNoiseBiomeSupplier {
    fn biome(&self, x: i32, y: i32, z: i32, noise: &MultiNoiseSampler) -> Biome {
        BIOME_ENTRIES
            .find_biome(&noise.sample(&NoisePos::Unblended(UnblendedNoisePos::new(x, y, z))))
    }
}

pub(crate) struct SuperflatBiomeGenerator {}

impl GeneratorInit for SuperflatBiomeGenerator {
    fn new(_: Seed) -> Self {
        Self {}
    }
}
