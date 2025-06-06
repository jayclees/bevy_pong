use bevy::diagnostic::FrameCount;
use bevy::image::ImageLoaderSettings;
use bevy::image::ImageSampler;

use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::screen::fade::FADE_IN_SECS;
use crate::screen::fade::fade_out;
use crate::screen::title::TitleAssets;
use crate::screen::wait_in_screen;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Splash.bevy()).load_collection::<TitleAssets>(),
    );
    app.add_systems(StateFlush, Screen::Splash.on_enter(spawn_splash_screen));
    app.add_systems(
        Update,
        Screen::Splash.on_update((
            wait_in_screen(FADE_IN_SECS + SPLASH_SCREEN_MIN_SECS),
            update_splash,
        )),
    );
}

const SPLASH_SCREEN_MIN_SECS: f32 = 0.8;

#[cfg_attr(feature = "native_dev", hot)]
fn spawn_splash_screen(
    mut commands: Commands,
    screen_root: Res<ScreenRoot>,
    asset_server: Res<AssetServer>,
) {
    commands
        .entity(screen_root.ui)
        .with_child(widget::column_center(children![(
            Name::new("SplashImage"),
            ImageNode::new(asset_server.load_with_settings(
                // TODO: Workaround for <https://github.com/bevyengine/bevy/issues/14246>.
                //       Use `embedded_asset!` once that's fixed.
                "image/splash.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::linear();
                },
            )),
            Node::DEFAULT.width(70.0),
            ThemeColor::BodyText.set::<ImageNode>(),
        )]));
}

//#[cfg_attr(feature = "native_dev", hot)]
fn update_splash(
    mut commands: Commands,
    progress: Res<ProgressTracker<BevyState<Screen>>>,
    frame: Res<FrameCount>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();
    if *last_done == done {
        return;
    }
    *last_done = done;

    // Continue to the next screen when ready.
    if done == total {
        commands.spawn(fade_out(Screen::Title));
    }

    info!("[Frame {}] Booting: {done} / {total}", frame.0);
}
