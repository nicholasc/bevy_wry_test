use bevy::{prelude::*, winit::WinitWindows};
use std::ops::Deref;
use wry::{
    dpi::{LogicalPosition, PhysicalSize},
    http::Request,
    WebView, WebViewBuilder,
};

struct WebViewWrapper {
    pub inner: Option<WebView>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Wry".to_string(),
                clip_children: false,
                ..default()
            }),
            ..Default::default()
        }))
        .add_systems(
            Startup,
            (
                setup_wrapper.before(setup_wry),
                setup_wry,
                setup_scene.after(setup_wry),
            ),
        )
        .run();
}

fn setup_wrapper(world: &mut World) {
    world.insert_non_send_resource(WebViewWrapper { inner: None });
}

fn setup_wry(
    window: Query<(Entity, &Window)>,
    windows: NonSend<WinitWindows>,
    mut webview_wrapper: NonSendMut<WebViewWrapper>,
) {
    let (entity, _) = window.single();
    let window = windows.get_window(entity).unwrap();

    let webview = WebViewBuilder::new()
        .with_transparent(true)
        .with_ipc_handler(|request: Request<String>| {
            dbg!(request);
        })
        .with_html(include_str!("../index.html"))
        .build_as_child(&window.deref())
        .unwrap();

    webview
        .set_bounds(wry::Rect {
            position: LogicalPosition::new(0, 0).into(),
            size: PhysicalSize::new(window.inner_size().width, window.inner_size().height).into(),
        })
        .unwrap();

    webview_wrapper.inner = Some(webview);
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    // camera
    commands.spawn((Camera3d::default(),));
}
