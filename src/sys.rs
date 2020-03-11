//! Raw bindings to the COM API of the WebView2 SDK.

#![allow(clippy::missing_safety_doc)]

use com::{com_interface, interfaces::IUnknown, ComPtr};
use std::ffi::c_void;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;
use winapi::shared::windef::*;

#[com_interface("7ED79562-90E1-47CD-A4E0-01D9211D7E3D")]
pub trait ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler: IUnknown {
    unsafe fn invoke(
        &self,
        result: HRESULT,
        created_environment: *mut *mut ICoreWebView2EnvironmentVTable,
    ) -> HRESULT;
}

#[com_interface("7dc2ec84-56cb-4fcc-b4c6-a9f85c7b2894")]
pub trait ICoreWebView2Environment: IUnknown {
    unsafe fn create_core_webview2_host(
        &self,
        parent_window: HWND,
        handler: ComPtr<dyn ICoreWebView2CreateCoreWebView2HostCompletedHandler>,
    ) -> HRESULT;
    // Rest TODO.
}

#[com_interface("E09F5D38-91E3-49D1-8182-70A616AA06B9")]
pub trait ICoreWebView2CreateCoreWebView2HostCompletedHandler: IUnknown {
    unsafe fn invoke(
        &self,
        result: HRESULT,
        created_host: *mut *mut ICoreWebView2HostVTable,
    ) -> HRESULT;
}

#[com_interface("6ddf7138-a19b-4e55-8994-8a198b07f492")]
pub trait ICoreWebView2Host: IUnknown {
    unsafe fn get_is_visible(&self, is_visible: *mut BOOL) -> HRESULT;
    unsafe fn put_is_visible(&self, is_visible: BOOL) -> HRESULT;
    unsafe fn get_bounds(&self, bounds: *mut RECT) -> HRESULT;
    unsafe fn put_bounds(&self, bounds: RECT) -> HRESULT;
    // Some declarations are not complete. They are only placeholders.
    unsafe fn get_zoom_factor(&self);
    unsafe fn put_zoom_factor(&self);
    unsafe fn add_zoom_factor_changed(&self);
    unsafe fn remove_zoom_factor_changed(&self);
    unsafe fn set_bounds_and_zoom_factor(&self);
    unsafe fn move_focus(&self, reason: MoveFocusReason) -> HRESULT;
    unsafe fn add_move_focus_requested(&self);
    unsafe fn remove_move_focus_requested(&self);
    unsafe fn add_got_focus(&self);
    unsafe fn remove_got_focus(&self);
    unsafe fn add_lost_focus(&self);
    unsafe fn remove_lost_focus(&self);
    unsafe fn add_accelerator_key_pressed(&self);
    unsafe fn remove_accelerator_key_pressed(&self);
    unsafe fn get_parent_window(&self);
    unsafe fn put_parent_window(&self);
    unsafe fn notify_parent_window_position_changed(&self) -> HRESULT;
    unsafe fn close(&self) -> HRESULT;
    unsafe fn get_core_webview2(&self, webview2: *mut *mut *mut ICoreWebView2VTable) -> HRESULT;
}

#[com_interface("5cc5293d-af6f-41d4-9619-44bd31ba4c93")]
pub trait ICoreWebView2: IUnknown {
    unsafe fn get_settings(&self, settings: *mut *mut *mut ICoreWebView2SettingsVTable) -> HRESULT;
    unsafe fn get_source(&self);
    unsafe fn navigate(&self, uri: LPCWSTR) -> HRESULT;
    unsafe fn navigate_to_string(&self, html_content: LPCWSTR) -> HRESULT;
    unsafe fn add_navigation_starting(&self);
    unsafe fn remove_navigation_starting(&self);
    unsafe fn add_content_loading(&self);
    unsafe fn remove_content_loading(&self);
    unsafe fn add_source_changed(&self);
    unsafe fn remove_source_changed(&self);
    unsafe fn add_history_changed(&self);
    unsafe fn remove_history_changed(&self);
    unsafe fn add_navigation_completed(&self);
    unsafe fn remove_navigation_completed(&self);
    unsafe fn add_frame_navigation_starting(&self);
    unsafe fn remove_frame_navigation_starting(&self);
    unsafe fn add_script_dialog_opening(&self);
    unsafe fn remove_script_dialog_opening(&self);
    unsafe fn add_permission_requested(&self);
    unsafe fn remove_permission_requested(&self);
    unsafe fn add_process_failed(&self);
    unsafe fn remove_process_failed(&self);
    unsafe fn add_script_to_execute_on_document_created(&self);
    unsafe fn remove_script_to_execute_on_document_created(&self);
    unsafe fn execute_script(&self, script: LPCWSTR, handler: /* TODO */ *mut c_void) -> HRESULT;
    unsafe fn capture_preview(&self);
    unsafe fn reload(&self);
    unsafe fn post_web_message_as_json(&self, web_message_as_json: LPCWSTR) -> HRESULT;
    unsafe fn post_web_message_as_string(&self, web_message_as_string: LPCWSTR) -> HRESULT;
    unsafe fn add_web_message_received(
        &self,
        handler: ComPtr<dyn ICoreWebView2WebMessageReceivedEventHandler>,
        token: *mut c_void,
    ) -> HRESULT;
    // Rest TODO.
}

#[com_interface("D58A964A-13C4-44FB-81AD-64AE242E9ADC")]
pub trait ICoreWebView2Settings: IUnknown {
    unsafe fn get_is_script_enabled(&self);
    unsafe fn put_is_script_enabled(&self);

    unsafe fn get_is_web_message_enabled(&self);
    unsafe fn put_is_web_message_enabled(&self);

    unsafe fn get_are_default_script_dialogs_enabled(&self);
    unsafe fn put_are_default_script_dialogs_enabled(&self);

    unsafe fn get_is_status_bar_enabled(&self);
    unsafe fn put_is_status_bar_enabled(&self, enabled: BOOL) -> HRESULT;

    unsafe fn get_are_dev_tools_enabled(&self);
    unsafe fn put_are_dev_tools_enabled(&self);

    unsafe fn get_are_default_context_menu_enabled(&self);
    unsafe fn put_are_default_context_menu_enabled(&self, enabled: BOOL) -> HRESULT;

    unsafe fn get_are_remote_objects_allowed(&self);
    unsafe fn put_are_remote_objects_allowed(&self);

    unsafe fn get_is_zoom_control_enabled(&self);
    unsafe fn put_is_zoom_control_enabled(&self, enalbed: BOOL) -> HRESULT;
}

#[com_interface("ABABDC66-DF8D-487D-A737-7B25E8F835AA")]
pub trait ICoreWebView2WebMessageReceivedEventHandler: IUnknown {
    unsafe fn invoke(
        &self,
        sender: ComPtr<dyn ICoreWebView2>,
        args: ComPtr<dyn ICoreWebView2WebMessageReceivedEventArgs>,
    ) -> HRESULT;
}

#[com_interface("B21D70E2-942E-44EB-B843-22C156FDE288")]
pub trait ICoreWebView2WebMessageReceivedEventArgs: IUnknown {
    unsafe fn get_source(&self, source: *mut LPWSTR) -> HRESULT;
    unsafe fn get_web_message_as_json(&self, web_message_as_json: *mut LPWSTR) -> HRESULT;
    unsafe fn get_web_message_as_string(&self, web_message_as_string: *mut LPWSTR) -> HRESULT;
}

#[repr(u32)]
pub enum MoveFocusReason {
    Programmatic = 0,
    Next = 1,
    Previous = 2,
}

pub type FnCreateCoreWebView2EnvironmentWithDetails = unsafe extern "system" fn(
    browserExecutableFolder: PCWSTR,
    userDataFolder: PCWSTR,
    additionalBrowserArguments: PCWSTR,
    environment: ComPtr<dyn ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>,
) -> HRESULT;
