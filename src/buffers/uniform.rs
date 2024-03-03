#![allow(dead_code, unused_variables, clippy::too_many_arguments, clippy::unnecessary_wraps)]

//External Modules
use std::mem::size_of;

use anyhow::Result;
use nalgebra_glm as glm;
use vulkanalia::prelude::v1_0::*;

//Internal Modules
use crate::data::AppData;
use crate::buffers::create_buffer;

//================================================
// Structs
//================================================
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UniformBufferObject {
    pub model: glm::Mat4,
    pub view: glm::Mat4,
    pub proj: glm::Mat4,
}

//================================================
// Buffers
//================================================

pub unsafe fn create_uniform_buffers(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
    data.uniform_buffers.clear();
    data.uniform_buffers_memory.clear();

    for _ in 0..data.swapchain_images.len() {
        let (uniform_buffer, uniform_buffer_memory) = create_buffer(
            instance,
            device,
            data,
            size_of::<UniformBufferObject>() as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        data.uniform_buffers.push(uniform_buffer);
        data.uniform_buffers_memory.push(uniform_buffer_memory);
    }

    Ok(())
}