//! Events which enable interactions between `Interactor` and `Interactable` entities.

use bevy::prelude::*;

/// Event sent by user to request an interaction from the given `Interactor` entity.
#[derive(Event)]
pub struct InteractorFiredEvent(pub Entity);

/// Event sent by the plugin once an `InteractorFiredEvent` has been processed. It should be caught
/// by the user to perform some action on the affected interactable entity.
///
/// It is not intended to be invoked directly.
#[derive(Event)]
pub struct InteractionEvent {
    /// `Interactor` entity which triggered this interaction.
    pub interactor: Entity,
    /// `Interactable` entity whicg is receiving this interaction.
    pub interactable: Entity,
}
