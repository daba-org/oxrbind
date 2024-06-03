use core::option::Option;

use oxr_binding::{
    XrPath, XrPosef, XrReferenceSpaceType, XrResult, XrSpaceLocation,
    XrSpaceLocationFlags, XrStructureType, XrTime,
};

use crate::core::action::Action;

#[derive(Debug)]
pub struct ReferenceSpaceCreateInfo {
    pub type_: XrStructureType,
    pub next: Option<std::os::raw::c_void>,
    pub reference_space_type: XrReferenceSpaceType,
    pub pose_in_reference_space: XrPosef,
}

#[derive(Debug)]
pub struct SpaceLocation {
    pub type_: XrStructureType,
    pub next: Option<std::os::raw::c_void>,
    pub location_flags: XrSpaceLocationFlags,
    pub pose: XrPosef,
}

pub struct ActionSpaceCreateInfo<'a> {
    pub type_: XrStructureType,
    pub next: Option<std::os::raw::c_void>,
    pub action: &'a dyn Action,
    pub subaction_path: XrPath,
    pub pose_in_action_space: XrPosef,
}

pub trait Space {
    fn new(create_info: ReferenceSpaceCreateInfo) -> Self
        where
            Self: Sized;

    fn clean_up(&mut self) -> Result<(), XrResult>;

    fn locate_space(
        &self,
        base_space: &impl Space,
        time: XrTime,
    ) -> Result<XrSpaceLocation, XrResult>;
}

pub struct MySpace {}

impl Space for MySpace {
    fn new(create_info: ReferenceSpaceCreateInfo) -> Self
        where
            Self: Sized {
        todo!()
    }

    fn clean_up(&mut self) -> Result<(), XrResult> {
        todo!()
    }

    fn locate_space(
        &self,
        base_space: &impl Space,
        time: XrTime,
    ) -> Result<XrSpaceLocation, XrResult> {
        todo!()
    }
}
