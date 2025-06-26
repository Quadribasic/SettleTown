use bevy::{
    diagnostic::{FrameCount, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::camera::{Camera, Viewport},
    window::{PresentMode, PrimaryWindow, WindowTheme},
};
use bevy_egui::{egui, EguiContextPass, EguiContexts, EguiPlugin};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Quadribasic Companies, LLC",
    version = "0.1.0",
    about = "SettleTown Entity Editor",
    long_about = "An entity editor for SettleTown."
)]
struct Args {
    /// Metadata for entities will be output in JSON format.
    #[arg(long)]
    output_json: bool,
}

/// Margin between UI elements.
const MARGIN: Val = Val::Px(12.);

#[derive(Component)]
pub struct EntityCamera;

#[derive(Resource)]
pub struct EditableEntity;

fn main() {
    let args = Args::parse();

    // Create the app and and default plugins before all else because this sets
    // up logging and other things that are necessary for the rest of the application.
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SettleTown Entity Editor".into(),
                name: Some("quadribasic.com".into()),
                resolution: (800., 600.).into(),
                present_mode: PresentMode::AutoVsync,

                // Tells Wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,

                // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,

                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },

                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                visible: false,
                ..default()
            }),
            ..default()
        }),
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin::default(),
        EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
    ));

    app.add_systems(EguiContextPass, initialize_ui);

    app.add_systems(Startup, setup_system);

    app.add_systems(
        Update,
        (
            // change_title,
            // toggle_theme,
            // toggle_cursor,
            // toggle_vsync,
            // toggle_window_controls,
            // cycle_cursor_icon,
            // switch_level,
            make_visible,
        ),
    );

    app.run();
}

fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.visible = true;
    }
}

fn initialize_ui(
    mut contexts: EguiContexts,
    mut camera: Single<&mut Camera, With<EntityCamera>>,
    window: Single<&mut Window, With<PrimaryWindow>>,
) {
    // egui context
    let ctx = contexts.ctx_mut();

    let mut left = egui::SidePanel::left("Entity Options")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Entity Options");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width(); // height is ignored, as the panel has a hight of 100% of the screen

    // Scale from logical units to physical units
    left *= window.scale_factor();

    // -------------------------------------------------
    // |  left   |                                     |
    // |  panel  |                                     |
    // |         |                                     |
    // |         |                                     |
    // |         |                                     |
    // |<-width->|        2D viewport                  |
    // |         |                                     |
    // |         |                                     |
    // |         |                                     |
    // |         |                                     |
    // |         |                                     |
    // -------------------------------------------------
    //
    // The upper left point of the viewport is the width of the left panel and the height of the
    // screen.
    //
    // The width of the 2D viewport the width of the remainder of the screen
    // Alternative the width can be calculated as follow:
    // size.x = window width - left panel width.
    //
    // The height of the 2d viewport is:
    // size.y = window height
    //
    // Therefore we use the alternative for the width, as we can calculate the Viewport as
    // following:

    let pos = UVec2::new(left as u32, 0);
    let size = UVec2::new(window.physical_width(), window.physical_height()) - pos;
    camera.viewport = Some(Viewport {
        physical_position: pos,
        physical_size: size,
        ..default()
    });
}

fn setup_system(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    editable_entity: Option<Res<EditableEntity>>,
) {
    if editable_entity.is_none() {
        let bold_font = asset_server.load("fonts/Montserrat-Bold.ttf");
        let regular_font = asset_server.load("fonts/Montserrat-Regular.ttf");
        let text_font_heading = TextFont {
            font: bold_font.clone(),
            font_size: 25.0,
            ..default()
        };

        let text_font_regular = TextFont {
            font: regular_font.clone(),
            font_size: 18.0,
            ..default()
        };
        let text_justification = JustifyText::Center;

        // Spawn a 2D camera that will render the scene.
        commands.spawn((Camera2d, EntityCamera));

        // Spawn some text indicating that no entities are loaded.
        commands
            .spawn(Node {
                // fill the entire window
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(MARGIN),
                row_gap: MARGIN,
                ..Default::default()
            })
            .with_children(|builder| {
                spawn_nested_text_bundle(
                    builder,
                    text_font_heading,
                    UiRect::right(MARGIN),
                    text_justification,
                    "No Entities Loaded",
                );
                spawn_nested_text_bundle(
                    builder,
                    text_font_regular,
                    UiRect::right(MARGIN),
                    text_justification,
                    "Create a new entity with CMD+N or load an existing entity package with CMD+O.",
                );
            });

        // // Circle
        // commands.spawn((
        //     Mesh2d(meshes.add(Circle::new(50.))),
        //     MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.1, 0.0)))),
        //     Transform::from_translation(Vec3::new(-150., 0., 0.)),
        // ));

        // // Rectangles
        // commands.spawn((
        //     Mesh2d(meshes.add(Rectangle::new(50., 100.))),
        //     MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.5, 0.4, 0.3)))),
        //     Transform::from_translation(Vec3::new(-50., 0., 0.)),
        // ));

        // commands.spawn((
        //     Mesh2d(meshes.add(Rectangle::new(50., 100.))),
        //     MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.5, 0.4, 0.3)))),
        //     Transform::from_translation(Vec3::new(50., 0., 0.)),
        // ));

        // // Hexagon
        // commands.spawn((
        //     Mesh2d(meshes.add(RegularPolygon::new(50., 6))),
        //     MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.8, 0.7, 0.6)))),
        //     Transform::from_translation(Vec3::new(150., 0., 0.)),
        // ));
    }
}

fn spawn_nested_text_bundle(
    builder: &mut ChildSpawnerCommands,
    font: TextFont,
    margin: UiRect,
    text_justification: JustifyText,
    text: &str,
) {
    builder
        .spawn((Node {
            margin,
            padding: UiRect::axes(Val::Px(5.), Val::Px(1.)),
            width: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                Text::new(text),
                font.clone(),
                TextColor::WHITE,
                TextLayout::new_with_justify(text_justification),
            ));
        });
}
