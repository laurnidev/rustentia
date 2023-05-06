use eframe::{
    egui::{Frame, Margin},
    epaint::{Color32, Rounding, Shadow, Stroke},
};

pub fn custom_frame() -> Frame {
    Frame {
        inner_margin: Margin {
            left: 5.0,
            right: 5.0,
            top: 5.0,
            bottom: 5.0,
        },
        outer_margin: Margin {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        },
        rounding: Rounding {
            nw: 2.0,
            ne: 2.0,
            sw: 2.0,
            se: 2.0,
        },
        shadow: Shadow::NONE,
        fill: Color32::from_rgb(40, 40, 40),
        stroke: Stroke {
            width: 0.5,
            color: Color32::DARK_GRAY,
        },
    }
}
