#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

//Modules
pub mod physical;
pub mod logical;

//External Modules
use log::*;
use thiserror::Error;
use vulkanalia::vk;

/// The required device extensions.
pub const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];

#[derive(Debug, Error)]
#[error("{0}")]
pub struct SuitabilityError(pub &'static str);



