#![allow(dead_code, unused_variables, clippy::too_many_arguments, clippy::unnecessary_wraps)]

//External Modules
use std::mem::size_of;

use nalgebra_glm as glm;
use vulkanalia::prelude::v1_0::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pos: glm::Vec2,
    color: glm::Vec3,
}

impl Vertex {
    pub fn new(pos: glm::Vec2, color: glm::Vec3) -> Self {
        Self { pos, color }
    }

    pub fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<Vertex>()  as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        let pos = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0)
            .build();
        let color = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(size_of::<glm::Vec2>() as u32)
            .build();
        [pos, color]
    }
}