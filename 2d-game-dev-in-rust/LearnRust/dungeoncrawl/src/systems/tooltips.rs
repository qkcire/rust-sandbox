use crate::prelude::*;

#[system]
#[read_component(Points)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(
  ecs: &SubWorld,
  #[resource] mouse_pos: &Point,
  #[resource] camera: &Camera,
) {
  let mut positions = <(Entity, &Point, &Name)>::query();
}