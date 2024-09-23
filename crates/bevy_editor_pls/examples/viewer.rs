use bevy::{
    asset::AssetPath, diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}, prelude::*, render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    }
};
use bevy_editor_pls::prelude::*;
use egui::epaint::tessellator::{path, Path};
use bevy_stl::StlPlugin;
use bevy_obj::ObjPlugin;

fn main() {
    // enable wireframe rendering
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings.features |= WgpuFeatures::POLYGON_MODE_LINE;

    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(wgpu_settings),
            ..default()
        }))
        .add_plugins((
            EditorPlugin::new(),
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            StlPlugin,
            ObjPlugin
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, file_drag_and_drop_system)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        ..Default::default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}


fn file_drag_and_drop_system(
    mut events: EventReader<FileDragAndDrop>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        info!("{:?}", event);
        match event {
            FileDragAndDrop::DroppedFile { path_buf, .. } => {
                let path = path_buf.to_string_lossy().to_string();
                // 获取文件后缀
                let file_extension = path.split('.').last().unwrap_or("");

                match file_extension {
                    "glb" | "gltf" => {
                        let scene_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset(path));
                        // 在这里确保 scene_handle 只在加载成功后使用
                        commands.spawn(SceneBundle {
                            scene: scene_handle,
                            ..default()
                        });
                    }
                    "stl"|"obj" => {
                        let mesh_handle = asset_server.load(path);
                        commands.spawn(PbrBundle {
                            mesh: mesh_handle,
                            ..default()
                        });
                    }
                    _ => {
                        info!("Unsupported file type: {}", file_extension);
                        return;
                    }
                }
            }
            _ => {}
        }
    }
}
