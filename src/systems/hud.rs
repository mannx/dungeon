use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn hud(ecs: &SubWorld) {
    let mut query = <&Health>::query().filter(component::<Player>());
    let health = query.iter(ecs).nth(0).unwrap();

    let mut batch = DrawBatch::new();
    batch.target(2);
    batch.print_centered(1, "Explore the dungeon. Cursoe to move");
    batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        health.current,
        health.max,
        ColorPair::new(RED, BLACK),
    );
    batch.print_color_centered(
        0,
        format!("Health: {}/{}", health.current, health.max),
        ColorPair::new(WHITE, RED),
    );
    batch.submit(10000).expect("batch error");
}
