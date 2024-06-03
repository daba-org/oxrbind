use oxr_binding::{XrResult, XrSessionCreateFlags, XrStructureType, XrSystemId, XrViewConfigurationType};

use crate::core::space::{ReferenceSpaceCreateInfo, Space};

use super::space::MySpace;

#[derive(Debug)]
pub struct SessionBeginInfo {
    pub type_: XrStructureType,
    pub next: Option<std::os::raw::c_void>,
    pub primary_view_configuration_type: XrViewConfigurationType,
}

#[derive(Debug)]
pub struct SessionCreateInfo {
    pub type_: XrStructureType,
    pub next: Option<std::os::raw::c_void>,
    pub create_flags: XrSessionCreateFlags,
    pub system_id: XrSystemId,
}


pub trait Session {
    fn new(create_info: SessionCreateInfo) -> Result<Self, XrResult>
        where
            Self: Sized;

    fn clean_up(&mut self) -> Result<(), XrResult>;

    fn begin(&self, begin_info: SessionBeginInfo) -> Result<(), XrResult>;

    fn end(&self) -> Result<(), XrResult>;

    fn request_exit(&self) -> Result<(), XrResult>;

    fn create_reference_space(
        &self,
        create_info: ReferenceSpaceCreateInfo,
    ) -> impl Space + Sized;

    // fn create_action_space(
    //     &self,
    //     create_info: ActionSpaceCreateInfo
    // ) -> impl Space + Sized;
}

pub struct MySession {}

impl Session for MySession {
    fn new(create_info: SessionCreateInfo) -> Result<Self, XrResult>
        where
            Self: Sized,
    {
        todo!()
    }

    fn clean_up(&mut self) -> Result<(), XrResult> {
        todo!()
    }

    fn begin(&self, begin_info: SessionBeginInfo) -> Result<(), XrResult> {
        todo!()
    }

    fn end(&self) -> Result<(), XrResult> {
        todo!()
    }

    fn request_exit(&self) -> Result<(), XrResult> {
        todo!()
    }

    fn create_reference_space(&self, create_info: ReferenceSpaceCreateInfo) -> impl Space + Sized {
        MySpace::new(create_info)
    }
}