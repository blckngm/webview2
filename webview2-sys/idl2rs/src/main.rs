use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::borrow::Cow;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[grammar = "idl.pest"]
struct IDLParser;

#[derive(Debug)]
enum Modifier {
    Pointer,
    Const,
}

#[derive(Debug, Default)]
struct Type<'a> {
    base_type: Cow<'a, str>,
    modifiers: Vec<Modifier>,
}

impl<'a> Type<'a> {
    fn from_pest(pair: Pair<'a, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::_type);

        let mut result = Self::default();

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::identifier => {
                    result.base_type = if p.as_str().eq_ignore_ascii_case("uint32") {
                        "u32".into()
                    } else if p.as_str().eq_ignore_ascii_case("uint64") {
                        "u64".into()
                    } else if p.as_str().eq_ignore_ascii_case("int32") {
                        "i32".into()
                    } else if p.as_str().eq_ignore_ascii_case("int64") {
                        "i64".into()
                    } else if p.as_str().eq_ignore_ascii_case("uint") {
                        "u32".into()
                    } else if p.as_str().eq_ignore_ascii_case("int") {
                        "i32".into()
                    } else if p.as_str().eq_ignore_ascii_case("double") {
                        "f64".into()
                    } else if p.as_str().starts_with("I") {
                        result.modifiers.push(Modifier::Pointer);
                        format!("{}VTable", p.as_str()).into()
                    } else if p.as_str().starts_with("COREWEBVIEW2_") {
                        remove_prefix_to_pascal("COREWEBVIEW2_", p.as_str()).into()
                    } else {
                        p.as_str().into()
                    }
                }
                Rule::pointer => result.modifiers.push(Modifier::Pointer),
                Rule::_const => result.modifiers.push(Modifier::Const),
                _ => {}
            }
        }
        result.modifiers.reverse();
        result
    }

    pub fn render(&self, w: &mut impl Write) -> io::Result<()> {
        for m in &self.modifiers {
            if matches!(m, Modifier::Pointer) {
                write!(w, "*mut ")?;
            }
        }

        write!(w, "{}", self.base_type)
    }
}

#[derive(Debug, Default)]
struct Parameter<'a> {
    attributes: Vec<&'a str>,
    r#type: Type<'a>,
    name: &'a str,
}

impl<'a> Parameter<'a> {
    fn from_pest(pair: Pair<'a, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::parameter);

        let mut result = Self::default();

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::parameter_attribute => result.attributes.push(p.as_str()),
                Rule::_type => result.r#type = Type::from_pest(p),
                Rule::identifier => result.name = p.as_str(),
                _ => {}
            }
        }
        result
    }

    pub fn render(&self, w: &mut impl Write) -> io::Result<()> {
        if !self.attributes.is_empty() {
            write!(w, "/* {} */ ", self.attributes.join(", "))?;
        };
        write!(w, "{}: ", camel_to_snake(self.name))?;
        self.r#type.render(w)
    }
}

#[derive(Debug, Default)]
struct Method<'a> {
    doc_comment: Option<&'a str>,
    attribute: Option<&'a str>,
    return_type: Type<'a>,
    name: &'a str,
    parameters: Vec<Parameter<'a>>,
}

impl<'a> Method<'a> {
    fn from_pest(pair: Pair<'a, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::method);

        let mut result = Self::default();

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::doc_comment => result.doc_comment = Some(p.as_str().trim_end_matches(" \t")),
                Rule::method_attribute => result.attribute = Some(p.as_str()),
                Rule::_type => result.return_type = Type::from_pest(p),
                Rule::method_name => result.name = p.as_str(),
                Rule::parameter => result.parameters.push(Parameter::from_pest(p)),
                _ => {}
            }
        }
        result
    }

    pub fn render(&self, w: &mut impl Write) -> io::Result<()> {
        write!(w, "{}", self.doc_comment.unwrap_or(""))?;
        let name_prefix = if self.attribute == Some("[propget]") {
            "get_"
        } else if self.attribute == Some("[propput]") {
            "put_"
        } else {
            ""
        };
        write!(
            w,
            "    unsafe fn {}{}(&self",
            name_prefix,
            camel_to_snake(self.name)
        )?;
        for p in &self.parameters {
            write!(w, ", ")?;
            p.render(w)?;
        }
        write!(w, ") -> ")?;
        self.return_type.render(w)?;
        writeln!(w, ";")
    }
}

#[derive(Debug, Default)]
struct TypedefEnum<'a> {
    doc_comment: Option<&'a str>,
    name: &'a str,
    variants: Vec<Variant<'a>>,
}

#[derive(Debug, Default)]
struct Variant<'a> {
    doc_comment: Option<&'a str>,
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> TypedefEnum<'a> {
    fn from_pest(pair: Pair<'a, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::typedef_enum);

        let mut result = Self::default();

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::doc_comment => result.doc_comment = Some(p.as_str().trim_end_matches(" \t")),
                Rule::identifier => result.name = p.as_str(),
                Rule::variant => {
                    let variant = {
                        let mut result = Variant::default();
                        for p in p.into_inner() {
                            match p.as_rule() {
                                Rule::doc_comment => {
                                    result.doc_comment = Some(p.as_str().trim_end_matches(" \t"))
                                }
                                Rule::identifier => result.name = p.as_str(),
                                Rule::variant_value => result.value = Some(p.as_str()),
                                _ => {}
                            }
                        }
                        result
                    };
                    result.variants.push(variant);
                }
                _ => {}
            }
        }
        result
    }

    pub fn render(&self, w: &mut impl Write) -> io::Result<()> {
        write!(w, "{}", self.doc_comment.unwrap_or(""))?;
        writeln!(w, "#[repr(u32)]")?;
        writeln!(w, "#[derive(Debug, Copy, Clone, Eq, PartialEq)]")?;
        writeln!(
            w,
            "pub enum {} {{",
            remove_prefix_to_pascal("COREWEBVIEW2_", self.name)
        )?;
        for variant in &self.variants {
            write!(w, "{}", variant.doc_comment.unwrap_or(""))?;
            if let Some(value) = variant.value {
                writeln!(
                    w,
                    "    {} = {},",
                    remove_prefix_to_pascal(self.name, variant.name),
                    value
                )?;
            } else {
                writeln!(
                    w,
                    "    {},",
                    remove_prefix_to_pascal(self.name, variant.name)
                )?;
            }
        }
        writeln!(w, "}}")
    }
}

#[derive(Debug, Default)]
struct Field<'a> {
    doc_comment: Option<&'a str>,
    name: &'a str,
    r#type: Type<'a>,
}

impl<'a> Field<'a> {
    fn from_pest(pair: Pair<'a, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::field);

        let mut result = Self::default();

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::doc_comment => result.doc_comment = Some(p.as_str().trim_end_matches(" \t")),
                Rule::_type => result.r#type = Type::from_pest(p),
                Rule::identifier => result.name = p.as_str(),
                _ => {}
            }
        }
        result
    }
}

#[derive(Debug, Default)]
struct TypedefStruct<'a> {
    doc_comment: Option<&'a str>,
    name: &'a str,
    fields: Vec<Field<'a>>,
}

impl<'a> TypedefStruct<'a> {
    fn from_pest(pair: Pair<'a, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::typedef_struct);

        let mut result = Self::default();

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::doc_comment => result.doc_comment = Some(p.as_str().trim_end_matches(" \t")),
                Rule::identifier => result.name = p.as_str(),
                Rule::field => result.fields.push(Field::from_pest(p)),
                _ => {}
            }
        }
        result
    }

    fn render(&self, w: &mut impl Write) -> io::Result<()> {
        write!(w, "{}", self.doc_comment.unwrap_or(""))?;
        writeln!(w, "#[repr(C)]")?;
        writeln!(w, "#[derive(Debug, Copy, Clone, Eq, PartialEq)]")?;
        writeln!(
            w,
            "pub struct {} {{",
            remove_prefix_to_pascal("COREWEBVIEW2_", self.name)
        )?;
        for field in &self.fields {
            write!(w, "{}", field.doc_comment.unwrap_or(""))?;
            write!(w, "    pub {}: ", camel_to_snake(field.name))?;
            field.r#type.render(w)?;
            writeln!(w, ",")?;
        }
        writeln!(w, "}}")
    }
}

#[derive(Debug, Default)]
struct Interface<'a> {
    doc_comment: Option<&'a str>,
    name: &'a str,
    parent: &'a str,
    uuid: Option<&'a str>,
    attributes: Vec<&'a str>,
    methods: Vec<Method<'a>>,
}

impl<'a> Interface<'a> {
    fn from_pest(pair: Pair<'a, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::interface);

        let mut result = Self::default();

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::doc_comment => result.doc_comment = Some(p.as_str().trim_end_matches(" \t")),
                Rule::uuid => result.uuid = Some(p.as_str()),
                Rule::other_attribute => result.attributes.push(p.as_str()),
                Rule::interface_name => result.name = p.as_str(),
                Rule::parent => result.parent = p.as_str(),
                Rule::method => result.methods.push(Method::from_pest(p)),
                _ => {}
            }
        }
        result
    }

    pub fn render(&self, w: &mut impl Write) -> io::Result<()> {
        write!(w, "{}", self.doc_comment.unwrap_or(""))?;
        if let Some(uuid) = self.uuid {
            writeln!(w, "#[com_interface(\"{}\")]", uuid)?;
        }
        writeln!(w, "pub trait {}: {} {{", self.name, self.parent)?;
        let mut first = true;
        for m in &self.methods {
            if first {
                first = false;
            } else {
                writeln!(w)?;
            }
            m.render(w)?;
        }
        writeln!(w, "}}")?;

        Ok(())
    }
}

#[derive(Debug, Default)]
struct Document<'a> {
    interfaces: Vec<Interface<'a>>,
    structs: Vec<TypedefStruct<'a>>,
    enums: Vec<TypedefEnum<'a>>,
}

impl<'a> Document<'a> {
    fn from_pest(pair: Pair<'a, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::document);

        let mut result = Self::default();

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::interface => result.interfaces.push(Interface::from_pest(p)),
                Rule::typedef_enum => result.enums.push(TypedefEnum::from_pest(p)),
                Rule::typedef_struct => result.structs.push(TypedefStruct::from_pest(p)),
                _ => {}
            }
        }
        result
    }

    pub fn render(&self, w: &mut impl Write) -> io::Result<()> {
        let mut first = true;
        for s in &self.structs {
            if !first {
                writeln!(w)?;
            } else {
                first = false;
            }
            s.render(w)?;
        }
        for e in &self.enums {
            if !first {
                writeln!(w)?;
            } else {
                first = false;
            }
            e.render(w)?;
        }
        for i in &self.interfaces {
            if !first {
                writeln!(w)?;
            } else {
                first = false;
            }
            i.render(w)?;
        }
        Ok(())
    }
}

fn camel_to_snake(input: &str) -> String {
    let mut new = String::new();
    let mut seen_lowercase = false;

    for c in input.chars() {
        if c.is_uppercase() {
            if seen_lowercase {
                seen_lowercase = false;
                new.push_str("_");
            }
            new.push_str(&c.to_lowercase().to_string());
        } else if c == '_' {
            seen_lowercase = false;
            new.push(c);
        } else {
            seen_lowercase = true;
            new.push_str(&c.to_string())
        }
    }

    new
}

fn remove_prefix_to_pascal(prefix: &str, input: &str) -> String {
    if input.starts_with(prefix) {
        screaming_snake_to_pascal(&input[prefix.len()..])
    } else {
        screaming_snake_to_pascal(input)
    }
}

// HELLO_WORLD -> HelloWorld
fn screaming_snake_to_pascal(input: &str) -> String {
    if input == "_PNG" || input == "_JPEG" {
        return input[1..].into();
    }

    let mut new = String::new();
    let mut last_is_underscore = true;

    for c in input.chars() {
        if c == '_' {
            last_is_underscore = true;
        } else {
            if last_is_underscore {
                last_is_underscore = false;
                new.push(c.to_ascii_uppercase());
            } else {
                new.push(c.to_ascii_lowercase());
            }
        }
    }

    new
}

fn remove_prefix(prefix: &str, input: &str) -> String {
    if input.starts_with(prefix) {
        input[prefix.len()..].into()
    } else {
        input.into()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut p = IDLParser::parse(Rule::document, &input).unwrap_or_else(|e| {
        eprintln!("Parsing error: {}", e);
        std::process::exit(1)
    });
    let doc = Document::from_pest(p.next().unwrap());

    let mut args = std::env::args();
    args.next();
    if args.next().as_deref() == Some("interface_wrappers") {
        println!("// Generated by idl2rs.");
        println!();

        for i in doc.interfaces.into_iter().chain(std::iter::once(Interface {
            name: "IStream",
            ..Default::default()
        })) {
            if i.name.ends_with("Handler") {
                continue;
            }

            let wrapper_name = remove_prefix("ICoreWebView2", i.name);
            let wrapper_name = if wrapper_name.is_empty() {
                "WebView".into()
            } else if wrapper_name == "_2" {
                "WebView_2".into()
            } else if wrapper_name == "_3" {
                "WebView_3".into()
            } else if wrapper_name  == "_4" {
                "WebView_4".into()
            } else if wrapper_name == "_5" {
                "WebView_5".into()
            } else if wrapper_name == "IStream" {
                "Stream".into()
            } else {
                wrapper_name
            };

            println!("/// Wrapper for `{}`.", i.name);
            println!("#[derive(Clone)]");
            println!("pub struct {} {{", wrapper_name);
            println!("    inner: ComRc<dyn {}>,", i.name);
            println!("}}");
            println!("impl From<ComRc<dyn {}>> for {} {{", i.name, wrapper_name);
            println!("    fn from(inner: ComRc<dyn {}>) -> Self {{", i.name);
            println!("        Self {{ inner }}");
            println!("    }}");
            println!("}}");
            println!("impl fmt::Debug for {} {{", wrapper_name);
            println!("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{");
            println!("        f.debug_struct(\"{}\").finish()", wrapper_name);
            println!("    }}");
            println!("}}");
            println!("impl {} {{", wrapper_name);
            println!("    pub fn into_inner(self) -> ComRc<dyn {}> {{", i.name);
            println!("        self.inner");
            println!("    }}");
            println!("    pub fn as_inner(&self) -> &ComRc<dyn {}> {{", i.name);
            println!("        &self.inner");
            println!("    }}");
            println!("}}");
            println!();
        }

        return;
    }

    print!(
        "{}",
        r#"//! Low Level Bindings for WebView2 SDK.
#![cfg(windows)]
#![allow(clippy::missing_safety_doc, non_snake_case, clippy::upper_case_acronyms)]

// Generated by idl2rs.

use com::{com_interface, interfaces::{IUnknown, iunknown::IUnknownVTable}};
use winapi::shared::minwindef::{*, ULONG};
use winapi::shared::ntdef::*;
use winapi::shared::windef::*;
use winapi::um::oaidl::VARIANT;
use winapi::um::objidlbase::STATSTG;
use std::ffi::c_void;

/// Represents a reference to a delegate that receives change notifications.
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct EventRegistrationToken {
    value: i64,
}

#[com_interface("0c733a30-2a1c-11ce-ade5-00aa0044773d")]
pub trait ISequentialStream: IUnknown {
    unsafe fn read(
        &self,
        pv: *mut c_void,
        cb: ULONG,
        pcbRead: *mut ULONG
    ) -> HRESULT;
    unsafe fn write(
        &self,
        pv: *const c_void,
        cb: ULONG,
        pcbWritten: *mut ULONG
    ) -> HRESULT;
}

#[com_interface("0000000c-0000-0000-C000-000000000046")]
pub trait IStream: ISequentialStream {
    unsafe fn seek(
        &self,
        dlibMove: LARGE_INTEGER,
        dwOrigin: DWORD,
        plibNewPosition: *mut ULARGE_INTEGER
    ) -> HRESULT;
    unsafe fn set_size(&self, libNewSize: ULARGE_INTEGER) -> HRESULT;
    unsafe fn copy_to(
        &self,
        pstm: *mut *mut IStreamVTable,
        cb: ULARGE_INTEGER,
        pcbRead: *mut ULARGE_INTEGER,
        pcbWritten: *mut ULARGE_INTEGER
    ) -> HRESULT;
    unsafe fn commit(&self, grfCommitFlags: DWORD) -> HRESULT;
    unsafe fn revert(&self) -> HRESULT;
    unsafe fn lock_region(
        &self,
        libOffset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        dwLockType: DWORD
    ) -> HRESULT;
    unsafe fn unlock_region(
        &self,
        libOffset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        dwLockType: DWORD
    ) -> HRESULT;
    unsafe fn stat(&self, pstatstg: *mut STATSTG, grfStatFlag: DWORD) -> HRESULT;
    unsafe fn clone(&self, ppstm: *mut *mut *mut IStreamVTable) -> HRESULT;
}


/// DLL export to create a WebView2 environment with a custom version of Edge,
/// user data directory and/or additional options.
///
/// browserExecutableFolder is the relative path to the folder that
/// contains the embedded Edge. The embedded Edge can be obtained by
/// copying the version named folder of an installed Edge, like
/// 73.0.52.0 sub folder of an installed 73.0.52.0 Edge. The folder
/// should have msedge.exe, msedge.dll, and so on.
/// Use null or empty string for browserExecutableFolder to create
/// WebView using Edge installed on the machine, in which case the
/// API will try to find a compatible version of Edge installed on the
/// machine according to the channel preference trying to find first
/// per user install and then per machine install.
///
/// The default channel search order is stable, beta, dev, and canary.
/// When there is an override WEBVIEW2_RELEASE_CHANNEL_PREFERENCE environment
/// variable or applicable releaseChannelPreference registry value
/// with the value of 1, the channel search order is reversed.
///
/// userDataFolder can be
/// specified to change the default user data folder location for
/// WebView2. The path can be an absolute file path or a relative file path
/// that is interpreted as relative to the current process's executable.
/// Otherwise, for UWP apps, the default user data folder will be
/// the app data folder for the package; for non-UWP apps,
/// the default user data folder `{Executable File Name}.WebView2`
/// will be created in the same directory next to the app executable.
/// WebView2 creation can fail if the executable is running in a directory
/// that the process doesn't have permission to create a new folder in.
/// The app is responsible to clean up its user data folder
/// when it is done.
///
/// Note that as a browser process might be shared among WebViews,
/// WebView creation will fail with HRESULT_FROM_WIN32(ERROR_INVALID_STATE) if
/// the specified options does not match the options of the WebViews that are
/// currently running in the shared browser process.
///
/// environment_created_handler is the handler result to the async operation
/// which will contain the WebView2Environment that got created.
///
/// The browserExecutableFolder, userDataFolder and additionalBrowserArguments
/// of the environmentOptions may be overridden by
/// values either specified in environment variables or in the registry.
///
/// When creating a WebView2Environment the following environment variables
/// are checked:
///
/// ```
/// WEBVIEW2_BROWSER_EXECUTABLE_FOLDER
/// WEBVIEW2_USER_DATA_FOLDER
/// WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS
/// WEBVIEW2_RELEASE_CHANNEL_PREFERENCE
/// ```
///
/// If an override environment variable is found then we use the
/// browserExecutableFolder, userDataFolder and additionalBrowserArguments
/// values as replacements for the corresponding values in
/// CreateCoreWebView2EnvironmentWithOptions parameters.
///
/// While not strictly overrides, there exists additional environment variables
/// that can be set:
///
/// ```
/// WEBVIEW2_WAIT_FOR_SCRIPT_DEBUGGER
/// ```
///
/// When found with a non-empty value, this indicates that the WebView is being
/// launched under a script debugger. In this case, the WebView will issue a
/// `Page.waitForDebugger` CDP command that will cause script execution inside the
/// WebView to pause on launch, until a debugger issues a corresponding
/// `Runtime.runIfWaitingForDebugger` CDP command to resume execution.
/// Note: There is no registry key equivalent of this environment variable.
///
/// ```
/// WEBVIEW2_PIPE_FOR_SCRIPT_DEBUGGER
/// ```
///
/// When found with a non-empty value, this indicates that the WebView is being
/// launched under a script debugger that also supports host applications that
/// use multiple WebViews. The value is used as the identifier for a named pipe
/// that will be opened and written to when a new WebView is created by the host
/// application. The payload will match that of the remote-debugging-port JSON
/// target and can be used by the external debugger to attach to a specific
/// WebView instance.
/// The format of the pipe created by the debugger should be:
/// `\\.\pipe\WebView2\Debugger\{app_name}\{pipe_name}`
/// where:
///
/// - `{app_name}` is the host application exe filename, e.g. WebView2Example.exe
/// - `{pipe_name}` is the value set for WEBVIEW2_PIPE_FOR_SCRIPT_DEBUGGER.
///
/// To enable debugging of the targets identified by the JSON you will also need
/// to set the WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS environment variable to
/// send `--remote-debugging-port={port_num}`
/// where:
///
/// - `{port_num}` is the port on which the CDP server will bind.
///
/// Be aware that setting both the WEBVIEW2_PIPE_FOR_SCRIPT_DEBUGGER and
/// WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS environment variables will cause the
/// WebViews hosted in your application and their contents to be exposed to
/// 3rd party applications such as debuggers.
///
/// Note: There is no registry key equivalent of this environment variable.
///
/// If none of those environment variables exist, then the registry is examined next.
/// The following registry keys are checked:
///
/// ```
/// [{Root}\Software\Policies\Microsoft\EmbeddedBrowserWebView\LoaderOverride\{AppId}]
/// "releaseChannelPreference"=dword:00000000
/// "browserExecutableFolder"=""
/// "userDataFolder"=""
/// "additionalBrowserArguments"=""
/// ```
///
/// In the unlikely scenario where some instances of WebView are open during
/// a browser update we could end up blocking the deletion of old Edge browsers.
/// To avoid running out of disk space a new WebView creation will fail
/// with the next error if it detects that there are many old versions present.
///
/// ```
/// ERROR_DISK_FULL
/// ```
///
/// The default maximum number of Edge versions allowed is 20.
///
/// The maximum number of old Edge versions allowed can be overwritten with the value
/// of the following environment variable.
///
/// ```
/// WEBVIEW2_MAX_INSTANCES
/// ```
///
/// If the Webview depends on an installed Edge and it is uninstalled
/// any subsequent creation will fail with the next error
///
/// ```
/// ERROR_PRODUCT_UNINSTALLED
/// ```
///
/// First we check with Root as HKLM and then HKCU.
/// AppId is first set to the Application User Model ID of the caller's process,
/// then if there's no corresponding registry key the AppId is
/// set to the executable name of the caller's process, or if that
/// isn't a registry key then '*'. If an override registry key is found then we
/// use the browserExecutableFolder, userDataFolder and additionalBrowserArguments
/// registry values as replacements for the corresponding values in
/// CreateCoreWebView2EnvironmentWithOptions parameters.
extern "stdcall" {
    pub fn CreateCoreWebView2EnvironmentWithOptions(
        browserExecutableFolder: PCWSTR,
        userDataFolder: PCWSTR,
        environment_options: *mut *mut ICoreWebView2EnvironmentOptionsVTable,
        environment_created_handler: *mut *mut ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandlerVTable
    ) -> HRESULT;
}

/// Get the browser version info including channel name if it is not the stable channel
/// or the Embedded Edge.
/// Channel names are beta, dev, and canary.
/// If an override exists for the browserExecutableFolder or the channel preference,
/// the override will be used.
/// If there isn't an override, then the parameter passed to
/// GetAvailableCoreWebView2BrowserVersionString is used.
extern "stdcall" {
    pub fn GetAvailableCoreWebView2BrowserVersionString(
        browser_executable_folder: PCWSTR,
        version_info: *mut LPWSTR,
    ) -> HRESULT;
}

/// This method is for anyone want to compare version correctly to determine
/// which version is newer, older or same. It can be used to determine whether
/// to use webview2 or certain feature base on version.
/// Sets the value of result to -1, 0 or 1 if version1 is less than, equal or
/// greater than version2 respectively.
/// Returns E_INVALIDARG if it fails to parse any of the version strings or any
/// input parameter is null.
/// Input can directly use the versionInfo obtained from
/// GetAvailableCoreWebView2BrowserVersionString, channel info will be ignored.
extern "stdcall" {
    pub fn CompareBrowserVersions(
        version1: PCWSTR,
        version2: PCWSTR,
        result: *mut i32,
    ) -> HRESULT;
}
"#
    );
    doc.render(&mut io::stdout()).unwrap();
}
