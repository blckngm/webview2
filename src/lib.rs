//! Rust bindings for
//! [WebView2](https://docs.microsoft.com/en-us/microsoft-edge/hosting/webview2).
//!
//! You need to install the new Chromium based Edge browser (try the canary/beta
//! channels if the stable channel does not work).
//!
//! By default, this crate ships a copy of the `WebView2Loader.dll` file for the
//! target platform (the `embed-dll` feature). This file is then extracted
//! alongside the executable file and dynamically loaded at runtime. License of
//! the DLL file (part of the WebView2 SDK) is included in the
//! `Microsoft.Web.WebView2.0.9.430` folder.
//!
//! There are some high level, idiomatic Rust wrappers, but they are very
//! incomplete. The low level bindings in `sys` though, is automatically
//! generated and complete. You can use the `as_raw` methods to convert to raw
//! COM objects and call all those methods. The `callback` macro can be helpful
//! for implementing callbacks as COM objects.

#![cfg(windows)]
// Caused by the `com_interface` macro.
#![allow(clippy::cmp_null)]
#![allow(clippy::type_complexity)]

pub mod sys;

use com::{interfaces::IUnknown, ComInterface, ComPtr, ComRc};
use std::cell::RefCell;
use std::fmt;
use std::io;
use std::mem;
use std::path::Path;
use std::ptr;
use widestring::{NulError, WideCStr, WideCString};
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;
use winapi::shared::windef::*;
use winapi::shared::winerror::{E_FAIL, E_INVALIDARG, HRESULT_FROM_WIN32, SUCCEEDED, S_OK};
use winapi::um::combaseapi::CoTaskMemFree;
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryW};

use sys::*;

#[cfg(all(feature = "embed-dll", target_arch = "x86_64"))]
const DLL: &[u8] =
    include_bytes!("..\\Microsoft.Web.WebView2.0.9.430\\build\\x64\\WebView2Loader.dll");
#[cfg(all(feature = "embed-dll", target_arch = "x86"))]
const DLL: &[u8] =
    include_bytes!("..\\Microsoft.Web.WebView2.0.9.430\\build\\x86\\WebView2Loader.dll");
#[cfg(all(feature = "embed-dll", target_arch = "aarch64"))]
const DLL: &[u8] =
    include_bytes!("..\\Microsoft.Web.WebView2.0.9.430\\build\\arm64\\WebView2Loader.dll");

/// Returns a pointer that implements the COM callback interface with the specified closure.
/// Inspired by C++ Microsoft::WRT::Callback.
#[macro_export]
macro_rules! callback {
    ($name:ident, move | $($arg:ident : $arg_type:ty),* $(,)?| -> $ret_type:ty { $($body:tt)* }) => {{
        #[com::co_class(implements($name))]
        struct Impl {
            cb: Box<dyn Fn($($arg_type),*) -> $ret_type>,
        }

        impl $name for Impl {
            unsafe fn invoke(&self, $($arg : $arg_type),*) -> $ret_type {
                (self.cb)($($arg),*)
            }
        }

        impl Impl {
            // It is never used.
            pub fn new() -> Box<Self> {
                unreachable!()
            }
            // Returns an owning ComPtr. Suitable for passing over FFI.
            // The receiver is responsible for releasing it.
            pub fn new_ptr(cb: impl Fn($($arg_type),*) -> $ret_type + 'static) -> com::ComPtr<dyn $name> {
                let e = Self::allocate(Box::new(cb));
                unsafe {
                    use com::interfaces::IUnknown;
                    e.add_ref();
                    com::ComPtr::<dyn $name>::new(Box::into_raw(e) as _)
                }
            }
        }

        Impl::new_ptr(move |$($arg : $arg_type),*| -> $ret_type { $($body)* })
    }}
}

// Call `AddRef` and convert to `ComRc`.
unsafe fn add_ref_to_rc<T: ComInterface + ?Sized>(
    ptr: *mut *mut <T as ComInterface>::VTable,
) -> ComRc<T> {
    let ptr = ComPtr::new(ptr);
    ptr.add_ref();
    ptr.upgrade()
}

/// `ICoreWebView2Environment`.
#[derive(Clone)]
pub struct Environment {
    inner: ComRc<dyn ICoreWebView2Environment>,
}

/// `ICoreWebView2Host`.
#[derive(Clone)]
pub struct Host {
    inner: ComRc<dyn ICoreWebView2Host>,
}

/// `ICoreWebView2`.
#[derive(Clone)]
#[repr(C)]
pub struct WebView {
    inner: ComRc<dyn ICoreWebView2>,
}

/// `ICoreWebView2Settings`.
#[derive(Clone)]
pub struct Settings {
    inner: ComRc<dyn ICoreWebView2Settings>,
}

/// `ICoreWebView2ContentLoadingEventArgs`.
#[derive(Clone)]
pub struct ContentLoadingEventArgs {
    inner: ComRc<dyn ICoreWebView2ContentLoadingEventArgs>,
}

/// `ICoreWebView2WebMessageReceivedEventArgs`.
#[derive(Clone)]
pub struct WebMessageReceivedEventArgs {
    inner: ComRc<dyn ICoreWebView2WebMessageReceivedEventArgs>,
}

/// `ICoreWebView2WebResourceRequest`.
#[derive(Clone)]
pub struct WebResourceRequest {
    inner: ComRc<dyn ICoreWebView2WebResourceRequest>,
}

/// `ICoreWebView2WebResourceResponse`.
#[derive(Clone)]
pub struct WebResourceResponse {
    inner: ComRc<dyn ICoreWebView2WebResourceResponse>,
}

/// `ICoreWebView2WebResourceRequestedEventArgs`.
#[derive(Clone)]
pub struct WebResourceRequestedEventArgs {
    inner: ComRc<dyn ICoreWebView2WebResourceRequestedEventArgs>,
}

/// `ICoreWebView2NavigationCompletedEventArgs`.
#[derive(Clone)]
pub struct NavigationCompletedEventArgs {
    inner: ComRc<dyn ICoreWebView2NavigationCompletedEventArgs>,
}

/// `ICoreWebView2NavigationStartingEventArgs`.
#[derive(Clone)]
pub struct NavigationStartingEventArgs {
    inner: ComRc<dyn ICoreWebView2NavigationStartingEventArgs>,
}

/// `ICoreWebView2SourceChangedEventArgs`.
#[derive(Clone)]
pub struct SourceChangedEventArgs {
    inner: ComRc<dyn ICoreWebView2SourceChangedEventArgs>,
}

/// A builder for calling the `CreateCoreWebView2EnvironmentWithDetails`
/// function.
#[derive(Default)]
pub struct EnvironmentBuilder<'a> {
    dll_file_path: Option<&'a Path>,
    browser_executable_folder: Option<&'a Path>,
    user_data_folder: Option<&'a Path>,
    additional_browser_arguments: Option<&'a str>,
}

impl<'a> EnvironmentBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_browser_executable_folder(self, browser_executable_folder: &'a Path) -> Self {
        Self {
            browser_executable_folder: Some(browser_executable_folder),
            ..self
        }
    }

    pub fn with_user_data_folder(self, user_data_folder: &'a Path) -> Self {
        Self {
            user_data_folder: Some(user_data_folder),
            ..self
        }
    }

    pub fn with_additional_browser_arguments(self, additional_browser_arguments: &'a str) -> Self {
        Self {
            additional_browser_arguments: Some(additional_browser_arguments),
            ..self
        }
    }

    /// Set path to the `WebView2Loader.dll` file.
    ///
    /// * When the `embed-dll` feature is enabled:
    ///
    ///   If it's a relative path, it will be resolved relative to the
    ///   executable's path. If the file does not exist, the embeded dll file
    ///   will be written there.
    ///
    /// * When the `embed-dll` feature is not enabled:
    ///
    ///   It will be simply passed to `LoadLibraryW`.
    ///
    /// Default value: `WebView2Loader.dll`.
    pub fn with_dll_file_path(self, dll_file_path: &'a Path) -> Self {
        Self {
            dll_file_path: Some(dll_file_path),
            ..self
        }
    }

    pub fn build(
        self,
        completed: impl FnOnce(Result<Environment>) -> Result<()> + 'static,
    ) -> Result<()> {
        let Self {
            dll_file_path,
            browser_executable_folder,
            user_data_folder,
            additional_browser_arguments,
        } = self;

        #[cfg(feature = "embed-dll")]
        let dll_file_path = {
            let dll_file_path = dll_file_path.unwrap_or_else(|| Path::new("WebView2Loader.dll"));
            let exe_path = std::env::current_exe()?;
            let exe_dir = exe_path.parent().unwrap();
            let dll_file_path = exe_dir.join(dll_file_path);
            if !dll_file_path.exists() {
                std::fs::write(&dll_file_path, DLL)?;
            }
            dll_file_path
        };
        #[cfg(not(feature = "embed-dll"))]
        let dll_file_path = dll_file_path.unwrap_or_else(|| Path::new("WebView2Loader.dll"));

        let create_fn: FnCreateCoreWebView2EnvironmentWithDetails = unsafe {
            let dll_file_path = WideCString::from_os_str(dll_file_path)?;
            let dll = LoadLibraryW(dll_file_path.as_ptr());
            if dll.is_null() {
                return Err(io::Error::last_os_error().into());
            }
            let create_fn = GetProcAddress(
                dll,
                "CreateCoreWebView2EnvironmentWithDetails\0".as_ptr() as *const i8,
            );
            if create_fn.is_null() {
                return Err(io::Error::last_os_error().into());
            }
            mem::transmute(create_fn)
        };

        let browser_executable_folder = if let Some(p) = browser_executable_folder {
            Some(WideCString::from_os_str(p)?)
        } else {
            None
        };
        let user_data_folder = if let Some(p) = user_data_folder {
            Some(WideCString::from_os_str(p)?)
        } else {
            None
        };
        let additional_browser_arguments = if let Some(a) = additional_browser_arguments {
            Some(WideCString::from_str(a)?)
        } else {
            None
        };

        let completed = RefCell::new(Some(completed));
        let completed = callback!(
            ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler,
            move |result: HRESULT,
                  created_environment: *mut *mut ICoreWebView2EnvironmentVTable|
                  -> HRESULT {
                let result = check_hresult(result).map(move |_| Environment {
                    inner: unsafe { add_ref_to_rc(created_environment) },
                });
                to_hresult(completed.borrow_mut().take().unwrap()(result))
            }
        );

        check_hresult(unsafe {
            create_fn(
                browser_executable_folder
                    .as_ref()
                    .map(|p| p.as_ptr())
                    .unwrap_or(ptr::null()),
                user_data_folder
                    .as_ref()
                    .map(|p| p.as_ptr())
                    .unwrap_or(ptr::null()),
                additional_browser_arguments
                    .as_ref()
                    .map(|p| p.as_ptr())
                    .unwrap_or(ptr::null()),
                completed.as_raw(),
            )
        })
    }
}

macro_rules! get_bool {
    ($get_method:ident) => {
        pub fn $get_method(&self) -> Result<bool> {
            let mut enabled: BOOL = 0;
            check_hresult(unsafe { self.inner.$get_method(&mut enabled) })?;
            Ok(enabled != 0)
        }
    };
}

macro_rules! put_bool {
    ($put_method:ident) => {
        pub fn $put_method(&self, enabled: bool) -> Result<()> {
            let enabled = if enabled { 1 } else { 0 };
            check_hresult(unsafe { self.inner.$put_method(enabled) })
        }
    };
}

macro_rules! get_string {
    ($get_string_method:ident) => {
        pub fn $get_string_method(&self) -> Result<String> {
            let mut result: LPWSTR = ptr::null_mut();
            check_hresult(unsafe { self.inner.$get_string_method(&mut result) })?;
            let result1 = unsafe { WideCStr::from_ptr_str(result) };
            let result1 = result1.to_string().map_err(|_| Error { hresult: E_FAIL });
            unsafe {
                CoTaskMemFree(result as _);
            }
            result1
        }
    }
}

macro_rules! call {
    ($method:ident) => {
        pub fn $method(&self) -> Result<()> {
            check_hresult(unsafe { self.inner.$method() })
        }
    };
}

macro_rules! add_event_handle_host {
    ($method:ident, $arg_type:ident) => {
        pub fn $method(
            &self,
            event_handler: impl Fn(Host) -> Result<()> + 'static,
        ) -> Result<EventRegistrationToken> {
            let mut token: EventRegistrationToken = unsafe { mem::zeroed() };

            let event_handler = callback!(
                $arg_type,
                move |sender: *mut *mut ICoreWebView2HostVTable,
                    _args: *mut *mut com::interfaces::iunknown::IUnknownVTable|
                    -> HRESULT {
                    let sender = Host {
                        inner: unsafe { add_ref_to_rc(sender) },
                    };
                    to_hresult(event_handler(sender))
                }
            );

            check_hresult(unsafe {
                self.inner.$method(event_handler.as_raw(), &mut token)
            })?;
            Ok(token)
        }
    };
}

macro_rules! add_event_handle_view {
    ($method:ident, $arg_type:ident) => {
        pub fn $method(
            &self,
            event_handler: impl Fn(WebView) -> Result<()> + 'static,
        ) -> Result<EventRegistrationToken> {
            let mut token: EventRegistrationToken = unsafe { mem::zeroed() };

            let event_handler = callback!(
                $arg_type,
                move |sender: *mut *mut ICoreWebView2VTable,
                    _args: *mut *mut com::interfaces::iunknown::IUnknownVTable|
                    -> HRESULT {
                    let sender = WebView {
                        inner: unsafe { add_ref_to_rc(sender) },
                    };
                    to_hresult(event_handler(sender))
                }
            );

            check_hresult(unsafe {
                self.inner.$method(event_handler.as_raw(), &mut token)
            })?;
            Ok(token)
        }
    };
}


macro_rules! add_event_handle {
    ($method:ident, $arg_type:ident, $arg_args:ident, $arg_args_type:ident) => {
        pub fn $method(
            &self,
            handler: impl Fn(WebView, $arg_args) -> Result<()> + 'static,
        ) -> Result<EventRegistrationToken> {
            let mut token: EventRegistrationToken = unsafe { mem::zeroed() };

            let handler = callback!(
                $arg_type,
                move |sender: *mut *mut ICoreWebView2VTable,
                    args: *mut *mut $arg_args_type|
                    -> HRESULT {
                    let sender = WebView {
                        inner: unsafe { add_ref_to_rc(sender) },
                    };
                    let args = $arg_args {
                        inner: unsafe { add_ref_to_rc(args) },
                    };
                    to_hresult(handler(sender, args))
                }
            );

            check_hresult(unsafe {
                self.inner.$method(handler.as_raw(), &mut token)
            })?;
            Ok(token)
        }
    };
}

macro_rules! remove_event_handle {
    ($method:ident) => {
        pub fn $method(&self, token: EventRegistrationToken) -> Result<()> {
            check_hresult(unsafe { self.inner.$method(token) })
        }
    };
}

impl Environment {
    pub fn create_host(
        &self,
        parent_window: HWND,
        completed: impl FnOnce(Result<Host>) -> Result<()> + 'static,
    ) -> Result<()> {
        let completed = RefCell::new(Some(completed));
        let completed = callback!(
            ICoreWebView2CreateCoreWebView2HostCompletedHandler,
            move |result: HRESULT, created_host: *mut *mut ICoreWebView2HostVTable| -> HRESULT {
                let result = check_hresult(result).map(|_| Host {
                    inner: unsafe { add_ref_to_rc(created_host) },
                });
                to_hresult(completed.borrow_mut().take().unwrap()(result))
            }
        );
        check_hresult(unsafe {
            self.inner
                .create_core_web_view2_host(parent_window, completed.as_raw())
        })
    }

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2Environment> {
        &self.inner
    }
}

impl Host {
    get_bool!(get_is_visible);
    put_bool!(put_is_visible);
    // TODO: get_bounds
    pub fn put_bounds(&self, bounds: RECT) -> Result<()> {
        check_hresult(unsafe { self.inner.put_bounds(bounds) })
    }
    // TODO: get_zoom_factor
    // TODO: put_zoom_factor
    // TODO: add_zoom_factor_changed //eventHandler
    remove_event_handle!(remove_zoom_factor_changed);
    // TODO: set_bounds_and_zoom_factor
    pub fn move_focus(&self, reason: MoveFocusReason) -> Result<()> {
        check_hresult(unsafe { self.inner.move_focus(reason) })
    }
    // TODO: add_move_focus_requested //eventHandler
    remove_event_handle!(remove_move_focus_requested);
    add_event_handle_host!(
        add_got_focus,
        ICoreWebView2FocusChangedEventHandler
    );
    remove_event_handle!(remove_got_focus);
    add_event_handle_host!(
        add_lost_focus,
        ICoreWebView2FocusChangedEventHandler
    );
    remove_event_handle!(remove_lost_focus);
    // TODO: add_accelerator_key_pressed //eventHandler
    remove_event_handle!(remove_accelerator_key_pressed);
    // TODO: get_parent_window
    // TODO: put_parent_window
    pub fn notify_parent_window_position_changed(&self) -> Result<()> {
        check_hresult(unsafe { self.inner.notify_parent_window_position_changed() })
    }
    call!(close);
    pub fn get_webview(&self) -> Result<WebView> {
        let mut ppv: *mut *mut ICoreWebView2VTable = ptr::null_mut();
        check_hresult(unsafe { self.inner.get_core_web_view2(&mut ppv) })?;
        Ok(WebView {
            inner: unsafe { add_ref_to_rc(ppv) },
        })
    }

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2Host> {
        &self.inner
    }
}

impl WebView {
    pub fn get_settings(&self) -> Result<Settings> {
        let mut ppv: *mut *mut ICoreWebView2SettingsVTable = ptr::null_mut();
        check_hresult(unsafe { self.inner.get_settings(&mut ppv) })?;
        Ok(Settings {
            inner: unsafe { add_ref_to_rc(ppv) },
        })
    }
    get_string!(get_source);
    pub fn navigate(&self, uri: &str) -> Result<()> {
        let uri = WideCString::from_str(uri)?;
        check_hresult(unsafe { self.inner.navigate(uri.as_ptr()) })
    }
    pub fn navigate_to_string(&self, html_content: &str) -> Result<()> {
        let html_content = WideCString::from_str(html_content)?;
        check_hresult(unsafe { self.inner.navigate_to_string(html_content.as_ptr()) })
    }
    add_event_handle!(
        add_navigation_starting,
        ICoreWebView2NavigationStartingEventHandler,
        NavigationStartingEventArgs,
        ICoreWebView2NavigationStartingEventArgsVTable
    );
    remove_event_handle!(remove_navigation_starting);
    add_event_handle!(
        add_content_loading,
        ICoreWebView2ContentLoadingEventHandler,
        ContentLoadingEventArgs,
        ICoreWebView2ContentLoadingEventArgsVTable
    );
    remove_event_handle!(remove_content_loading);
    add_event_handle!(
        add_source_changed,
        ICoreWebView2SourceChangedEventHandler,
        SourceChangedEventArgs,
        ICoreWebView2SourceChangedEventArgsVTable
    );
    remove_event_handle!(remove_source_changed);
    add_event_handle_view!(
        add_history_changed,
        ICoreWebView2HistoryChangedEventHandler
    );
    remove_event_handle!(remove_history_changed);
    add_event_handle!(
        add_navigation_completed,
        ICoreWebView2NavigationCompletedEventHandler,
        NavigationCompletedEventArgs,
        ICoreWebView2NavigationCompletedEventArgsVTable
    );
    remove_event_handle!(remove_navigation_completed);
    // TODO: add_frame_navigation_starting //eventHandler
    remove_event_handle!(remove_frame_navigation_starting);
    // TODO: add_script_dialog_opening //eventHandler
    remove_event_handle!(remove_script_dialog_opening);
    // TODO: add_permission_requested //eventHandler
    remove_event_handle!(remove_permission_requested);
    // TODO: add_process_failed //eventHandler
    remove_event_handle!(remove_process_failed);
    // Don't take an `Option<impl FnOnce>`:
    // https://users.rust-lang.org/t/solved-how-to-pass-none-to-a-function-when-an-option-closure-is-expected/10956/8
    pub fn add_script_to_execute_on_document_created(
        &self,
        script: &str,
        callback: impl FnOnce(String) -> Result<()> + 'static,
    ) -> Result<()> {
        let script = WideCString::from_str(script)?;
        let callback = RefCell::new(Some(callback));
        let callback = callback!(
            ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
            move |error_code: HRESULT, id: LPCWSTR| -> HRESULT {
                to_hresult(check_hresult(error_code).and_then(|_| {
                    let id = unsafe { WideCStr::from_ptr_str(id) }
                        .to_string()
                        .map_err(|_| Error::new(E_FAIL))?;
                    if let Some(callback) = callback.borrow_mut().take() {
                        callback(id)
                    } else {
                        Ok(())
                    }
                }))
            }
        );
        check_hresult(unsafe {
            self.inner
                .add_script_to_execute_on_document_created(script.as_ptr(), callback.as_raw())
        })
    }
    pub fn remove_script_to_execute_on_document_created(&self, id: &str) -> Result<()> {
        let id = WideCString::from_str(id)?;
        check_hresult(unsafe {
            self.inner
                .remove_script_to_execute_on_document_created(id.as_ptr())
        })
    }
    pub fn execute_script(
        &self,
        script: &str,
        callback: impl FnOnce(String) -> Result<()> + 'static,
    ) -> Result<()> {
        let script = WideCString::from_str(script)?;
        let callback = RefCell::new(Some(callback));
        let callback = callback!(
            ICoreWebView2ExecuteScriptCompletedHandler,
            move |error_code: HRESULT, result_object_as_json: LPCWSTR| -> HRESULT {
                to_hresult(check_hresult(error_code).and_then(|_| {
                    let result_object_as_json_string =
                        unsafe { WideCStr::from_ptr_str(result_object_as_json) }
                            .to_string()
                            .map_err(|_| Error::new(E_FAIL))?;
                    if let Some(callback) = callback.borrow_mut().take() {
                        callback(result_object_as_json_string)
                    } else {
                        Ok(())
                    }
                }))
            }
        );
        check_hresult(unsafe {
            self.inner
                .execute_script(script.as_ptr(), callback.as_raw())
        })
    }
    add_event_handle_view!(
        add_document_title_changed,
        ICoreWebView2DocumentTitleChangedEventHandler
    );
    remove_event_handle!(remove_document_title_changed);
    // TODO: capture_preview
    call!(reload);
    pub fn post_web_message_as_json(&self, web_message_as_json: &str) -> Result<()> {
        let message = WideCString::from_str(web_message_as_json)?;
        check_hresult(unsafe { self.inner.post_web_message_as_json(message.as_ptr()) })
    }
    pub fn post_web_message_as_string(&self, web_message_as_string: &str) -> Result<()> {
        let message = WideCString::from_str(web_message_as_string)?;
        check_hresult(unsafe { self.inner.post_web_message_as_string(message.as_ptr()) })
    }
    add_event_handle!(
        add_web_message_received,
        ICoreWebView2WebMessageReceivedEventHandler,
        WebMessageReceivedEventArgs,
        ICoreWebView2WebMessageReceivedEventArgsVTable
    );
    remove_event_handle!(remove_web_message_received);
    // TODO: call_dev_tools_protocol_method
    // TODO: get_browser_process_id
    get_bool!(get_can_go_back);
    get_bool!(get_can_go_forward);
    call!(go_back);
    call!(go_forward);
    // TODO: get_dev_tools_protocol_event_receiver
    call!(stop);
    // TODO: add_new_window_requested //eventHandler
    remove_event_handle!(remove_new_window_requested);
    get_string!(get_document_title);
    // TODO: add_remote_object ??
    // TODO: remove_remote_object ??
    call!(open_dev_tools_window);
    add_event_handle_view!(
        add_contains_full_screen_element_changed,
        ICoreWebView2ContainsFullScreenElementChangedEventHandler
    );
    remove_event_handle!(remove_contains_full_screen_element_changed);
    get_bool!(get_contains_full_screen_element);
    add_event_handle!(
        add_web_resource_requested,
        ICoreWebView2WebResourceRequestedEventHandler,
        WebResourceRequestedEventArgs,
        ICoreWebView2WebResourceRequestedEventArgsVTable
    );
    remove_event_handle!(remove_web_resource_requested);
    // TODO: add_web_resource_requested_filter
    // TODO: remove_web_resource_requested_filter
    add_event_handle_view!(
        add_window_close_requested,
        ICoreWebView2WindowCloseRequestedEventHandler
    );
    remove_event_handle!(remove_window_close_requested);

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2> {
        &self.inner
    }
}

impl Settings {
    get_bool!(get_is_script_enabled);
    put_bool!(put_is_script_enabled);

    get_bool!(get_is_web_message_enabled);
    put_bool!(put_is_web_message_enabled);

    get_bool!(get_are_default_script_dialogs_enabled);
    put_bool!(put_are_default_script_dialogs_enabled);

    get_bool!(get_is_status_bar_enabled);
    put_bool!(put_is_status_bar_enabled);

    get_bool!(get_are_dev_tools_enabled);
    put_bool!(put_are_dev_tools_enabled);

    get_bool!(get_are_default_context_menus_enabled);
    put_bool!(put_are_default_context_menus_enabled);

    get_bool!(get_are_remote_objects_allowed);
    put_bool!(put_are_remote_objects_allowed);

    get_bool!(get_is_zoom_control_enabled);
    put_bool!(put_is_zoom_control_enabled);

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2Settings> {
        &self.inner
    }
}

impl ContentLoadingEventArgs {
    get_bool!(get_is_error_page);
    // TODO: get_navigation_id //UINT64

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2ContentLoadingEventArgs> {
        &self.inner
    }
}

impl WebMessageReceivedEventArgs {
    get_string!(get_source);
    get_string!(try_get_web_message_as_string);
    get_string!(get_web_message_as_json);

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2WebMessageReceivedEventArgs> {
        &self.inner
    }
}

impl WebResourceRequest {
    // TODO: get_uri //LPWSTR
    // TODO: put_uri //LPCWSTR
    // TODO: get_method //LPWSTR
    // TODO: put_method //LPCWSTR
    // TODO: get_content //IStreamVTable
    // TODO: put_content //IStreamVTable
    // TODO: get_headers //ICoreWebView2HttpRequestHeadersVTable

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2WebResourceRequest> {
        &self.inner
    }
}

impl WebResourceResponse {
    // TODO: get_content //IStreamVTable
    // TODO: put_content //IStreamVTable
    // TODO: get_headers //ICoreWebView2HttpResponseHeadersVTable
    // TODO: get_status_code //i32
    // TODO: put_status_code //i32
    // TODO: get_reason_phrase //LPWSTR
    // TODO: put_reason_phrase //LPCWSTR

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2WebResourceResponse> {
        &self.inner
    }
}

impl WebResourceRequestedEventArgs {
    // TODO: get_request //ICoreWebView2WebResourceRequestVTable
    // TODO: get_response //ICoreWebView2WebResourceResponseVTable
    // TODO: put_response //ICoreWebView2WebResourceResponseVTable
    // TODO: get_deferral //ICoreWebView2DeferralVTable
    // TODO: get_resource_context //CORE_WEBVIEW2_WEB_RESOURCE_CONTEXT

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2WebResourceRequestedEventArgs> {
        &self.inner
    }
}

impl NavigationCompletedEventArgs {
    get_bool!(get_is_success);
    // TODO: get_web_error_status //CORE_WEBVIEW2_WEB_ERROR_STATUS
    // TODO: get_navigation_id //UINT64

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2NavigationCompletedEventArgs> {
        &self.inner
    }
}

impl NavigationStartingEventArgs {
    // TODO: get_uri //LPWSTR
    get_bool!(get_is_user_initiated);
    get_bool!(get_is_redirected);
    // TODO: get_request_headers //ICoreWebView2HttpRequestHeadersVTable
    get_bool!(get_cancel);
    put_bool!(put_cancel);
    // TODO: get_navigation_id //UINT64

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2NavigationStartingEventArgs> {
        &self.inner
    }
}

impl SourceChangedEventArgs {
    get_bool!(get_is_new_document);

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2SourceChangedEventArgs> {
        &self.inner
    }
}

#[doc(inline)]
pub type MoveFocusReason = sys::CORE_WEBVIEW2_MOVE_FOCUS_REASON;

#[doc(inline)]
pub use sys::EventRegistrationToken;

/// A webview2 error. Actually, an `HRESULT`.
#[derive(Debug, Eq, PartialEq)]
pub struct Error {
    hresult: HRESULT,
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "webview2 error, HRESULT {:#X}", self.hresult as u32)
    }
}

impl std::error::Error for Error {}

impl From<NulError<u16>> for Error {
    fn from(_: NulError<u16>) -> Error {
        Error {
            hresult: E_INVALIDARG,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        match e.raw_os_error() {
            Some(e) => Error::new(HRESULT_FROM_WIN32(e as u32)),
            _ => Error::new(E_FAIL),
        }
    }
}

impl Error {
    pub fn new(hresult: HRESULT) -> Self {
        Self { hresult }
    }

    pub fn hresult(&self) -> HRESULT {
        self.hresult
    }
}

/// Check a `HRESULT`, if it is `SUCCEEDED`, return `Ok(())`. Otherwide return
/// an error containing the `HRESULT`.
pub fn check_hresult(hresult: HRESULT) -> Result<()> {
    if SUCCEEDED(hresult) {
        Ok(())
    } else {
        Err(Error { hresult })
    }
}

fn to_hresult<T>(r: Result<T>) -> HRESULT {
    match r {
        Ok(_) => S_OK,
        Err(Error { hresult }) => hresult,
    }
}
