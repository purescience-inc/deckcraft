//! The Pure edition: everything that differs from upstream DeckCraft when the cargo feature
//! `pure` is on. Upstream files only carry short guards (`if crate::pure::ON { … }`), so both
//! sides always compile and the default build stays upstream's app.
//!
//! What lives here: the visible names, the "Paper" palette and its mapping onto
//! [`Tokens`](crate::theme::Tokens), the sizes of the chrome, and the pieces of chrome that are
//! drawn differently (title bar, ribbon frame, File screen rows).

use egui::Color32;

use crate::theme::Tokens;

pub mod chrome;
pub mod widgets;

/// Whether this build is the Pure edition.
pub const ON: bool = cfg!(feature = "pure");

/// The name shown wherever upstream shows its own.
pub const NAME: &str = "PureSlides (Craft)";

/// The product this edition is built on. Used in the About line and nowhere else.
pub const UPSTREAM: &str = "DeckCraft";

/// The product name for visible text: ours in the Pure edition, upstream's otherwise.
pub const fn app_name() -> &'static str {
    if ON { NAME } else { UPSTREAM }
}

/// "PureSlides (Craft) — built on DeckCraft 0.18.1 (MIT or Apache-2.0), © its contributors".
pub fn about_line() -> String {
    format!("{NAME} — built on {UPSTREAM} {} (MIT or Apache-2.0), © its contributors", env!("CARGO_PKG_VERSION"))
}

// ---------------------------------------------------------------------------------------------
// Palette
// ---------------------------------------------------------------------------------------------

/// The "Paper" palette, one value per role.
#[derive(Clone, Copy, Debug)]
pub struct Paper {
    /// Title bar and status bar.
    pub frame: Color32,
    /// Ribbon, tab strip, side panes, the File screen.
    pub panel: Color32,
    /// Menus, popups and dialogs.
    pub popup: Color32,
    /// The ground behind the slide.
    pub canvas: Color32,
    /// Dividers and group separators.
    pub rule: Color32,
    /// Control outlines and bar edges.
    pub line: Color32,
    /// Input focus and emphasis.
    pub strong: Color32,
    pub text: Color32,
    /// Secondary text and icons.
    pub secondary: Color32,
    pub disabled: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    /// A toggled command or the chosen item: its fill and its mark.
    pub on_fill: Color32,
    pub on_mark: Color32,
    /// The one primary action, the active tab's rule, focus, links.
    pub accent: Color32,
    pub accent_hover: Color32,
    pub on_accent: Color32,
    /// Selected text in chrome text fields.
    pub selection: Color32,
    pub danger: Color32,
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

pub const LIGHT: Paper = Paper {
    frame: rgb(0xf7f8f9),
    panel: rgb(0xffffff),
    popup: rgb(0xffffff),
    canvas: rgb(0xe9ebee),
    rule: rgb(0xe3e5e9),
    line: rgb(0xcdd1d7),
    strong: rgb(0xaab0b9),
    text: rgb(0x1f2328),
    secondary: rgb(0x4f5661),
    disabled: rgb(0x9aa1ab),
    hover: rgb(0xf0f2f4),
    pressed: rgb(0xe3e5e9),
    on_fill: rgb(0xe9eff9),
    on_mark: rgb(0x1f3a5f),
    accent: rgb(0x1f3a5f),
    accent_hover: rgb(0x172d4b),
    on_accent: rgb(0xffffff),
    selection: rgb(0xcfdcee),
    danger: rgb(0xa8291f),
};

pub const DARK: Paper = Paper {
    frame: rgb(0x1b1f23),
    panel: rgb(0x1f2328),
    popup: rgb(0x262b31),
    canvas: rgb(0x16191d),
    rule: rgb(0x2d3238),
    line: rgb(0x3a4047),
    strong: rgb(0x555c65),
    text: rgb(0xe7e9ec),
    secondary: rgb(0xb3b9c2),
    disabled: rgb(0x6b727c),
    hover: rgb(0x2a2f35),
    pressed: rgb(0x32383f),
    on_fill: rgb(0x26364b),
    on_mark: rgb(0x9dbbe4),
    accent: rgb(0x9dbbe4),
    accent_hover: rgb(0xb7cdec),
    on_accent: rgb(0x16191d),
    selection: rgb(0x34506f),
    danger: rgb(0xf0968d),
};

pub const fn paper(dark: bool) -> &'static Paper {
    if dark { &DARK } else { &LIGHT }
}

/// The palette mapped onto upstream's tokens. `base` is upstream's set for the same brightness:
/// the tokens that belong to the slide canvas (selection frame, handles, guides, caret, the
/// selection tint inside slide text) are taken from it unchanged.
pub fn tokens(base: Tokens) -> Tokens {
    let p = paper(base.dark);
    Tokens {
        dark: base.dark,
        chrome: p.frame,
        card: p.panel,
        card_border: p.line,
        text_field: p.panel,
        canvas: p.canvas,
        text: p.text,
        text_dim: p.secondary,
        text_faint: p.disabled,
        border: p.line,
        separator: p.rule,
        hover: p.hover,
        pressed: p.pressed,
        selected: p.on_fill,
        accent: p.accent,
        accent_text: p.on_accent,
        contextual: p.accent,
        field_bg: p.panel,
        control: p.rule,
        control_hover: p.line,
        control_pressed: p.strong,
        shadow: Color32::TRANSPARENT,
        ..base
    }
}

// ---------------------------------------------------------------------------------------------
// Sizes
// ---------------------------------------------------------------------------------------------

/// Corner radius of buttons, inputs, menus and dialogs. Bars and panels have none.
pub const RADIUS: u8 = 3;
/// Height of one row of ribbon controls. A large button is three rows tall.
pub const ROW_H: f32 = 24.0;
/// Small ribbon icons, and every title bar and status bar icon.
pub const ICON: f32 = 16.0;
/// Large ribbon icons.
pub const ICON_LARGE: f32 = 24.0;

// ---------------------------------------------------------------------------------------------
// What the Pure edition leaves out
// ---------------------------------------------------------------------------------------------

/// A dialog or menu title with upstream's product name replaced by ours.
pub fn title(upstream: &'static str) -> &'static str {
    if !ON {
        return upstream;
    }
    match upstream {
        "DeckCraft" => NAME,
        "About DeckCraft" => "About PureSlides (Craft)",
        other => other,
    }
}

/// Commands the command search does not list: the host opens, closes and keeps the files, picks
/// light or dark, and updates the app; community links are not shown. The commands themselves
/// still run when called by id.
const UNLISTED: &[&str] = &[
    "app.links",
    "app.checkUpdates",
    "app.home",
    "app.openDialog",
    "app.saveAsDialog",
    "app.closePresentation",
    "app.browseThemeDialog",
    "view.dark",
    "file.new",
    "file.open",
    "file.close",
    "file.saveAs",
    "file.recovery.open",
];

/// The command search's entries `(id, label, shortcut)` as the Pure edition lists them.
pub fn palette(items: &mut Vec<(String, String, Option<&'static str>)>) {
    items.retain(|(id, ..)| !UNLISTED.contains(&id.as_str()));
    for (_, label, _) in items.iter_mut() {
        if label == "About DeckCraft" {
            *label = title("About DeckCraft").to_string();
        }
    }
}

/// The quick access toolbar: Save, Undo, Redo.
pub fn qat_defaults() -> Vec<crate::qat::QatItem> {
    use crate::icons::Icon;
    use crate::qat::QatItem;
    vec![QatItem::new("file.save", "Save", Icon::Save), QatItem::new("edit.undo", "Undo", Icon::Undo), QatItem::new("edit.redo", "Redo", Icon::Redo)]
}

/// The File screen's side list. The host opens, closes and keeps the files, so Home, Open, Save
/// As and Close are left out; New is offered only when nothing is open, where it cannot leave the
/// host's tab pointing at another presentation.
pub fn file_entries(has_doc: bool) -> (Vec<crate::backstage::Nav>, Vec<crate::backstage::Nav>) {
    use crate::backstage::{Nav, Page};
    let top =
        if has_doc { vec![Nav::Page(Page::Info), Nav::Save, Nav::Page(Page::Print), Nav::Page(Page::Export)] } else { vec![Nav::Page(Page::New)] };
    (top, vec![Nav::Page(Page::About), Nav::Options])
}

/// The page shown when `page` is asked for: a page the side list does not have gives way to the
/// first one it has.
pub fn file_page(page: crate::backstage::Page, has_doc: bool) -> crate::backstage::Page {
    use crate::backstage::{Nav, Page};
    let (top, bottom) = file_entries(has_doc);
    if top.iter().chain(&bottom).any(|n| *n == Nav::Page(page)) {
        return page;
    }
    if has_doc { Page::Info } else { Page::New }
}

/// The About text, shared by the About dialog and File › About: our name, what it is built on,
/// and upstream's own licence and copyright line, unchanged. No links.
pub fn about_text(ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    ui.label(egui::RichText::new(about_line()).color(t.text));
    ui.label(egui::RichText::new(crate::dialogs::version_line()).color(t.text_dim));
    ui.add_space(8.0);
    ui.label(egui::RichText::new("DeckCraft is an open-source, clean-room project. Not affiliated with Microsoft.").color(t.text_dim));
    ui.label(egui::RichText::new("MIT OR Apache-2.0 · © 2026 ArtCraft Team and the DeckCraft contributors").color(t.text_dim));
}

/// The About dialog's first tab. Contributors and Models stay on the tabs beside it.
pub fn about(ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(4.0);
    ui.label(egui::RichText::new(NAME).font(crate::theme::bold(14.0)).color(t.text));
    ui.add_space(4.0);
    about_text(ui);
    ui.add_space(4.0);
}

/// File › About: the About text and the way to the contributor credits upstream compiled in.
/// Light and dark follow the host, so there is no appearance choice here.
pub fn about_page(app: &mut crate::SlideApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    ui.label(egui::RichText::new(NAME).font(crate::theme::bold(14.0)).color(t.text));
    ui.add_space(4.0);
    about_text(ui);
    ui.add_space(16.0);
    if ui.button("Credits…").on_hover_text("The people and models who built DeckCraft").clicked() {
        let _ = app.run("app.about", serde_json::json!({}));
    }
}

// ---------------------------------------------------------------- dialogs

/// A dialog's header: the title at the leading edge, a close button at the far edge, one rule
/// under them. Upstream's title bar centres the title in a grey band.
pub const DIALOG_HEADER: f32 = 44.0;

/// The space around a dialog's content.
pub const DIALOG_MARGIN: i8 = 16;

/// A dialog's OK and Cancel buttons at their least.
pub const DIALOG_BUTTON: egui::Vec2 = egui::vec2(76.0, 28.0);

/// A dialog's frame in the Pure edition: white (the popup colour in dark), one line, 3px corners,
/// its margin, and room at the top for the header [`dialog_header`] paints after it is shown.
pub fn dialog_frame(style: &egui::Style) -> egui::Frame {
    let m = DIALOG_MARGIN;
    egui::Frame::window(style).fill(paper(style.visuals.dark_mode).popup).inner_margin(egui::Margin { left: m, right: m, top: DIALOG_HEADER as i8 + 12, bottom: m })
}

/// Inside a dialog's body: 24px control rows on the 4px step, and room between them.
pub fn dialog_spacing(ui: &mut egui::Ui) {
    let s = ui.spacing_mut();
    s.interact_size.y = ROW_H;
    s.item_spacing = egui::vec2(8.0, 6.0);
    s.button_padding = egui::vec2(10.0, 4.0);
}

/// Paints the header of the dialog `window` (the response its `Window::show` returned) on its
/// layer and handles its close button. True when the close button was pressed.
pub fn dialog_header(ctx: &egui::Context, window: &egui::Response, title: &str) -> bool {
    let style = ctx.global_style();
    let p = paper(style.visuals.dark_mode);
    let rtl = style.right_to_left;
    let stroke = style.visuals.window_stroke.width;
    let rect = egui::Rect::from_min_size(window.rect.min + egui::vec2(stroke, stroke), egui::vec2(window.rect.width() - 2.0 * stroke, DIALOG_HEADER));
    let id = window.layer_id.id.with("pure_dialog_header");
    let ui = egui::Ui::new(ctx.clone(), id, egui::UiBuilder::new().layer_id(window.layer_id).max_rect(rect));
    let painter = ctx.layer_painter(window.layer_id);
    let galley = painter.layout_no_wrap(title.to_string(), crate::theme::bold(14.0), p.text);
    let inset = f32::from(DIALOG_MARGIN);
    let x = if rtl { rect.max.x - inset - galley.size().x } else { rect.min.x + inset };
    painter.galley(egui::pos2(x, rect.center().y - galley.size().y / 2.0), galley, p.text);
    let side = 28.0;
    let cx = if rtl { rect.min.x + 8.0 + side / 2.0 } else { rect.max.x - 8.0 - side / 2.0 };
    let close = egui::Rect::from_center_size(egui::pos2(cx, rect.center().y), egui::vec2(side, side));
    let r = ui.interact(close, id.with("close"), egui::Sense::click());
    r.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, "Close"));
    let r = r.on_hover_text("Close");
    let corner = egui::CornerRadius::same(RADIUS);
    if r.is_pointer_button_down_on() {
        painter.rect_filled(close, corner, p.pressed);
    } else if r.hovered() {
        painter.rect_filled(close, corner, p.hover);
    }
    let c = egui::pos2(painter.round_to_pixel_center(close.center().x), painter.round_to_pixel_center(close.center().y));
    let s = egui::Stroke::new(1.25, if r.hovered() { p.text } else { p.secondary });
    let g = 5.0;
    painter.line_segment([c + egui::vec2(-g, -g), c + egui::vec2(g, g)], s);
    painter.line_segment([c + egui::vec2(-g, g), c + egui::vec2(g, -g)], s);
    let y = painter.round_to_pixel_center(rect.max.y - 0.5);
    painter.line_segment([egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)], egui::Stroke::new(1.0, p.rule));
    r.clicked()
}

/// Before a dialog's OK and Cancel: room, and one rule the full width of the dialog.
pub fn dialog_footer_rule(ui: &mut egui::Ui) {
    let p = paper(ui.visuals().dark_mode);
    ui.add_space(12.0);
    let inset = f32::from(DIALOG_MARGIN);
    let r = ui.max_rect();
    let y = ui.painter().round_to_pixel_center(ui.cursor().min.y);
    ui.painter().line_segment([egui::pos2(r.min.x - inset, y), egui::pos2(r.max.x + inset, y)], egui::Stroke::new(1.0, p.rule));
    ui.add_space(12.0);
}

/// A row of tabs in a dialog: 13px labels, the chosen one in the text colour, bold, on a 2px accent
/// rule; the others in the secondary colour with a quiet fill under the pointer; one rule under the
/// row. Upstream shows them as selectable chips. Returns the tab clicked.
pub fn dialog_tabs(ui: &mut egui::Ui, labels: &[&str], selected: usize) -> Option<usize> {
    const H: f32 = 30.0;
    const PAD: f32 = 12.0;
    let p = paper(ui.visuals().dark_mode);
    let font = |on: bool| if on { crate::theme::bold(13.0) } else { crate::theme::font(13.0) };
    let need: f32 = labels.iter().map(|l| ui.painter().layout_no_wrap(l.to_string(), font(true), p.text).size().x + 2.0 * PAD).sum();
    ui.set_min_width(need);
    let origin = ui.cursor().min;
    let width = ui.available_width();
    let rtl = ui.style().right_to_left;
    let mut x = if rtl { origin.x + width } else { origin.x };
    let mut clicked = None;
    for (i, label) in labels.iter().enumerate() {
        let on = i == selected;
        let w = ui.painter().layout_no_wrap(label.to_string(), font(true), p.text).size().x + 2.0 * PAD;
        let left = if rtl { x - w } else { x };
        let r = egui::Rect::from_min_size(egui::pos2(left, origin.y), egui::vec2(w, H));
        let resp = ui.interact(r, ui.id().with(("pure_dialog_tab", i)), egui::Sense::click());
        resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, true, on, *label));
        let painter = ui.painter();
        if !on && resp.hovered() {
            painter.rect_filled(r.shrink2(egui::vec2(0.0, 3.0)), egui::CornerRadius::same(RADIUS), p.hover);
        }
        let ink = if on || resp.hovered() { p.text } else { p.secondary };
        let galley = painter.layout_no_wrap(label.to_string(), font(on), ink);
        painter.galley(egui::pos2(r.center().x - galley.size().x / 2.0, r.center().y - galley.size().y / 2.0 - 1.0), galley, ink);
        if on {
            painter.rect_filled(egui::Rect::from_min_max(egui::pos2(r.min.x + 4.0, r.max.y - 2.0), egui::pos2(r.max.x - 4.0, r.max.y)), 0.0, p.accent);
        }
        if resp.clicked() && !on {
            clicked = Some(i);
        }
        x += if rtl { -w } else { w };
    }
    ui.allocate_space(egui::vec2(width, H));
    let y = ui.painter().round_to_pixel_center(origin.y + H - 0.5);
    ui.painter().line_segment([egui::pos2(origin.x, y), egui::pos2(origin.x + width, y)], egui::Stroke::new(1.0, p.rule));
    ui.add_space(4.0);
    clicked
}
