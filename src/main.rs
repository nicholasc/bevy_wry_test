use bevy::{prelude::*, winit::WinitWindows};
use std::ops::Deref;
use wry::{
    dpi::{LogicalPosition, PhysicalSize},
    http::Request,
    WebView, WebViewBuilder,
};

#[derive(Default)]
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
            ..default()
        }))
        .add_systems(Startup, (setup_wry, setup_scene.after(setup_wry)))
        .init_non_send_resource::<WebViewWrapper>()
        .run();
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
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
