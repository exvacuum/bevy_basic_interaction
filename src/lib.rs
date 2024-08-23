#![warn(missing_docs)]

//! This library provides basic "interaction" functionality
//! Entities which can trigger interactions get the `Interactor` component, and entities which can
//! Be interacted with get the `Interactable` component. An `InteractorFiredEvent` can be invoked for
//! a given `Interactor`, which does the following:
//!   1. Checks to make sure there is at least 1 `Interactable` in range
//!   2. If the nearest `Interactable` is "exclusive", an `InteractionEvent` is invoked for only that
//!      entity
//!   3. If the nearest `Interactable` is *not* "exclusive", an individual `InteractionEvent` is invoked for
//!      that entity and each "non-exclusive" `Interactable` entity within range

use std::f32::consts::PI;

use bevy::prelude::*;
use components::{Interactable, Interactor};
use events::{InteractionEvent, InteractorFiredEvent};

pub mod components;
pub mod events;

/// Plugin which enables interaction functionality.
/// Sets up event handling for `InteractorFiredEvent` to automatically trigger the correct
/// `InteractionEvent`s
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_interactor_events, update_interactor_targets))
            .add_event::<InteractorFiredEvent>()
            .add_event::<InteractionEvent>();
    }
}

fn handle_interactor_events(
    mut interactor_events: EventReader<InteractorFiredEvent>,
    interactable_query: Query<&Interactable>,
    interactor_query: Query<&Interactor>,
    mut event_writer: EventWriter<InteractionEvent>,
) {
    for InteractorFiredEvent(interactor_entity) in interactor_events.read() {
        let interactor = interactor_query.get(*interactor_entity).unwrap();

        if let Some(interactable_entity) = interactor.closest {
            let interactable = interactable_query.get(interactable_entity).unwrap();
            if interactable.exclusive {
                event_writer.send(InteractionEvent {
                    interactor: *interactor_entity,
                    interactable: interactable_entity,
                });
                continue;
            } else {
                for interactable_entity in &interactor.targets {
                    let interactable = interactable_query.get(*interactable_entity).unwrap();
                    if !interactable.exclusive {
                        event_writer.send(InteractionEvent {
                            interactor: *interactor_entity,
                            interactable: *interactable_entity,
                        });
                    }
                }
            }
        }
    }
}

fn update_interactor_targets(
    mut interactable_query: Query<(Entity, &Interactable)>,
    mut interactor_query: Query<(Entity, &mut Interactor)>,
    transform_query: Query<&GlobalTransform>,
) {
    for (interactor_entity, mut interactor) in interactor_query.iter_mut() {
        let interactor_transform = transform_query.get(interactor_entity).unwrap();

        let mut closest_active_interactable: Option<(f32, Entity)> = None;
        for (interactable_entity, interactable) in interactable_query.iter_mut() {
            let interactable_transform = transform_query.get(interactable_entity).unwrap();
            let interactable_distance_squared = interactable_transform
                .translation()
                .distance_squared(interactor_transform.translation());
            let interactable_arccosine = f32::acos(
                interactor_transform.forward().dot(
                    (interactable_transform.translation() - interactor_transform.translation())
                        .normalize(),
                ),
            );
            if interactable_distance_squared < interactable.max_distance_squared
                && interactable_arccosine < PI / 8.0
            {
                interactor.targets.insert(interactable_entity);
                if let Some((arccosine, _)) = closest_active_interactable {
                    if interactable_arccosine < arccosine {
                        closest_active_interactable =
                            Some((interactable_arccosine, interactable_entity));
                    }
                } else {
                    closest_active_interactable =
                        Some((interactable_arccosine, interactable_entity));
                }
            } else {
                interactor.targets.remove(&interactable_entity);
            }
        }
        interactor.closest = if let Some((_, interactable_entity)) = closest_active_interactable {
            Some(interactable_entity)
        } else {
            None
        }
    }
}
