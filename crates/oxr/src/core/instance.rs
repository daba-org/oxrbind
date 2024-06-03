use std::{ffi::CStr, mem::transmute, os::raw::c_char};

use oxr_binding::*;

use crate::core::system::SystemGetInfo;

use super::{action_set::{ActionSet, ActionSetCreateInfo, MyActionSet}, session::{MySession, Session, SessionCreateInfo}};

#[derive(Debug)]
struct ApplicationInfo {
    pub application_name: String,
    pub application_version: u32,
    pub engine_name: String,
    pub engine_version: u32,
    pub api_version: XrVersion,
}

#[derive(Debug)]
struct InstanceCreateInfo {
    pub type_: XrStructureType,
    pub next: Option<*const std::os::raw::c_void>,
    pub create_flags: XrInstanceCreateFlags,
    pub application_info: ApplicationInfo,
    pub enabled_api_layer_count: u32,
    pub enabled_api_layer_names: Vec<String>,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: Vec<String>,
}

#[derive(Debug)]
struct ExtensionProperties {
    pub type_: XrStructureType,
    pub next: Option<std::os::raw::c_void>,
    pub extension_name: String,
    pub extension_version: u32,
}

#[derive(Debug)]
struct ApiLayerProperties {
    pub type_: XrStructureType,
    pub next: Option<std::os::raw::c_void>,
    pub layer_name: String,
    pub spec_version: XrVersion,
    pub layer_version: u32,
    pub description: String,
}

#[derive(Debug)]
struct ViewConfigurationProperties {
    pub type_: XrStructureType,
    pub next: Option<std::os::raw::c_void>,
    pub view_configuration_type: XrViewConfigurationType,
    pub fox_mutable: XrBool32,
}

#[derive(Debug)]
struct SystemProperties {
    pub type_: XrStructureType,
    pub next: Option<std::os::raw::c_void>,
    pub system_id: XrSystemId,
    pub vendor_id: u32,
    pub system_name: String,
    pub graphics_properties: XrSystemGraphicsProperties,
    pub tracking_properties: XrSystemTrackingProperties,
}


trait Instance {
    fn new(create_info: InstanceCreateInfo) -> Result<Self, XrResult>
        where
            Self: Sized;

    fn clean_up(&mut self) -> Result<(), XrResult>;

    fn enumerate_api_layer_properties(
        &self,
    ) -> Result<&Vec<ApiLayerProperties>, XrResult>;

    fn enumerate_api_layer_properties_owned(
        &self,
    ) -> Result<&Vec<ApiLayerProperties>, XrResult> {
        self.enumerate_api_layer_properties().clone()
    }

    fn enumerate_instance_extension_properties(
        &self,
        layer_name: String,
    ) -> Result<&Vec<ExtensionProperties>, XrResult>;

    fn enumerate_instance_extension_properties_owned(
        &self,
        layer_name: String,
    ) -> Result<&Vec<ExtensionProperties>, XrResult> {
        self.enumerate_instance_extension_properties(layer_name).clone()
    }

    fn create_session(
        &self,
        create_info: SessionCreateInfo,
    ) -> impl Session + Sized;

    fn enumerate_view_configurations(
        &self,
        system_id: XrSystemId,
    ) -> Result<Vec<XrViewConfigurationType>, XrResult>;

    fn get_view_configuration_properties(
        &self,
        system_id: XrSystemId,
        view_configuration_type: XrViewConfigurationType,
    ) -> Result<ViewConfigurationProperties, XrResult>;

    fn get_system(
        &self,
        get_info: SystemGetInfo,
    ) -> XrSystemId;

    fn get_system_properties(
        &self,
        system_id: XrSystemId,
    ) -> SystemProperties;

    fn enumerate_environment_blend_modes(
        &self,
        system_id: XrSystemId,
        view_configuration_type: XrViewConfigurationType,
    ) -> Result<Vec<XrEnvironmentBlendMode>, XrResult>;

    fn create_action_set(
        &self,
        create_info: ActionSetCreateInfo,
    ) -> Result<impl ActionSet, XrResult>;
}

// #[xr_instance]
#[derive(Default, Debug)]
struct MyXrInstance {
    instance_extensions: Vec<ExtensionProperties>,
    api_layers: Vec<ApiLayerProperties>,
}

impl Instance for MyXrInstance {
    fn new(create_info: InstanceCreateInfo) -> Result<Self, XrResult>
        where
            Self: Sized,
    {
        Ok(Default::default())
    }

    fn clean_up(&mut self) -> Result<(), XrResult> {
        Ok(())
    }

    fn enumerate_api_layer_properties(
        &self,
    ) -> Result<&Vec<ApiLayerProperties>, XrResult> {
        Ok(&self.api_layers)
    }

    fn enumerate_instance_extension_properties(
        &self,
        layer_name: String,
    ) -> Result<&Vec<ExtensionProperties>, XrResult> {
        Ok(&self.instance_extensions)
    }

    fn create_session(
        &self,
        create_info: SessionCreateInfo,
    ) -> impl Session + Sized {
        MySession::new(create_info).unwrap()
    }

    fn enumerate_view_configurations(&self, system_id: XrSystemId) -> Result<Vec<XrViewConfigurationType>, XrResult> {
        todo!()
    }

    fn get_view_configuration_properties(&self, system_id: XrSystemId, view_configuration_type: XrViewConfigurationType) -> Result<ViewConfigurationProperties, XrResult> {
        todo!()
    }

    fn get_system(&self, get_info: SystemGetInfo) -> XrSystemId {
        todo!()
    }

    fn get_system_properties(&self, system_id: XrSystemId) -> SystemProperties {
        todo!()
    }

    fn enumerate_environment_blend_modes(&self, system_id: XrSystemId, view_configuration_type: XrViewConfigurationType) -> Result<Vec<XrEnvironmentBlendMode>, XrResult> {
        todo!()
    }

    fn create_action_set(
        &self,
        create_info: ActionSetCreateInfo,
    ) -> Result<impl ActionSet, XrResult> {
        Ok(MyActionSet::new(create_info))
    }
}


fn c_char_array_to_string(c_char_array: &[c_char; 128]) -> String {
    // Convert the slice to a CStr
    let c_str = unsafe { CStr::from_ptr(c_char_array.as_ptr()) };

    // Convert CStr to a String
    c_str.to_string_lossy().into_owned()
}

fn c_str_vec_to_safe(api_layers: *const *const c_char) -> Vec<String> {
    let mut vec = Vec::new();
    if api_layers.is_null() {
        return vec;
    }

    let mut current = api_layers;
    unsafe {
        while !(*current).is_null() {
            let c_str = CStr::from_ptr(*current);
            if let Ok(str_slice) = c_str.to_str() {
                vec.push(str_slice.to_string());
            }
            current = current.add(1);
        }
    }

    vec
}

#[no_mangle]
pub unsafe extern "C" fn xrCreateInstance(
    create_info: *const XrInstanceCreateInfo,
    instance: *mut XrInstance,
) -> XrResult {
    let unsafe_info = *create_info;
    let info = InstanceCreateInfo {
        type_: unsafe_info.type_,
        next: None,
        create_flags: unsafe_info.createFlags,
        application_info: ApplicationInfo {
            application_name: c_char_array_to_string(&unsafe_info.applicationInfo.applicationName),
            application_version: unsafe_info.applicationInfo.applicationVersion,
            engine_name: c_char_array_to_string(&unsafe_info.applicationInfo.engineName),
            engine_version: unsafe_info.applicationInfo.engineVersion,
            api_version: unsafe_info.applicationInfo.apiVersion,
        },
        enabled_api_layer_count: unsafe_info.enabledApiLayerCount,
        enabled_api_layer_names: c_str_vec_to_safe(unsafe_info.enabledApiLayerNames),
        enabled_extension_count: unsafe_info.enabledExtensionCount,
        enabled_extension_names: c_str_vec_to_safe(unsafe_info.enabledExtensionNames),
    };

    let new_instance = MyXrInstance::new(info);
    let new_instance = match new_instance {
        Ok(new_instance) => Box::new(new_instance),
        Err(xr_code) => return xr_code,
    };
    let raw_ptr = Box::into_raw(new_instance);
    *instance = raw_ptr as *mut XrInstance_T;

    XrResult_XR_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn xrDestroyInstance(instance: *mut XrInstance) -> XrResult {
    let my_instance = instance as *mut MyXrInstance;
    let mut my_instance = Box::from_raw(my_instance);
    match my_instance.as_mut().clean_up() {
        Err(xr_code) => return xr_code,
        _ => ()
    }
    drop(my_instance);

    XrResult_XR_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn xrEnumerateApiLayerProperties(
    property_capacity_input: u32,
    property_count_output: *mut u32,
    properties: *mut XrApiLayerProperties,
) -> XrResult {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn xrEnumerateInstanceExtensionProperties(
    layer_name: *const ::std::os::raw::c_char,
    property_capacity_input: u32,
    property_count_output: *mut u32,
    properties: *mut XrExtensionProperties,
) -> XrResult {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn xrGetInstanceProcAddr(
    instance: XrInstance,
    name: *const ::std::os::raw::c_char,
    function: *mut PFN_xrVoidFunction,
) -> XrResult {
    if instance as u32 == XR_NULL_HANDLE {
        let name = CStr::from_ptr(name).to_string_lossy().into_owned();

        match name.as_str() {
            "xrEnumerateInstanceExtensionProperties" => {
                let func_ptr = Some(xrEnumerateInstanceExtensionProperties as unsafe extern "C" fn(
                    *const c_char,
                    u32,
                    *mut u32,
                    *mut XrExtensionProperties,
                ) -> XrResult);
                *function = Some(transmute(func_ptr));
                return XrResult_XR_SUCCESS;
            }
            "xrEnumerateApiLayerProperties" => {
                let func_ptr = Some(xrEnumerateApiLayerProperties as unsafe extern "C" fn(
                    u32,
                    *mut u32,
                    *mut XrApiLayerProperties,
                ) -> XrResult);
                *function = Some(transmute(func_ptr));
                return XrResult_XR_SUCCESS;
            }
            "xrCreateInstance" => {
                let func_ptr = xrCreateInstance as unsafe extern "C" fn(
                    *const XrInstanceCreateInfo,
                    *mut XrInstance,
                ) -> XrResult;
                *function = transmute(Some(func_ptr));
                return XrResult_XR_SUCCESS;
            }
            _ => {
                *function = None;
                return XrResult_XR_SUCCESS;
            }
        }
    }

    return XrResult_XR_SUCCESS;
}
