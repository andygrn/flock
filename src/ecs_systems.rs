use specs::{ReadStorage, Fetch, System, Entities};
use ecs_entities::Utterance;
use time::Tm;

pub struct GameTimeNow(pub Tm);

pub struct UtteranceSystem;

impl<'a> System<'a> for UtteranceSystem {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, Utterance>,
                       Fetch<'a, GameTimeNow>);

    fn run(&mut self, (entities, utterances, now): Self::SystemData) {
        use specs::Join;
        (&*entities, &utterances).join().for_each(|(entity, utterance)| {
            if utterance.dead_at < now.0 {
                entities.delete(entity).unwrap();
            }
        });
    }
}
