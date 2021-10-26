use std::path::PathBuf;
use derive_new::new;
// use westiny_common::resources::map::build_map;
// use westiny_common::events::WestinyEvent;

#[derive(new)]
pub struct ServerState {
    resources: PathBuf,
}

impl ServerState {
    // fn place_objects(&self, commands: &mut Commands, seed: Seed) {
    //     build_map(commands, seed, &self.resources.join("map"))
    //         .expect("Map could not be created");
    // }
}






// impl State<GameData<'static, 'static>, WestinyEvent> for ServerState {
//     fn on_start(&mut self, data: StateData<'_, GameData<'static, 'static>>) {
//         const MAGIC_SEED: u64 = 0;
//
//         let seed = Seed(MAGIC_SEED);
//         data.world.insert(ClientRegistry::new(16));
//         data.world.insert(NetworkIdSupplier::new());
//
//         data.world.insert(seed);
//
//         GunResource::initialize(data.world, self.resources.clone()).expect("Unable to initialize gun assets");
//
//         self.place_objects(data.world, seed);
//     }
//
//     fn update(&mut self, data: StateData<'_, GameData<'static, 'static>>) -> Trans<GameData<'static, 'static>, WestinyEvent> {
//         let time = *data.world.fetch::<Time>();
//         log_fps(&time);
//         data.data.update(&data.world);
//         log_clients(&time, &data.world.fetch::<ClientRegistry>());
//         Trans::None
//     }
// }
