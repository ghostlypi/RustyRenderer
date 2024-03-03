#![allow(dead_code, unused_variables, clippy::too_many_arguments, clippy::unnecessary_wraps)]

//External Modules
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

//Internal Modules
use crate::data::AppData;

//================================================
// Framebuffers
//================================================

pub unsafe fn create_framebuffers(device: &Device, data: &mut AppData) -> Result<()> {
    data.framebuffers = data
        .swapchain_image_views
        .iter()
        .map(|i| {
            let attachments = &[*i];
            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(data.render_pass)
                .attachments(attachments)
                .width(data.swapchain_extent.width)
                .height(data.swapchain_extent.height)
                .layers(1);

            device.create_framebuffer(&create_info, None)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}