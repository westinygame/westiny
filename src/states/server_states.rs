use westiny_common::events::WestinyEvent;
use amethyst::prelude::*;
use amethyst::core::Time;
use westiny_server::resources::{ClientRegistry, NetworkIdSupplier};
use westiny_server::resources::collision;

use log::info;
use std::path::PathBuf;
use derive_new::new;
use westiny_common::resources::map::build_map;
use westiny_common::components::Projectile;
use westiny_common::resources::Seed;

#[derive(new)]
pub struct ServerState {
    resources: PathBuf,
}

impl ServerState {
    fn place_objects(&self, world: &mut World, seed: Seed) {
        build_map(world, seed, &self.resources.join("map"))
            .expect("Map could not be created");
    }
}

fn log_fps(time: &Time) {
    if time.frame_number() % 60 == 0 {
        // Note: this is not an average, calculated from the last frame delta.
        info!("FPS: {}", 1.0 / time.delta_real_seconds());
    }
}


fn log_clients(time: &Time, registry: &ClientRegistry) {
    if time.frame_number() % 600 == 0 {
        log::info!("Number of players online: {}", registry.client_count());
        log::debug!("{}", &registry);
    }
}


impl State<GameData<'static, 'static>, WestinyEvent> for ServerState {
    fn on_start(&mut self, data: StateData<'_, GameData<'static, 'static>>) {
        const MAGIC_SEED: u64 = 0;

        let seed = Seed(MAGIC_SEED);
        data.world.insert(ClientRegistry::new(16));
        data.world.insert(NetworkIdSupplier::new());
        data.world.insert(collision::Collisions::default());
        data.world.insert(collision::ProjectileCollisions::default());
        data.world.insert(seed);

        data.world.register::<Projectile>();

        self.place_objects(data.world, seed);
    }

    fn update(&mut self, data: StateData<'_, GameData<'static, 'static>>) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        let time = *data.world.fetch::<Time>();
        log_fps(&time);
        data.data.update(&data.world);
        log_clients(&time, &data.world.fetch::<ClientRegistry>());
        Trans::None
    }
}
