use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_inspector_egui::egui;
use i18n::t;

pub struct DiagnosticsWindow;
impl EditorWindow for DiagnosticsWindow {
    type State = ();
    const NAME: &'static str = "Diagnostics";

    fn ui(world: &mut World, _cx: EditorWindowContext, ui: &mut egui::Ui) {
        let diagnostics = match world.get_resource::<DiagnosticsStore>() {
            Some(diagnostics) => diagnostics,
            None => {
                ui.label(String::from(t!("Diagnostics resource not available")));
                return;
            }
        };
        diagnostic_ui(ui, diagnostics);
    }
}

fn diagnostic_ui(ui: &mut egui::Ui, diagnostics: &DiagnosticsStore) {
    egui::Grid::new("frame time diagnostics").show(ui, |ui| {
        let mut has_diagnostics = false;
        for diagnostic in diagnostics.iter() {
            has_diagnostics = true;
            ui.label(String::from(t!(diagnostic.path().as_str())));
            if let Some(average) = diagnostic.average() {
                ui.label(format!("{:.2}", average));
            }
            ui.end_row();
        }

        if !has_diagnostics {
            ui.label(
                String::from(t!(r#"No diagnostics found. Possible plugins to add:
            - `FrameTimeDiagnosticsPlugin`
            - `EntityCountDiagnisticsPlugin`
            - `AssetCountDiagnosticsPlugin`
            "#))
                ,
            );
        }
    });
}
