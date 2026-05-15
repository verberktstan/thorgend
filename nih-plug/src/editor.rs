use crate::thorgend_params::ThorgendParams;
use nih_plug::prelude::{Editor, ParamSetter};
use nih_plug_egui::{create_egui_editor, egui, widgets::ParamSlider, EguiState};
use std::sync::Arc;

pub fn create(params: Arc<ThorgendParams>, egui_state: Arc<EguiState>) -> Option<Box<dyn Editor>> {
  create_egui_editor(
    egui_state,
    params,
    |_ctx, _params| {},
    |ctx, setter, params| {
      draw(ctx, setter, params);
    },
  )
}

fn draw(ctx: &egui::Context, setter: &ParamSetter, params: &Arc<ThorgendParams>) {
  egui::CentralPanel::default().show(ctx, |ui| {
    ui.add(ParamSlider::for_param(&params.voices, setter));
    ui.add(ParamSlider::for_param(&params.num_cps, setter));
    ui.add(ParamSlider::for_param(&params.attack, setter));
    ui.add(ParamSlider::for_param(&params.decay, setter));
    ui.add(ParamSlider::for_param(&params.sustain, setter));
    ui.add(ParamSlider::for_param(&params.release, setter));
    ui.add(ParamSlider::for_param(&params.output_gain, setter));
    ui.add(ParamSlider::for_param(&params.noisyness, setter));
    ui.add(ParamSlider::for_param(&params.noiseindex, setter));
    ui.add(ParamSlider::for_param(&params.noisespeed, setter));
    ui.add(ParamSlider::for_param(&params.lfo_sh_freq, setter));
    ui.add(ParamSlider::for_param(&params.dichotomization, setter));
  });
}
