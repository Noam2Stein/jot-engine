pub use bevy_ecs::{
    self,
    prelude::{
        Added, Command, Commands, Component, Entity, IntoScheduleConfigs, Mut, Query, QueryBuilder,
        QueryState, Ref, RemovedComponents, Res, ResMut, Resource, Schedule, Schedules, System,
        With, Without,
    },
};

pub type Ecs = bevy_ecs::prelude::World;
