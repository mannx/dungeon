use crate::prelude::*;
use legion::world::SubWorld;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut batch = DrawBatch::new();
    batch.target(1);

    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            batch.set(*pos - offset, render.color, render.glyph);
        });

    batch.submit(5000).expect("Batch Error")
}
