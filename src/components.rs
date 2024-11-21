//! Components used for interactions

use bevy::{prelude::*, utils::HashSet};

/// Component which enables an entity to request interactions.
///
/// An entity with an `Interactor` component can be passed to an `InteractorFiredEvent` in order to
/// start an interaction with any nearby `Interactable` entities.
#[derive(Component, Default, Debug)]
pub struct Interactor {
    /// All `Interactable` targets in-range of this interactor.
    pub targets: HashSet<Entity>,
    /// The closest `Interactable` target to this interactor, if any.
    pub closest: Option<Entity>,
}

/// Component which enables an entity to recieve interactions.
///
/// An entity with an `Interactable` component might get passed to an `InteractionEvent` when an
/// `Interactor` requests an interaction, if the interactable is in range.
#[derive(Component, Clone, Debug)]
pub struct Interactable {
    /// An optional name for this interactable
    pub name: Option<String>,
    /// An optional description of the action
    pub description: Option<String>,
    /// Predicate to check to see if interaction is possible
    pub predicate: Option<fn(Entity, &mut World) -> bool>,
    pub(crate) exclusive: bool,
    pub(crate) max_distance_squared: f32,
    pub(crate) possible: bool,
    /// Whether this pickup is enabled
    pub enabled: bool,
}

impl Interactable {
    /// Construct a new instance of this component.
    ///
    /// If exclusive, this interactable will only be interacted with if it's the closest one to the
    /// interactor, and the interaction will *not* be processed for any other in-range
    /// interactables.
    pub fn new(max_distance: f32, exclusive: bool, name: Option<String>, description: Option<String>, predicate: Option<fn(Entity, &mut World) -> bool>) -> Self {
        Self {
            name,
            description,
            predicate,
            exclusive,
            max_distance_squared: max_distance * max_distance,
            possible: true,
            enabled: true,
        }
    }

    /// Gets whether this interaction is currently possible. Set this value using predicate.
    pub fn possible(&self) -> bool {
        self.possible
    }
}

impl Default for Interactable {
    fn default() -> Self {
        Self::new(1.0, false, None, None, None)
    }
}

