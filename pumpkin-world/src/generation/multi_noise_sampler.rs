use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Map};

use crate::biome::Biome;

use super::noise::density::component_functions::SharedComponentReference;
use super::noise::density::NoisePos;

#[derive(Clone, Serialize, Deserialize)]
pub struct NoiseValuePoint {
    pub temperature: f64,
    pub erosion: f64,
    pub depth: f64,
    pub continents: f64,
    pub weirdness: f64,
    pub humidity: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NoiseValueRange {
    pub temperature: [f64; 2],
    pub erosion: [f64; 2],
    pub depth: [f64; 2],
    pub continents: [f64; 2],
    pub weirdness: [f64; 2],
    pub humidity: [f64; 2],
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct BiomeEntries {
    #[serde_as(as = "Map<_, _>")]
    nodes: Vec<(Biome, NoiseValuePoint)>,
}

pub struct MultiNoiseSampler {
    pub(crate) temperature: SharedComponentReference,
    pub(crate) erosion: SharedComponentReference,
    pub(crate) depth: SharedComponentReference,
    pub(crate) continents: SharedComponentReference,
    pub(crate) weirdness: SharedComponentReference,
    pub(crate) humidity: SharedComponentReference,
}

impl MultiNoiseSampler {
    pub fn sample(&self, pos: &NoisePos) -> NoiseValuePoint {
        NoiseValuePoint {
            temperature: self.temperature.sample(pos),
            erosion: self.erosion.sample(pos),
            depth: self.depth.sample(pos),
            continents: self.continents.sample(pos),
            weirdness: self.weirdness.sample(pos),
            humidity: self.humidity.sample(pos),
        }
    }
}

pub struct SearchTree<T> {
    root: TreeNode<T>,
}

pub enum TreeNode<T> {
    Leaf {
        value: T,
        point: NoiseValuePoint,
    },
    Branch {
        children: Vec<TreeNode<T>>,
        bounds: NoiseValueRange,
    },
}

impl<T> SearchTree<T> {
    pub fn new(entries: Vec<(T, NoiseValuePoint)>) -> Self {
        let root = Self::build_tree(entries);
        SearchTree { root }
    }

    fn build_tree(entries: Vec<(T, NoiseValuePoint)>) -> TreeNode<T> {
        if entries.len() == 1 {
            let (value, point) = entries.into_iter().next().unwrap();
            TreeNode::Leaf { value, point }
        } else {
            let mid = entries.len() / 2;
            let mut sorted_entries = entries.clone();
            sorted_entries
                .sort_by(|(_, a), (_, b)| (a.temperature).partial_cmp(&b.temperature).unwrap());
            let (value, point) = sorted_entries[mid].clone();
            let children = vec![
                TreeNode::Leaf { value, point },
                TreeNode::Leaf { value, point },
            ];
            TreeNode::Branch {
                children,
                bounds: NoiseValueRange {
                    temperature: [0.0, 1.0],
                    erosion: [0.0, 1.0],
                    depth: [0.0, 1.0],
                    continents: [0.0, 1.0],
                    weirdness: [0.0, 1.0],
                    humidity: [0.0, 1.0],
                },
            }
        }
    }

    pub fn find_best_match(&self, point: &NoiseValuePoint) -> Option<T> {
        self.find_best_match_recursive(&self.root, point)
    }

    fn find_best_match_recursive(&self, node: &TreeNode<T>, point: &NoiseValuePoint) -> Option<T> {
        match node {
            TreeNode::Leaf {
                value,
                point: node_point,
            } => {
                if point == node_point {
                    Some(value.clone())
                } else {
                    None
                }
            }
            TreeNode::Branch {
                children,
                bounds: _,
            } => {
                for child in children {
                    if let Some(result) = self.find_best_match_recursive(child, point) {
                        return Some(result);
                    }
                }
                None
            }
        }
    }
}

impl NoiseValuePoint {
    pub fn distance_squared(&self, other: &Self) -> f64 {
        let temp_diff = self.temperature - other.temperature;
        let erosion_diff = self.erosion - other.erosion;
        let depth_diff = self.depth - other.depth;
        let continents_diff = self.continents - other.continents;
        let weirdness_diff = self.weirdness - other.weirdness;
        let humidity_diff = self.humidity - other.humidity;

        temp_diff * temp_diff
            + erosion_diff * erosion_diff
            + depth_diff * depth_diff
            + continents_diff * continents_diff
            + weirdness_diff * weirdness_diff
            + humidity_diff * humidity_diff
    }
}

impl BiomeEntries {
    pub fn find_biome(&self, point: &NoiseValuePoint) -> Biome {
        let mut closest_biome = None;
        let mut min_distance = f64::MAX;

        for (biome, range) in &self.nodes {
            let distance = range.distance_squared(point);
            if distance < min_distance {
                min_distance = distance;
                closest_biome = Some(biome);
            }
        }

        closest_biome.unwrap_or(Biome::Plains) // Default biome if none matches.
    }
}

fn main() {
    // Example usage
    let biomes = vec![
        (
            Biome::Plains,
            NoiseValuePoint {
                temperature: 0.8,
                erosion: 0.3,
                depth: 0.1,
                continents: 0.5,
                weirdness: 0.4,
                humidity: 0.9,
            },
        ),
        (
            Biome::SnowyTiga,
            NoiseValuePoint {
                temperature: -0.5,
                erosion: 0.1,
                depth: 0.2,
                continents: 0.7,
                weirdness: 0.2,
                humidity: 0.4,
            },
        ),
    ];

    let search_tree = SearchTree::new(biomes);

    let query = NoiseValuePoint {
        temperature: 0.7,
        erosion: 0.3,
        depth: 0.1,
        continents: 0.5,
        weirdness: 0.4,
        humidity: 0.8,
    };

    let result = search_tree.find_best_match(&query);
    println!("{:?}", result);
}
impl NoiseValuePoint {
    /// Calculates the squared distance between two NoiseValuePoints.
    pub fn distance_squared(&self, other: &Self) -> f64 {
        let temp_diff = self.temperature - other.temperature;
        let erosion_diff = self.erosion - other.erosion;
        let depth_diff = self.depth - other.depth;
        let continents_diff = self.continents - other.continents;
        let weirdness_diff = self.weirdness - other.weirdness;
        let humidity_diff = self.humidity - other.humidity;

        temp_diff * temp_diff
            + erosion_diff * erosion_diff
            + depth_diff * depth_diff
            + continents_diff * continents_diff
            + weirdness_diff * weirdness_diff
            + humidity_diff * humidity_diff
    }
}

// Example usage:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biome_search_tree() {
        let biomes = vec![
            (
                Biome::Plains,
                NoiseValuePoint {
                    temperature: 0.8,
                    erosion: 0.3,
                    depth: 0.1,
                    continents: 0.5,
                    weirdness: 0.4,
                    humidity: 0.9,
                },
            ),
            (
                Biome::SnowyTiga,
                NoiseValuePoint {
                    temperature: -0.5,
                    erosion: 0.1,
                    depth: 0.2,
                    continents: 0.7,
                    weirdness: 0.2,
                    humidity: 0.4,
                },
            ),
        ];

        let search_tree = BiomeEntries::new(biomes);

        let query = NoiseValuePoint {
            temperature: 0.7,
            erosion: 0.3,
            depth: 0.1,
            continents: 0.5,
            weirdness: 0.4,
            humidity: 0.8,
        };

        let result = search_tree.find_biome(&query);
        assert_eq!(result, Biome::Plains);
    }
}
