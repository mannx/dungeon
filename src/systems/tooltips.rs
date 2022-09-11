use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse: &Point, #[resource] camera: &Camera) {
    let mut pos = <(Entity, &Point, &Name)>::query();

    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse + offset;

    let mut batch = DrawBatch::new();
    batch.target(2);

    pos.iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos)
        .for_each(|(entity, _, name)| {
            let screen = *mouse * 4;
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };

            batch.print(screen, &display);
        });

    batch.submit(10100).expect("batch error");
}
