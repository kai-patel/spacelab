pub mod universe {
    use bevy::prelude::{Res, debug};
    use petgraph::prelude::*;
    pub type Galaxy = UnGraph<SolarSystem, u8>;

    #[derive(Debug, Default)]
    pub struct SolarSystem {
        star: String,
        planets: Vec<Planet>,
    }

    #[derive(Debug, Default)]
    struct Planet((String, Option<Vec<Station>>));

    #[derive(Debug, Default)]
    struct Station(String);

    pub fn debug_universe(universe: Res<Galaxy>) {
        debug!("{:?}", universe);
    }
}
