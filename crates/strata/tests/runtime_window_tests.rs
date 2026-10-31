use strata::prelude::*;
use strata_dom::View;
use strata_platform::{MockEventLoop, MockPlatformWindow, PlatformEvent, SoftwareSurfacePresenter, SwapchainCompositor, SwapchainFormat, WindowConfig};

#[component]
fn TestWindowApp() -> View {
    rsx! {
        <div class="flex flex-col w-full h-full bg-[#0a0a0a]">
            <header class="flex h-[60px] bg-[#1a1a1a]">
                <span class="text-white">"Strata Live Window"</span>
            </header>
            <main class="flex flex-1 p-4">
                <div class="flex w-[200px] h-[100px] bg-[#2a2a2a]">
                    <span>"Content Card"</span>
                </div>
            </main>
        </div>
    }
}

#[test]
fn test_strata_launch_window_initialization_and_first_frame() {
    let mut config = WindowConfig::default();
    config.title = "Strata Test Application".to_string();
    config.width = 1280;
    config.height = 720;

    let (runtime, window, presenter) = strata::launch_window(TestWindowApp, config);

    assert_eq!(window.title(), "Strata Test Application");
    assert_eq!(window.size(), (1280, 720));
    assert_eq!(presenter.width, 1280);
    assert_eq!(presenter.height, 720);
    assert_eq!(presenter.presented_frames, 1);
    assert_eq!(presenter.format, SwapchainFormat::Bgra8Unorm);

    // Verify initial layout & display list
    assert_eq!(runtime.viewport_width, 1280.0);
    assert_eq!(runtime.viewport_height, 720.0);
    assert!(runtime.display_list.commands.len() > 0);
}

#[test]
fn test_app_runtime_step_event_resize_and_scale() {
    let view = TestWindowApp();
    let mut runtime = AppRuntime::new(view, 800.0, 600.0, 1.0);
    assert_eq!(runtime.viewport_width, 800.0);
    assert_eq!(runtime.viewport_height, 600.0);
    assert_eq!(runtime.dpr, 1.0);

    // Resize event
    let resize_event = PlatformEvent::WindowResized { width: 1920, height: 1080 };
    let dirty = runtime.step_event(&resize_event);
    assert!(dirty);
    assert_eq!(runtime.viewport_width, 1920.0);
    assert_eq!(runtime.viewport_height, 1080.0);

    // Scale factor event
    let scale_event = PlatformEvent::ScaleFactorChanged { scale_factor: 2.0 };
    let dirty_scale = runtime.step_event(&scale_event);
    assert!(dirty_scale);
    assert_eq!(runtime.dpr, 2.0);

    // Focus event should not trigger dirty layout
    let focus_event = PlatformEvent::WindowFocused(true);
    assert!(!runtime.step_event(&focus_event));
}

#[test]
fn test_app_runtime_run_with_window_event_loop() {
    let view = TestWindowApp();
    let mut runtime = AppRuntime::new(view, 800.0, 600.0, 1.0);

    let mut window = MockPlatformWindow::new_mock();
    let mut event_loop = MockEventLoop::new();
    let mut compositor = SwapchainCompositor::new(800, 600, 1.0);
    let mut presenter = SoftwareSurfacePresenter::new(800, 600, SwapchainFormat::Bgra8Unorm);

    // Queue simulated OS events
    event_loop.queue_event(PlatformEvent::WindowResized { width: 1024, height: 768 });
    event_loop.queue_event(PlatformEvent::WindowFocused(true));
    event_loop.queue_event(PlatformEvent::WindowCloseRequested);

    runtime.run_with_window(&mut window, &mut event_loop, &mut compositor, &mut presenter);

    // Verify final state
    assert_eq!(runtime.viewport_width, 1024.0);
    assert_eq!(runtime.viewport_height, 768.0);
    assert!(presenter.presented_frames >= 2); // Initial frame + resize frame
}
