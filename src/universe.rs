pub mod universe {
    use bevy::prelude::{debug, Res};
    use petgraph::prelude::*;

    #[derive(Debug, Default)]
    pub struct Galaxy(Vec<u32>);

    impl Galaxy {
        pub fn new() -> Self {
            Galaxy::default()
        }

        pub fn from_file(_file_path: &str) -> Self {
            println!("TODO: Load Galaxy from file");
            let graph = random_graph(5, EdgeProbability(0.9));
            println!("Random graph generated: {:?}", graph);
            Galaxy::new()
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Default)]
    pub struct SolarSystem {
        star: String,
        planets: Vec<Planet>,
    }

    #[derive(Debug, Default)]
    struct Planet((String, Option<Vec<Station>>));

    #[derive(Debug, Default)]
    struct Station(String);

    #[derive(Debug, Default)]
    struct EdgeProbability(f32);

    #[allow(dead_code)]
    impl EdgeProbability {
        fn new(p: f32) -> Self {
            EdgeProbability(p.clamp(0., 1.))
        }

        fn get(&self) -> f32 {
            self.0
        }
    }

    pub fn debug_universe(universe: Res<Galaxy>) {
        debug!("Debug Universe {:?}", universe);
    }

    fn random_graph(order: u32, probability: EdgeProbability) -> UnGraph<u32, ()> {
        let mut edges = Vec::<(u32, u32)>::new();
        for i in 0..=order {
            for j in i..=order {
                if i != j {
                    if rand::random::<f32>() < probability.get() {
                        edges.push((i, j));
                    }
                }
            }
        }

        UnGraph::from_edges(edges)
    }
}
