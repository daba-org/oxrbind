use oxr_binding::{XrFormFactor, XrStructureType};

#[derive(Debug)]
pub struct SystemGetInfo {
    type_: XrStructureType,
    next: Option<*const std::os::raw::c_void>,
    form_factor: XrFormFactor,
}

pub trait System {
    fn new(get_info: SystemGetInfo) -> Self
        where
            Self: Sized;
}
