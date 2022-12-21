use crate::resources::ClientRegistry;
use bevy::app::PluginGroupBuilder;
use bevy::diagnostic::{
    Diagnostic, DiagnosticId, Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
};
use bevy::prelude::{App, Plugin, PluginGroup, Res, ResMut};
use bevy::utils::Duration;

#[derive(Default)]
pub struct WestinyDiagnosticsPlugin;

impl Plugin for WestinyDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup_system)
            .add_system(Self::log_clients);
    }
}

impl WestinyDiagnosticsPlugin {
    pub const CLIENT_DIAG_ID: DiagnosticId =
        DiagnosticId::from_u128(11365578623599151819941644670456314399);

    fn log_clients(mut diag: ResMut<Diagnostics>, registry: Res<ClientRegistry>) {
        diag.add_measurement(Self::CLIENT_DIAG_ID, || registry.client_count() as f64);
    }

    fn setup_system(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(
            Self::CLIENT_DIAG_ID,
            "Number of online players",
            1,
        ));
    }
}

pub struct DiagnosticPlugins;

impl PluginGroup for DiagnosticPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(Self)
            .add(LogDiagnosticsPlugin {
                filter: Some(vec![
                    FrameTimeDiagnosticsPlugin::FPS,
                    WestinyDiagnosticsPlugin::CLIENT_DIAG_ID,
                ]),
                debug: false,
                wait_duration: Duration::from_secs(3),
            })
            .add(FrameTimeDiagnosticsPlugin::default())
            .add(WestinyDiagnosticsPlugin::default())
    }
}

impl Plugin for DiagnosticPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<Diagnostics>();
    }
}
