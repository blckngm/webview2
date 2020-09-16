use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use winapi::shared::windef::*;
use winapi::um::winuser::*;
use winit::dpi::Size;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::windows::WindowExtWindows;
use winit::window::WindowBuilder;

fn main() {
    // First we need to create an window. Here we use winit, but you can also
    // use other crates, or use raw windows APIs. Just make sure that you can
    // have the HWND, and can handle window messages, esp. WM_SIZE.
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WebView2 - Host Web Communication")
        .with_inner_size(Size::Logical((900, 600).into()))
        .build(&event_loop)
        .unwrap();

    let webview_controller: Rc<RefCell<Option<webview2::Controller>>> = Rc::new(RefCell::new(None));
    let webview: Rc<RefCell<Option<webview2::WebView>>> = Rc::new(RefCell::new(None));

    let create_result = {
        let webview_controller = webview_controller.clone();
        let hwnd = window.hwnd() as HWND;

        // Create an webview2 environment.
        webview2::EnvironmentBuilder::new().build(move |env| {
            // Then we create the controller. Here we need the HWND.
            env.expect("env").create_controller(hwnd, move |controller| {
                let controller = controller.expect("create host");
                // Then we get the WebView.
                let w = controller.get_webview().expect("get_webview");

                // We can manipulate settings of the webview.
                let _ = w.get_settings().map(|settings| {
                    let _ = settings.put_is_status_bar_enabled(false);
                    let _ = settings.put_are_default_context_menus_enabled(false);
                    let _ = settings.put_is_zoom_control_enabled(false);
                });

                // Tell the controller it should occupy the whole parent window.
                unsafe {
                    let mut rect = mem::zeroed();
                    GetClientRect(hwnd, &mut rect);
                    controller.put_bounds(rect).expect("put_bounds");
                }

                // Make the webview display some HTML.
                w.navigate_to_string("<h2>WebView2 - Host Web Communication</h2><script>window.chrome.webview.postMessage('hello from web-view!')</script>").expect("navigate to string");
                // Host-web communication. Add a callback for web messages.
                w.add_web_message_received(|w, args| {
                    let message = args.try_get_web_message_as_string()?;
                    println!("Message from webview: {}", message);
                    // Execute some script in the webview.
                    w.execute_script("document.write('<h2>WebView2 - Host Web Communication</h2><p>I got your message!</p>')", |_| Ok(()))
                }).expect("add_web_message_received");

                *webview_controller.borrow_mut() = Some(controller);
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
                    if let Some(ref webview_host) = webview_controller.borrow().as_ref() {
                        webview_host.close().expect("close");
                    }
                    *control_flow = ControlFlow::Exit;
                }
                // Notify the webview when the parent window is moved.
                WindowEvent::Moved(_) => {
                    if let Some(ref webview_host) = webview_controller.borrow().as_ref() {
                        let _ = webview_host.notify_parent_window_position_changed();
                    }
                }
                // Update webview bounds when the parent window is resized.
                WindowEvent::Resized(new_size) => {
                    if let Some(ref webview_host) = webview_controller.borrow().as_ref() {
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
                // In a production application you should probably wire the
                // visibility of the webview to SC_MINIMIZE and SC_RESTORE
                // system commands.
                if input.virtual_keycode == Some(winit::event::VirtualKeyCode::S) {
                    if let Some(ref webview_host) = webview_controller.borrow().as_ref() {
                        webview_host
                            .put_is_visible(true)
                            .expect("put_is_visible true");
                    }
                } else if input.virtual_keycode == Some(winit::event::VirtualKeyCode::H) {
                    if let Some(ref webview_host) = webview_controller.borrow().as_ref() {
                        webview_host
                            .put_is_visible(false)
                            .expect("put_is_visible false");
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
