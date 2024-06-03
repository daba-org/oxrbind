use oxr_binding::{XrResult, XrStructureType};

#[derive(Debug)]
pub struct ActionSetCreateInfo {
    pub type_: XrStructureType,
    pub next: Option<std::os::raw::c_void>,
    pub action_set_name: String,
    pub localized_action_set_name: String,
    pub priority: u32,
}

pub trait ActionSet {
    fn new(create_info: ActionSetCreateInfo) -> Self;

    fn clean_up(&mut self) -> Result<(), XrResult>;
}

pub struct MyActionSet {}

impl ActionSet for MyActionSet {
    fn new(create_info: ActionSetCreateInfo) -> Self {
        todo!()
    }

    fn clean_up(&mut self) -> Result<(), XrResult> {
        todo!()
    }
}