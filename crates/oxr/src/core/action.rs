use oxr_binding::*;

#[derive(Debug)]
pub struct ActionCreateInfo {
    pub type_: XrStructureType,
    pub next: Option<std::os::raw::c_void>,
    pub action_name: String,
    pub action_type: XrActionType,
    pub count_subaction_paths: u32,
    pub subaction_paths: Vec<XrPath>,
    pub localized_action_name: String,
}

pub trait Action {
    fn new(create_info: ActionCreateInfo) -> Self where Self: Sized;

    fn clean_up(&mut self) -> Result<(), XrResult>;
}