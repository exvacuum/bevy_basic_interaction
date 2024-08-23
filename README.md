# bevy_basic_interaction

![License](https://img.shields.io/badge/license-0BSD%2FMIT%2FApache-blue.svg)
![Tag](https://img.shields.io/github/v/tag/exvacuum/bevy_basic_interaction)
![Build](https://img.shields.io/github/actions/workflow/status/exvacuum/bevy_basic_interaction/rust.yml)

This crate provides an `InteractionPlugin` which enables a basic interaction system.

This plugin is mostly for my own internal use, but feel free to use it or contribute if you feel like it.

## Compatibility

| Crate Version | Bevy Version |
|---            |---           |
| 0.1           | 0.14         |

## Installation

If enough people (like literally 1 person) want this on crates.io I'll consider putting it up there, but for now just add the GitHub URL to your `Cargo.toml`:

```toml
[dependencies.bevy_basic_interaction]
git = "https://github.com/exvacuum/bevy_basic_interaction"
```

## Usage

This plugin works based on a system of `Interactor`s and `Interactable`s. `Interactor`s maintain a list of "target" `Interactable`s which are in range of the interactor (based on a maximum distance defined in the *interactable*). The user can invoke an `InteractorFiredEvent`, passing the `Entity` with an `Interactor` component on it, and the plugin will automatically determine which interactables to invoke a corresponding `InteractionEvent` for.

In main function:
```rs
use bevy::prelude::*;
use bevy_basic_interaction::InteractionPlugin;

fn main() {
    App::new()
        // ...
        .add_plugins(InteractionPlugin)
        // ...
        .run();
}
```

Raising an interactor event (`entity` must be an entity with both an `Interactor` and a `GlobalTransform` component):
```rs
// ...
event_writer.send(InteractorFiredEvent(entity));
// ...
```

Handling an interaction event:
```rs
// ...
for InteractionEvent { interactable, .. } in event_reader.read() {
    let interactable = interactable_query.get(interactable).unwrap();
    // Do something with interactable here...
}
// ...
```

## License

This crate is licensed under your choice of 0BSD, Apache-2.0, or MIT license.

