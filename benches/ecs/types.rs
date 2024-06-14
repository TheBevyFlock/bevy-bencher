use bevy::ecs::prelude::*;

/// A 32-bit wide component.
///
/// This is intended to not match with [`B`]'s size, to force the ECS to deal with padding.
#[derive(Component, Clone)]
pub struct A(#[allow(dead_code)] pub u32);

/// A 16-bit wide component.
///
/// This is intended to not match with [`A`]'s size, to force the ECS to deal with padding.
#[derive(Component, Clone)]
pub struct B(#[allow(dead_code)] pub u16);
