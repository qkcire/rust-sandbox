use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims : Vec<(Entity, Entity)> = attackers// (1)
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim) )// (2)
        .collect();// (3)

    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Player>()
            .is_ok();
        
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>() 
            {
                healt.current -= 1;
                if health.current < 1 && !is_Player {
                    commands.remove(*victim);
                }
            }
        commands.remove(*message);
    });
}
