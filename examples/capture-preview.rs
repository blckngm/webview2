use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use webview2;
use winapi::shared::windef::*;
use winapi::um::winuser::*;
use winit::dpi::Size;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::windows::WindowExtWindows;
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WebView2 - Host Web Communication")
        .with_inner_size(Size::Logical((900, 600).into()))
        .build(&event_loop)
        .unwrap();

    let webview_host: Rc<RefCell<Option<webview2::Host>>> = Rc::new(RefCell::new(None));
    let webview: Rc<RefCell<Option<webview2::WebView>>> = Rc::new(RefCell::new(None));

    let create_result = {
        let webview = webview.clone();
        let webview_host = webview_host.clone();
        let hwnd = window.hwnd() as HWND;

        webview2::EnvironmentBuilder::new().build(move |env| {
            env.expect("env").create_host(hwnd, move |host| {
                let host = host.expect("create host");
                let w = host.get_webview().expect("get_webview");

                let _ = w.get_settings().map(|settings| {
                    let _ = settings.put_is_status_bar_enabled(false);
                    let _ = settings.put_are_default_context_menus_enabled(false);
                    let _ = settings.put_is_zoom_control_enabled(false);
                });

                unsafe {
                    let mut rect = mem::zeroed();
                    GetClientRect(hwnd, &mut rect);
                    host.put_bounds(rect).expect("put_bounds");
                }

                w.navigate("https://wikipedia.org").unwrap();

                *webview_host.borrow_mut() = Some(host);
                *webview.borrow_mut() = Some(w);
                Ok(())
            })
        })
    };
    if let Err(e) = create_result {
        eprintln!(
            "Failed to create webview environment: {}. Is the new edge browser installed?",
            e
        );
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    if let Some(ref webview_host) = webview_host.borrow().as_ref() {
                        webview_host.close().expect("close");
                    }
                    *control_flow = ControlFlow::Exit;
                }
                // Notify the webview when the parent window is moved.
                WindowEvent::Moved(_) => {
                    if let Some(ref webview_host) = webview_host.borrow().as_ref() {
                        let _ = webview_host.notify_parent_window_position_changed();
                    }
                }
                // Update webview bounds when the parent window is resized.
                WindowEvent::Resized(new_size) => {
                    if let Some(ref webview_host) = webview_host.borrow().as_ref() {
                        let r = RECT {
                            left: 0,
                            top: 0,
                            right: new_size.width as i32,
                            bottom: new_size.height as i32,
                        };
                        webview_host.put_bounds(r).expect("put_bounds");
                    }
                }
                _ => {}
            },
            Event::DeviceEvent {
                event: winit::event::DeviceEvent::Key(input),
                ..
            } => {
                if input.virtual_keycode == Some(winit::event::VirtualKeyCode::Snapshot) {
                    if let Some(ref webview) = webview.borrow().as_ref() {
                        let mut stream = webview2::Stream::from_bytes(&[]);
                        webview.capture_preview(
                            webview2_sys::CORE_WEBVIEW2_CAPTURE_PREVIEW_IMAGE_FORMAT::CORE_WEBVIEW2_CAPTURE_PREVIEW_IMAGE_FORMAT_PNG,
                            stream.clone(),
                            move |r| {
                                if r.is_ok() {
                                    use std::io::{self, Seek};
                                    stream.seek(io::SeekFrom::Start(0)).unwrap();
                                    let mut preview_png = std::fs::OpenOptions::new()
                                        .create(true)
                                        .write(true)
                                        .truncate(true)
                                        .open("preview.png")
                                        .unwrap();
                                    io::copy(&mut stream, &mut preview_png).unwrap();
                                }
                                Ok(())
                            }
                        ).unwrap();
                    }
                }
            }
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {}
            _ => (),
        }
    });
}
