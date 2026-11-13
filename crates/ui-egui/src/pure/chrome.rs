//! The Pure edition's window chrome: egui's own widget style, the interface font, the title bar
//! and the ribbon's tab strip and frame.

use egui::{Align2, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, Rect, Sense, Stroke, StrokeKind, Ui, pos2, vec2};
use serde_json::json;

use super::widgets::{self, RIBBON_H};
use super::{ICON, NAME, Paper, RADIUS, ROW_H, paper};
use crate::SlideApp;
use crate::icons::{self, Icon};
use crate::theme::{self, Tokens};

/// Height of the title bar.
pub const TITLE_H: f32 = 40.0;
/// Height of the ribbon's tab strip.
pub const TABS_H: f32 = 32.0;
/// Space above and below the ribbon's controls.
const RIBBON_PAD: f32 = 8.0;
/// Width of the File screen's side list.
pub const NAV_W: f32 = 220.0;

// ---------------------------------------------------------------------------------------------
// egui's style
// ---------------------------------------------------------------------------------------------

/// The palette on egui's own widgets (buttons, fields, checkboxes, menus, dialogs): 3-point
/// corners, a Line outline, no shadows, no growth on hover.
pub fn visuals(v: &mut egui::Visuals, t: &Tokens) {
    let p = paper(t.dark);
    let r = CornerRadius::same(RADIUS);
    v.panel_fill = p.frame;
    v.window_fill = p.popup;
    v.window_stroke = Stroke::new(1.0, p.line);
    v.window_corner_radius = r;
    v.menu_corner_radius = r;
    v.window_shadow = egui::epaint::Shadow::NONE;
    v.popup_shadow = egui::epaint::Shadow::NONE;
    v.extreme_bg_color = p.panel;
    v.text_edit_bg_color = Some(p.panel);
    v.faint_bg_color = p.hover;
    v.selection.bg_fill = p.selection;
    v.selection.stroke = Stroke::new(1.0, p.text);
    v.hyperlink_color = p.accent;
    v.warn_fg_color = p.danger;
    v.error_fg_color = p.danger;
    let w = &mut v.widgets;
    for s in [&mut w.noninteractive, &mut w.inactive, &mut w.hovered, &mut w.active, &mut w.open] {
        s.corner_radius = r;
        s.expansion = 0.0;
        s.fg_stroke = Stroke::new(1.0, p.text);
    }
    w.noninteractive.bg_stroke = Stroke::new(1.0, p.rule);
    w.noninteractive.fg_stroke = Stroke::new(1.0, p.secondary);
    w.inactive.weak_bg_fill = Color32::TRANSPARENT;
    w.inactive.bg_fill = p.rule;
    w.inactive.bg_stroke = Stroke::new(1.0, p.line);
    w.hovered.weak_bg_fill = p.hover;
    w.hovered.bg_fill = p.line;
    w.hovered.bg_stroke = Stroke::new(1.0, p.strong);
    w.active.weak_bg_fill = p.pressed;
    w.active.bg_fill = p.strong;
    w.active.bg_stroke = Stroke::new(1.0, p.strong);
    w.open.weak_bg_fill = p.pressed;
    w.open.bg_fill = p.line;
    w.open.bg_stroke = Stroke::new(1.0, p.strong);
}

/// Sizes and type on egui's own widgets: 24-point rows, 13-point text, 14-point dialog titles.
pub fn style(s: &mut egui::Style) {
    s.spacing.interact_size.y = ROW_H;
    s.spacing.button_padding = vec2(8.0, 3.0);
    s.spacing.icon_width = 16.0;
    s.spacing.icon_width_inner = 8.0;
    s.spacing.menu_margin = egui::Margin::same(4);
    s.text_styles.insert(egui::TextStyle::Heading, theme::bold(14.0));
    s.animation_time = 0.0;
}

#[cfg(feature = "pure")]
const INTER: [&[u8]; 2] =
    [include_bytes!("../../../../assets/fonts/Inter-Regular.ttf"), include_bytes!("../../../../assets/fonts/Inter-SemiBold.ttf")];
#[cfg(not(feature = "pure"))]
const INTER: [&[u8]; 2] = [&[], &[]];

/// Inter for all chrome: Regular first in the interface family, SemiBold first in the bold one
/// (the active tab, the title, headings). Upstream's faces stay behind them as fallbacks.
pub fn fonts(defs: &mut FontDefinitions) {
    for (name, bytes, family) in
        [("pure-inter-regular", INTER[0], FontFamily::Proportional), ("pure-inter-semibold", INTER[1], FontFamily::Name("bold".into()))]
    {
        if bytes.is_empty() {
            continue;
        }
        defs.font_data.insert(name.to_string(), std::sync::Arc::new(FontData::from_static(bytes)));
        defs.families.entry(family).or_default().insert(0, name.to_string());
    }
}

// ---------------------------------------------------------------------------------------------
// Icons
// ---------------------------------------------------------------------------------------------

/// The ink an icon is drawn in: Secondary where a caller passes the text colour, Disabled for a
/// disabled icon (at full strength, where upstream fades its ink).
pub fn icon_ink(ink: Color32, disabled: bool) -> Color32 {
    for p in [&super::LIGHT, &super::DARK] {
        if disabled && (ink == p.text || ink == p.secondary) {
            return p.disabled;
        }
        if ink == p.text {
            return p.secondary;
        }
    }
    ink
}

/// An icon's accent colour: upstream's orange, the app's old identity colour, becomes the accent
/// of the palette the `ink` belongs to. The other accents are part of the drawing and stay.
pub fn icon_accent(accent: Color32, ink: Color32) -> Color32 {
    if accent != icons::ORANGE {
        return accent;
    }
    let light_ink = u32::from(ink.r()) + u32::from(ink.g()) + u32::from(ink.b()) > 384;
    paper(light_ink).accent
}

// ---------------------------------------------------------------------------------------------
// Title bar
// ---------------------------------------------------------------------------------------------

/// How a title bar button with a label is drawn.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// The one primary action: filled with the accent.
    Primary,
    /// Flat, with a Line outline.
    Outline,
}

/// A labelled title bar button, one row tall, with a 16-point mark before its label.
fn bar_button(ui: &mut Ui, mark: Option<Icon>, label: &str, tip: &str, kind: Kind, enabled: bool) -> egui::Response {
    let p = paper(Tokens::get(ui.ctx()).dark);
    let font = theme::font(13.0);
    let tw = ui.fonts_mut(|f| f.layout_no_wrap(label.to_string(), font.clone(), p.text).size().x).ceil();
    let w = 10.0 + ICON + 6.0 + tw + 12.0;
    let (rect, resp) = ui.allocate_exact_size(vec2(w, ROW_H), if enabled { Sense::click() } else { Sense::hover() });
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, label));
    let painter = ui.painter_at(rect);
    let r = CornerRadius::same(RADIUS);
    let down = resp.is_pointer_button_down_on();
    let ink = match kind {
        Kind::Primary => {
            painter.rect_filled(rect, r, if enabled && (resp.hovered() || down) { p.accent_hover } else { p.accent });
            p.on_accent
        }
        Kind::Outline => {
            if let Some(c) = widgets::fill(ui, &resp, false, p) {
                painter.rect_filled(rect, r, c);
            }
            painter.rect_stroke(rect, r, Stroke::new(1.0, if resp.hovered() { p.strong } else { p.line }), StrokeKind::Inside);
            if enabled { p.text } else { p.disabled }
        }
    };
    if resp.has_focus() {
        painter.rect_stroke(rect.shrink(2.0), r, Stroke::new(1.0, if kind == Kind::Primary { p.on_accent } else { p.accent }), StrokeKind::Inside);
    }
    let icon_rect = Rect::from_min_size(pos2(rect.min.x + 10.0, rect.center().y - ICON / 2.0), vec2(ICON, ICON));
    match mark {
        Some(icon) => icons::paint(&painter, icon_rect, icon, ink, !enabled),
        // Present: a plain play triangle in the label's colour.
        None => {
            let c = icon_rect.center();
            painter.add(egui::Shape::convex_polygon(vec![pos2(c.x - 3.5, c.y - 5.0), pos2(c.x + 5.0, c.y), pos2(c.x - 3.5, c.y + 5.0)], ink, Stroke::NONE));
        }
    }
    painter.text(pos2(icon_rect.max.x + 6.0, rect.center().y), Align2::LEFT_CENTER, label, font, ink);
    resp.on_hover_text(tip)
}

/// The search box: opens the command search. A box with a Line outline, as wide as it has room.
fn search_box(ui: &mut Ui, width: f32) -> egui::Response {
    let p = paper(Tokens::get(ui.ctx()).dark);
    let (rect, resp) = ui.allocate_exact_size(vec2(width, ROW_H), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, "Search commands"));
    let painter = ui.painter_at(rect);
    let r = CornerRadius::same(RADIUS);
    let outline = if resp.has_focus() {
        p.accent
    } else if resp.hovered() {
        p.strong
    } else {
        p.line
    };
    painter.rect(rect, r, p.panel, Stroke::new(1.0, outline), StrokeKind::Inside);
    icons::paint(&painter, Rect::from_min_size(pos2(rect.min.x + 6.0, rect.center().y - ICON / 2.0), vec2(ICON, ICON)), Icon::Search, p.secondary, false);
    painter.with_clip_rect(rect.shrink2(vec2(6.0, 0.0))).text(
        pos2(rect.min.x + 28.0, rect.center().y),
        Align2::LEFT_CENTER,
        "Search",
        theme::font(12.0),
        p.disabled,
    );
    resp.on_hover_text(format!("Search commands ({})", crate::ribbon::pretty_shortcut("Cmd+Shift+P")))
}

/// The name Share hands the host: the presentation's file name, or its title while it has none.
fn share_name(app: &SlideApp) -> Option<String> {
    let d = app.session.active()?;
    let file = d.path.as_deref().and_then(|p| std::path::Path::new(p).file_name()).map(|n| n.to_string_lossy().to_string());
    Some(file.unwrap_or_else(|| d.title()))
}

/// The title bar, one row: quick access at the left, the presentation's name and whether it is
/// saved in the centre, then search, Comments, Present and (with a host) Share. `left` is where
/// content starts (after the traffic lights on macOS) and `height` the bar's height.
pub fn title_bar(app: &mut SlideApp, ui: &mut Ui, left: f32, height: f32) {
    let t = Tokens::get(ui.ctx());
    let p = paper(t.dark);
    egui::Panel::top("title_bar").exact_size(height).show_separator_line(false).frame(egui::Frame::NONE.fill(p.frame)).show(ui, |ui| {
        let rect = ui.max_rect();
        ui.painter().hline(rect.x_range(), rect.max.y - 0.5, Stroke::new(1.0, p.line));
        // Dragging the empty title bar moves the window, as upstream's does.
        let drag = ui.interact(rect, ui.scope_id().with("titledrag"), Sense::click_and_drag());
        drag.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Window, true, "Window title bar"));
        if drag.drag_started() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        if drag.double_clicked() {
            let max = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
        }
        let has_doc = app.session.active().is_some();
        let mut lead = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(Rect::from_min_max(pos2(rect.min.x + left, rect.min.y), pos2(rect.min.x + left + 320.0, rect.max.y)))
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        lead.spacing_mut().item_spacing.x = 4.0;
        crate::qat::show(app, &mut lead);
        let lead_end = lead.min_rect().max.x;

        let mut trail = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(Rect::from_min_max(pos2(rect.center().x, rect.min.y), pos2(rect.max.x - 8.0, rect.max.y)))
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
        );
        trail.spacing_mut().item_spacing.x = 8.0;
        if let Some(name) = share_name(app).filter(|_| app.services.host_share.is_some())
            && bar_button(&mut trail, Some(Icon::Share), "Share", "Share this presentation", Kind::Outline, true).clicked()
            && let Some(share) = app.services.host_share.as_ref()
        {
            share(&name);
        }
        if bar_button(&mut trail, None, "Present", "Start the slide show from this slide", Kind::Primary, has_doc).clicked() && has_doc {
            let from = app.session.active().map(|d| d.selection.slide).unwrap_or(0);
            app.start_show(from, false);
        }
        if widgets::icon_toggle(&mut trail, Icon::CommentsBubble, "Comments", has_doc, app.ui.pane.as_deref() == Some("comments")).clicked() {
            let _ = app.run("view.pane", json!({"pane": "comments", "toggle": true}));
        }
        let room = (trail.available_width() - 24.0).clamp(0.0, 220.0);
        if room >= 96.0 && search_box(&mut trail, room).clicked() {
            app.palette = Some((String::new(), 0));
        }
        let trail_start = trail.min_rect().min.x;

        // The name (600) and "Saved" or "Edited" (Secondary), centred in the window when there is
        // room and otherwise in the space between the two sides.
        let (name, state) = match app.session.active() {
            Some(d) => (d.title(), if d.is_dirty() { "Edited" } else { "Saved" }),
            None => (NAME.to_string(), ""),
        };
        let name_g = ui.painter().layout_no_wrap(name, theme::bold(14.0), p.text);
        let state_g = ui.painter().layout_no_wrap(state.to_string(), theme::font(14.0), p.secondary);
        let gap = if state.is_empty() { 0.0 } else { 10.0 };
        let w = name_g.size().x + gap + state_g.size().x;
        let free = Rect::from_min_max(pos2(lead_end + 12.0, rect.min.y), pos2(trail_start - 12.0, rect.max.y));
        let x = (rect.center().x - w / 2.0).clamp(free.min.x, (free.max.x - w).max(free.min.x)).round();
        let painter = ui.painter().with_clip_rect(free);
        let y = |h: f32| (rect.center().y - h / 2.0).round();
        painter.galley(pos2(x, y(name_g.size().y)), name_g.clone(), p.text);
        painter.galley(pos2(x + name_g.size().x + gap, y(state_g.size().y)), state_g, p.secondary);
    });
}

// ---------------------------------------------------------------------------------------------
// Ribbon frame
// ---------------------------------------------------------------------------------------------

/// Height of the ribbon panel: the tab strip, the controls with their padding, and the line
/// under them.
pub fn ribbon_height(collapsed: bool) -> f32 {
    TABS_H + if collapsed { 1.0 } else { RIBBON_H + 2.0 * RIBBON_PAD + 1.0 }
}

/// Where the ribbon's controls go inside the panel `rect`, and the line under the panel.
pub fn ribbon_frame(ui: &Ui, rect: Rect) -> Rect {
    let p = paper(Tokens::get(ui.ctx()).dark);
    ui.painter().hline(rect.x_range(), rect.max.y - 0.5, Stroke::new(1.0, p.line));
    Rect::from_min_size(pos2(rect.min.x + 2.0, rect.min.y + TABS_H + RIBBON_PAD), vec2(rect.width() - 4.0, RIBBON_H))
}

/// A ribbon tab: 13-point text, Secondary at rest; the active tab in Text at weight 600 with a
/// 2-point accent rule under it. "File" is a tab like the others.
pub fn paint_tab(ui: &Ui, r: Rect, name: &str, active: bool, resp: &egui::Response) {
    let p = paper(Tokens::get(ui.ctx()).dark);
    let painter = ui.painter();
    if !active {
        let c = if resp.is_pointer_button_down_on() {
            Some(p.pressed)
        } else if resp.hovered() {
            Some(p.hover)
        } else {
            None
        };
        if let Some(c) = c {
            painter.rect_filled(Rect::from_min_max(pos2(r.min.x, r.min.y + 4.0), pos2(r.max.x, r.max.y - 4.0)), CornerRadius::same(RADIUS), c);
        }
    }
    if resp.has_focus() {
        painter.rect_stroke(r.shrink2(vec2(0.0, 4.0)), CornerRadius::same(RADIUS), Stroke::new(1.0, p.accent), StrokeKind::Inside);
    }
    let (font, ink) = if active { (theme::bold(13.0), p.text) } else { (theme::font(13.0), p.secondary) };
    painter.text(r.center(), Align2::CENTER_CENTER, name, font, ink);
    if active {
        painter.rect_filled(Rect::from_min_max(pos2(r.min.x + 12.0, r.max.y - 2.0), pos2(r.max.x - 12.0, r.max.y)), CornerRadius::ZERO, p.accent);
    }
}

/// The chevron that scrolls a ribbon too wide for the window: a flat button the height of the
/// ribbon's controls, set off from them by a rule.
pub fn paint_edge_cue(ui: &Ui, rect: Rect, left: bool, resp: &egui::Response) {
    let p = paper(Tokens::get(ui.ctx()).dark);
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::ZERO, p.panel);
    if let Some(c) = widgets::fill(ui, resp, false, p) {
        painter.rect_filled(rect, CornerRadius::same(RADIUS), c);
    }
    let icon = if left { Icon::ChevronLeft } else { Icon::ChevronRight };
    let min = pos2((rect.center().x - ICON / 2.0).round(), (rect.center().y - ICON / 2.0).round());
    icons::paint(painter, Rect::from_min_size(min, vec2(ICON, ICON)), icon, p.secondary, false);
}

// ---------------------------------------------------------------------------------------------
// Side panes and the status bar
// ---------------------------------------------------------------------------------------------

/// The fill of a side pane (slide thumbnails, notes): a panel, where upstream uses its frame grey.
pub fn pane_fill(t: &Tokens) -> Color32 {
    if super::ON { paper(t.dark).panel } else { t.chrome }
}

/// The line along the top of the status bar.
pub fn status_edge(ui: &Ui, rect: Rect) {
    let p: &Paper = paper(Tokens::get(ui.ctx()).dark);
    ui.painter().hline(rect.x_range(), rect.min.y + 0.5, Stroke::new(1.0, p.line));
}
