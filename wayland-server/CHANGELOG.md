# CHANGELOG: wayland-server

## Unreleased

## 0.30.0-beta.4

#### Breaking changes

- `Resource::post_error` no longer requires a `&mut DisplayHandle`

#### Additions

- Introduce `Resource::client_id`

## 0.30.0-beta.2

#### Breaking changes

- `delegate_dispatch!` can no longer delegate multiple interfaces at once, in order to properly support
  generic delegate base types.

## 0.30.0-beta.1

#### Breaking changes

- Large rework of the API as a consequence of the rework of the backend.

## 0.30.0-alpha10

- Introduce conversion methods between `wayland_backend::Handle` and `DisplayHandle`

## 0.30.0-alpha7

- Introduce `DataInit::custom_init()`

## 0.30.0-alpha5

- Introduce `Display::backend()`

## 0.30.0-alpha4

#### Breaking changes

- The trait `DestructionNotify` is removed, and replaced by a `Dispatch::destroyed()` method.

## 0.30.0-alpha2

#### Breaking changes

- The `DelegateDispatch` mechanism is changed around an explicit trait-base extraction of module
  state from the main compositor state.
- The `DisplayHandle` no longer has a type parameter
- Global manipulation methods are moved from `DisplayHandle` to `Display`

## 0.30.0-alpha1

Full rework of the crate, which is now organized around a trait-based `Dispatch` metchanism.

This can effectively be considered a new crate altogether.
