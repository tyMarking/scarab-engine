use std::fmt::Debug;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use serde::{Deserialize, Serialize};

use crate::{
    gameobject::{
        entity::registry::{EntityRegistry, GameTickArgs, RegisteredEntity},
        field::Field,
        HasSolidity,
    },
    rendering::{registry::TextureRegistry, View},
    Camera, HasBox, HasBoxMut, PhysBox, ScarabResult,
};

#[derive(Debug, Serialize, Deserialize)]
/// A wrapper over all things in the app right now
pub struct Scene<E, V> {
    field: Field,
    field_view: V,
    entity_registry: EntityRegistry<E>,
    #[serde(skip)]
    #[serde(default = "Vec::new")]
    pending_attacks: Vec<PendingAttack<E>>,
}

impl<E, V> Scene<E, V>
where
    E: RegisteredEntity + Debug + 'static,
    V: View<Viewed = Field>,
{
    /// Initializes a new scene with the given field, field view and no entities
    pub fn new(field: Field, field_view: V) -> Self {
        Self {
            field,
            field_view,
            entity_registry: EntityRegistry::default(),
            pending_attacks: Vec::default(),
        }
    }

    /// Renders everything in the scene
    pub fn render(
        &mut self,
        args: &RenderArgs,
        camera: &Camera,
        ctx: Context,
        texture_registry: &TextureRegistry,
        gl: &mut GlGraphics,
    ) -> ScarabResult<()> {
        self.field_view
            .render(&mut self.field, args, &camera, ctx, texture_registry, gl)?;

        for registered_entity in &mut self.entity_registry {
            registered_entity.render(args, camera, ctx, texture_registry, gl)?;
        }
        Ok(())
    }

    /// Registers a new entity to the scene
    pub fn register_entity(&mut self, to_register: E) -> ScarabResult<()> {
        self.entity_registry.register(to_register)
    }

    /// Gets a reference to the scene's [Field]
    pub fn get_field(&self) -> &Field {
        &self.field
    }

    /// Runs the physics update for all of the scene's entities
    pub fn tick_entities(&mut self, dt: f64) -> ScarabResult<()> {
        let mut args = GameTickArgs {
            field: &self.field,
            pending_attacks: &mut self.pending_attacks,
            dt,
        };
        for (i, registered_entity) in self.entity_registry.iter_mut().enumerate() {
            registered_entity.game_tick(i, &mut args)?;
        }

        self.handle_entity_collisions()?;

        self.process_pending_attacks()?;

        Ok(())
    }

    // TODO! Find a way to pin the return type of this to a specific type within the registry
    /// Optionally returns a mutable reference to the scene's player
    pub fn player_mut<'e, 's: 'e>(&mut self) -> Option<&mut E::Player<'e, 's>> {
        self.entity_registry.player_mut()
    }

    fn handle_entity_collisions(&mut self) -> ScarabResult<()> {
        // This is kinda gross, but I don't really know how else to do it
        // we'll see later how necessary it is to change
        for this_index in 0..self.entity_registry.len() {
            if let Some(this_one) = self.entity_registry.get_one(this_index) {
                if !this_one.inner_entity().get_solidity().has_solidity() {
                    continue;
                }

                let this_one_box = *this_one.inner_entity().get_box();

                for other_index in 0..this_index {
                    if this_index == other_index {
                        continue;
                    }
                    if let Some(other_one) = self.entity_registry.get_one_mut(other_index) {
                        if other_one.inner_entity().get_solidity().has_solidity() {
                            other_one
                                .inner_entity_mut()
                                .get_box_mut()
                                .shift_to_nonoverlapping(&this_one_box);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn process_pending_attacks(&mut self) -> ScarabResult<()> {
        let _ = self.pending_attacks.drain_filter(|attack| {
            let keep_attack = self
                .entity_registry
                .iter_mut()
                .enumerate()
                .filter_map(|(i, e)| {
                    // TODO! remove inefficient retrieval of overlapping entities
                    // Do not attack if it's the source and the source can't be targeted
                    if !(!attack.can_target_src && i == attack.src_idx)
                        && e.inner_entity().get_box().has_overlap(&attack.target_area)
                    {
                        let res = attack.attack.do_attack(e).ok();
                        Some(res).flatten()
                    } else {
                        None
                    }
                })
                .any(|b| b);
            self.entity_registry
                .get_one_mut(attack.src_idx)
                .map(|src| attack.attack.update_src(src))
                .or_else(|| {
                    println!(
                        "error processing attack: could not find source entity: {:?}",
                        attack
                    );
                    None
                });

            // Drain filter *REMOVES* when true
            !keep_attack
        });

        Ok(())
    }
}

#[derive(Debug)]
/// An attack that the scene should process on the next game tick
pub struct PendingAttack<E> {
    /// The entity registry index of the attack's source entity
    pub src_idx: usize,
    /// Should the attack be able to target the source entity
    pub can_target_src: bool,
    /// The attack's target area
    /// TODO: this could be changed into a more generalized "AttackTarget" which could just
    /// get the nearest "n" entities within a range for example
    pub target_area: PhysBox,
    /// Handles the logic of doing the attack on the enemy
    pub attack: Box<dyn Attack<E>>,
}

/// Attacks that can be done targeting an enemy
pub trait Attack<E>: Debug {
    /// Apply the main effect of the attack to the target entity (i.e. do damage, apply status effects, etc.)
    /// Returns whether or not the attack needs to process on the next tick
    fn do_attack(&mut self, target: &mut E) -> ScarabResult<bool>;

    /// Apply any necessary updates to the source of the attack
    /// This could be animation states, draining energy or any other necessary effect
    fn update_src(&mut self, src: &mut E) -> ScarabResult<()>;
}
