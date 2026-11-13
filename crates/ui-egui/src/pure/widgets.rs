//! The ribbon's controls as the Pure edition draws them: an even grid of 24-point rows, 16-point
//! small icons and 24-point large ones, flat at rest, with hover, pressed, on, disabled and
//! keyboard-focus states from the Paper palette. Each function takes over from the upstream
//! widget of the same name in `widgets.rs` and keeps its click targets and accessible names.

use egui::{Align2, Color32, CornerRadius, Rect, Response, Sense, Stroke, StrokeKind, Ui, pos2, vec2};

use super::{ICON, ICON_LARGE, Paper, RADIUS, ROW_H, paper};
use crate::icons::{self, Icon};
use crate::theme::{self, Tokens};

/// Height of the ribbon's content: three rows.
pub const RIBBON_H: f32 = 3.0 * ROW_H;
/// Width of the dropdown target at the right of a split button.
const ARROW_W: f32 = 14.0;
/// Where a large button's label starts, below its icon.
const LABEL_TOP: f32 = 36.0;
/// Distance between the two lines of a large button's label.
const LABEL_PITCH: f32 = 14.0;

fn radius() -> CornerRadius {
    CornerRadius::same(RADIUS)
}

fn sense(enabled: bool) -> Sense {
    if enabled { Sense::click() } else { Sense::hover() }
}

/// The fill behind a control for its state, if any: on, pressed, hover.
pub(crate) fn fill(ui: &Ui, r: &Response, on: bool, p: &Paper) -> Option<Color32> {
    if !r.sense.senses_click() && !r.sense.senses_drag() {
        return None;
    }
    if on {
        Some(p.on_fill)
    } else if r.is_pointer_button_down_on() {
        Some(p.pressed)
    } else if r.hovered() && ui.is_enabled() {
        Some(p.hover)
    } else {
        None
    }
}

/// Paints a control's state fill and, with keyboard focus, a 1-point accent outline.
pub(crate) fn state(ui: &Ui, painter: &egui::Painter, rect: Rect, r: &Response, on: bool, p: &Paper) {
    if let Some(c) = fill(ui, r, on, p) {
        painter.rect_filled(rect, radius(), c);
    }
    if r.has_focus() {
        painter.rect_stroke(rect, radius(), Stroke::new(1.0, p.accent), StrokeKind::Inside);
    }
}

/// The ink of a control's icon and label: the accent mark when on.
fn ink(on: bool, p: &Paper) -> Color32 {
    if on { p.on_mark } else { p.text }
}

/// A 16-point icon centred on `c`, on whole pixels so its strokes stay sharp.
fn small_icon(painter: &egui::Painter, c: egui::Pos2, icon: Icon, ink: Color32, disabled: bool) {
    let min = pos2((c.x - ICON / 2.0).round(), (c.y - ICON / 2.0).round());
    icons::paint(painter, Rect::from_min_size(min, vec2(ICON, ICON)), icon, ink, disabled);
}

fn text_width(ui: &Ui, text: &str, font: &egui::FontId) -> f32 {
    ui.fonts_mut(|f| f.layout_no_wrap(text.to_string(), font.clone(), Color32::WHITE).size().x)
}

/// A large ribbon button: a 24-point icon over a one- or two-line label, three rows tall. With
/// `split`, the chevron at the end of the label is its own target.
pub fn big_button_ex(ui: &mut Ui, icon: Icon, label: &str, enabled: bool, split: bool, selected: bool) -> (Response, Option<Response>) {
    let p = paper(Tokens::get(ui.ctx()).dark);
    let font = theme::font(12.0);
    let lines = crate::widgets::wrap_label(label);
    let text_w = lines.iter().map(|l| text_width(ui, l, &font)).fold(0.0f32, f32::max);
    let w = (text_w + 12.0 + if split { ARROW_W } else { 0.0 }).max(48.0).ceil();
    let (rect, resp) = ui.allocate_exact_size(vec2(w, RIBBON_H), sense(enabled));
    let spoken = label.replace('\n', " ");
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, &spoken));
    let painter = ui.painter_at(rect);
    let arrow_rect = split.then(|| Rect::from_min_max(pos2(rect.max.x - ARROW_W, rect.min.y + LABEL_TOP - 2.0), rect.max));
    let arrow_resp = arrow_rect.map(|r| ui.interact(r, resp.id.with("arrow"), sense(enabled)));
    if let Some(a) = &arrow_resp {
        a.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, format!("{spoken} options")));
    }
    state(ui, &painter, rect, &resp, selected, p);
    if let (Some(ar), Some(a)) = (arrow_rect, &arrow_resp) {
        state(ui, &painter, ar, a, false, p);
    }
    let label_rect = if split { Rect::from_min_max(rect.min, pos2(rect.max.x - ARROW_W, rect.max.y)) } else { rect };
    let icon_min = pos2((label_rect.center().x - ICON_LARGE / 2.0).round(), rect.min.y + 6.0);
    icons::paint(&painter, Rect::from_min_size(icon_min, vec2(ICON_LARGE, ICON_LARGE)), icon, ink(selected, p), !enabled);
    let col = if enabled { ink(selected, p) } else { p.disabled };
    let mut y = rect.min.y + LABEL_TOP;
    for l in &lines {
        painter.text(pos2(label_rect.center().x, y), Align2::CENTER_TOP, l, font.clone(), col);
        y += LABEL_PITCH;
    }
    if split {
        let chevron_y = rect.min.y + LABEL_TOP + LABEL_PITCH * lines.len().saturating_sub(1) as f32 + 8.0;
        chevron(&painter, pos2(rect.max.x - 7.0, chevron_y), if enabled { p.secondary } else { p.disabled });
    }
    (resp, arrow_resp)
}

/// The dropdown chevron: one weight and one size everywhere.
pub fn chevron(painter: &egui::Painter, c: egui::Pos2, col: Color32) {
    let c = pos2(c.x.round() + 0.5, c.y.round());
    let s = 3.0;
    painter.line_segment([pos2(c.x - s, c.y - s * 0.5), pos2(c.x, c.y + s * 0.5)], Stroke::new(1.0, col));
    painter.line_segment([pos2(c.x, c.y + s * 0.5), pos2(c.x + s, c.y - s * 0.5)], Stroke::new(1.0, col));
}

/// A small ribbon button: a 16-point icon and a label in one 24-point row.
pub fn small_button_named(
    ui: &mut Ui,
    icon: Icon,
    label: &str,
    name: &str,
    enabled: bool,
    selected: bool,
    split: bool,
) -> (Response, Option<Response>) {
    let p = paper(Tokens::get(ui.ctx()).dark);
    let font = theme::font(12.0);
    let tw = if label.is_empty() { 0.0 } else { (text_width(ui, label, &font) + 6.0).ceil() };
    let arrow_w = if split { 12.0 } else { 0.0 };
    let (rect, resp) = ui.allocate_exact_size(vec2(ROW_H + tw + arrow_w, ROW_H), if enabled && !split { Sense::click() } else { Sense::hover() });
    if !name.is_empty() && !split {
        resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, name));
    }
    let painter = ui.painter_at(rect);
    let main_rect = Rect::from_min_max(rect.min, pos2(rect.max.x - arrow_w, rect.max.y));
    let arrow_rect = Rect::from_min_max(pos2(rect.max.x - arrow_w, rect.min.y), rect.max);
    let main_resp = if split { ui.interact(main_rect, resp.id.with("m"), sense(enabled)) } else { resp.clone() };
    let arrow_resp = split.then(|| ui.interact(arrow_rect, resp.id.with("a"), sense(enabled)));
    if split {
        main_resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, name));
        if let Some(a) = &arrow_resp {
            a.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, format!("{name} options")));
        }
    }
    state(ui, &painter, main_rect, &main_resp, selected, p);
    if let Some(a) = &arrow_resp {
        state(ui, &painter, arrow_rect, a, false, p);
    }
    small_icon(&painter, pos2(rect.min.x + ROW_H / 2.0, rect.center().y), icon, ink(selected, p), !enabled);
    if !label.is_empty() {
        let col = if enabled { ink(selected, p) } else { p.disabled };
        painter.text(pos2(rect.min.x + ROW_H, rect.center().y), Align2::LEFT_CENTER, label, font, col);
    }
    if split {
        chevron(&painter, pos2(rect.max.x - 6.0, rect.center().y), if enabled { p.secondary } else { p.disabled });
    }
    (if split { main_resp } else { resp }, arrow_resp)
}

/// A menu choice ticked in the icon column, its text lined up with the small buttons around it.
pub fn check_item(ui: &mut Ui, label: &str, enabled: bool, on: bool) -> Response {
    let p = paper(Tokens::get(ui.ctx()).dark);
    let font = theme::font(12.0);
    let tw = (text_width(ui, label, &font) + 6.0).ceil();
    let (rect, resp) = ui.allocate_exact_size(vec2(ROW_H + tw, ROW_H), sense(enabled));
    resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::CheckBox, enabled, on, label));
    let painter = ui.painter_at(rect);
    state(ui, &painter, rect, &resp, false, p);
    if on {
        small_icon(&painter, pos2(rect.min.x + ROW_H / 2.0, rect.center().y), Icon::Check, p.on_mark, !enabled);
    }
    painter.text(pos2(rect.min.x + ROW_H, rect.center().y), Align2::LEFT_CENTER, label, font, if enabled { p.text } else { p.disabled });
    resp
}

/// A square icon button, one row tall (B, I, U, alignment, the title bar's buttons).
pub fn icon_toggle(ui: &mut Ui, icon: Icon, tip: &str, enabled: bool, on: bool) -> Response {
    let p = paper(Tokens::get(ui.ctx()).dark);
    let (rect, resp) = ui.allocate_exact_size(vec2(ROW_H, ROW_H), sense(enabled));
    let painter = ui.painter_at(rect);
    state(ui, &painter, rect, &resp, on, p);
    small_icon(&painter, rect.center(), icon, ink(on, p), !enabled);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, tip));
    resp.on_hover_text(tip)
}

/// An icon with a colour bar under it and a dropdown arrow (Font Color, Highlight).
pub fn color_split(ui: &mut Ui, icon: Icon, tip: &str, color: Color32, enabled: bool) -> (Response, Response) {
    let p = paper(Tokens::get(ui.ctx()).dark);
    let (rect, resp) = ui.allocate_exact_size(vec2(ROW_H + 12.0, ROW_H), Sense::hover());
    let main = ui.interact(Rect::from_min_size(rect.min, vec2(ROW_H, ROW_H)), resp.id.with("m"), sense(enabled));
    main.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, tip));
    let arrow = ui.interact(Rect::from_min_max(pos2(rect.min.x + ROW_H, rect.min.y), rect.max), resp.id.with("a"), sense(enabled));
    arrow.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, format!("{tip} options")));
    let painter = ui.painter_at(rect);
    state(ui, &painter, main.rect, &main, false, p);
    state(ui, &painter, arrow.rect, &arrow, false, p);
    // The icon sits two points high to leave room for the colour bar beneath it.
    icons::paint(&painter, Rect::from_min_size(rect.min + vec2(4.0, 2.0), vec2(ICON, ICON)), icon, p.text, !enabled);
    painter.rect_filled(Rect::from_min_size(rect.min + vec2(4.0, 19.0), vec2(ICON, 3.0)), CornerRadius::ZERO, if enabled { color } else { p.disabled });
    chevron(&painter, pos2(rect.max.x - 6.0, rect.center().y), if enabled { p.secondary } else { p.disabled });
    (main.on_hover_text(tip), arrow)
}

/// The rule between two ribbon groups: one point wide, eight points of space either side.
pub fn separator(ui: &mut Ui) {
    let p = paper(Tokens::get(ui.ctx()).dark);
    let (rect, _) = ui.allocate_exact_size(vec2(17.0, RIBBON_H), Sense::hover());
    let x = rect.min.x.round() + 8.5;
    ui.painter().vline(x, (rect.min.y + 4.0)..=(rect.max.y - 4.0), Stroke::new(1.0, p.rule));
}

/// Rows of small controls stacked in one ribbon column. A column with fewer than three rows is
/// centred in the ribbon's height: the height its rows took on the last frame says how far down
/// it starts.
pub fn rows<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    let id = ui.auto_id_with("pure_rows");
    let last: Option<f32> = ui.ctx().data(|d| d.get_temp(id));
    let top = last.map_or(0.0, |h| ((RIBBON_H - h) / 2.0).floor().max(0.0));
    let (out, used) = ui
        .vertical(|ui| {
            ui.spacing_mut().item_spacing = vec2(1.0, 0.0);
            ui.add_space(top);
            let start = ui.cursor().min.y;
            let out = add(ui);
            (out, ui.min_rect().max.y - start)
        })
        .inner;
    if last.is_none_or(|h| (h - used).abs() > 0.5) {
        ui.ctx().data_mut(|d| d.insert_temp(id, used));
        ui.ctx().request_repaint();
    }
    out
}

/// A text dropdown field (font name, size): a 3-point box with a Line outline.
pub fn paint_dropdown(ui: &Ui, rect: Rect, resp: &Response, current: &str, enabled: bool) {
    let p = paper(Tokens::get(ui.ctx()).dark);
    let painter = ui.painter_at(rect);
    let outline = if resp.has_focus() {
        p.accent
    } else if resp.hovered() && enabled {
        p.strong
    } else {
        p.line
    };
    painter.rect(rect, radius(), p.panel, Stroke::new(1.0, outline), StrokeKind::Inside);
    let clip = Rect::from_min_max(pos2(rect.min.x + 8.0, rect.min.y), pos2(rect.max.x - 18.0, rect.max.y));
    painter.with_clip_rect(clip).text(
        pos2(rect.min.x + 8.0, rect.center().y),
        Align2::LEFT_CENTER,
        current,
        theme::font(12.0),
        if enabled { p.text } else { p.disabled },
    );
    chevron(&painter, pos2(rect.max.x - 10.0, rect.center().y), if enabled { p.secondary } else { p.disabled });
}
