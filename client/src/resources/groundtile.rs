use amethyst::tiles::Tile;
use amethyst::ecs::World;
use amethyst::core::math::Point3;

#[derive(Default, Clone)]
pub struct GroundTile;
impl Tile for GroundTile {
    fn sprite(&self, point: Point3<u32>, _: &World) -> Option<usize> {
        Some(((point.x + point.y) % 2) as usize)
    }
}

