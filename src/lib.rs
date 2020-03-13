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

/// `ICoreWebView2WebMessageReceivedEventArgs`.
#[derive(Clone)]
pub struct WebMessageReceivedEventArgs {
    inner: ComRc<dyn ICoreWebView2WebMessageReceivedEventArgs>,
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
    pub fn put_is_visible(&self, is_visible: bool) -> Result<()> {
        let is_visible: BOOL = if is_visible { 1 } else { 0 };
        check_hresult(unsafe { self.inner.put_is_visible(is_visible) })
    }
    pub fn put_bounds(&self, bounds: RECT) -> Result<()> {
        check_hresult(unsafe { self.inner.put_bounds(bounds) })
    }
    pub fn move_focus(&self, reason: MoveFocusReason) -> Result<()> {
        check_hresult(unsafe { self.inner.move_focus(reason) })
    }
    pub fn notify_parent_window_position_changed(&self) -> Result<()> {
        check_hresult(unsafe { self.inner.notify_parent_window_position_changed() })
    }
    pub fn close(&self) -> Result<()> {
        check_hresult(unsafe { self.inner.close() })
    }
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
    pub fn navigate(&self, uri: &str) -> Result<()> {
        let uri = WideCString::from_str(uri)?;
        check_hresult(unsafe { self.inner.navigate(uri.as_ptr()) })
    }
    pub fn navigate_to_string(&self, html_content: &str) -> Result<()> {
        let html_content = WideCString::from_str(html_content)?;
        check_hresult(unsafe { self.inner.navigate_to_string(html_content.as_ptr()) })
    }
    // TODO: callback.
    pub fn execute_script(&self, script: &str) -> Result<()> {
        let script = WideCString::from_str(script)?;
        check_hresult(unsafe { self.inner.execute_script(script.as_ptr(), ptr::null_mut()) })
    }
    pub fn post_web_message_as_json(&self, web_message_as_json: &str) -> Result<()> {
        let message = WideCString::from_str(web_message_as_json)?;
        check_hresult(unsafe { self.inner.post_web_message_as_json(message.as_ptr()) })
    }
    pub fn post_web_message_as_string(&self, web_message_as_string: &str) -> Result<()> {
        let message = WideCString::from_str(web_message_as_string)?;
        check_hresult(unsafe { self.inner.post_web_message_as_string(message.as_ptr()) })
    }
    // TODO: token and remove.
    pub fn add_web_message_received(
        &self,
        handler: impl Fn(WebView, WebMessageReceivedEventArgs) -> Result<()> + 'static,
    ) -> Result<()> {
        let handler = callback!(
            ICoreWebView2WebMessageReceivedEventHandler,
            move |sender: *mut *mut ICoreWebView2VTable,
                  args: *mut *mut ICoreWebView2WebMessageReceivedEventArgsVTable|
                  -> HRESULT {
                let sender = WebView {
                    inner: unsafe { add_ref_to_rc(sender) },
                };
                let args = WebMessageReceivedEventArgs {
                    inner: unsafe { add_ref_to_rc(args) },
                };
                to_hresult(handler(sender, args))
            }
        );

        check_hresult(unsafe {
            self.inner
                .add_web_message_received(handler.as_raw(), ptr::null_mut())
        })
    }

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2> {
        &self.inner
    }
}

impl Settings {
    pub fn put_is_status_bar_enabled(&self, enabled: bool) -> Result<()> {
        let enabled = if enabled { 1 } else { 0 };
        check_hresult(unsafe { self.inner.put_is_status_bar_enabled(enabled) })
    }

    pub fn put_are_default_context_menus_enabled(&self, enabled: bool) -> Result<()> {
        let enabled = if enabled { 1 } else { 0 };
        check_hresult(unsafe { self.inner.put_are_default_context_menus_enabled(enabled) })
    }

    pub fn put_is_zoom_control_enabled(&self, enabled: bool) -> Result<()> {
        let enabled = if enabled { 1 } else { 0 };
        check_hresult(unsafe { self.inner.put_is_zoom_control_enabled(enabled) })
    }

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2Settings> {
        &self.inner
    }
}

impl WebMessageReceivedEventArgs {
    pub fn get_web_message_as_string(&self) -> Result<String> {
        let mut message: LPWSTR = ptr::null_mut();
        check_hresult(unsafe { self.inner.try_get_web_message_as_string(&mut message) })?;
        let message1 = unsafe { WideCStr::from_ptr_str(message) };
        let message1 = message1.to_string().map_err(|_| Error { hresult: E_FAIL });
        unsafe {
            CoTaskMemFree(message as _);
        }
        message1
    }

    pub fn as_raw(&self) -> &ComRc<dyn ICoreWebView2WebMessageReceivedEventArgs> {
        &self.inner
    }
}

#[doc(inline)]
pub type MoveFocusReason = sys::CORE_WEBVIEW2_MOVE_FOCUS_REASON;

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
