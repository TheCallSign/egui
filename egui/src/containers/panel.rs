//! Panels are fixed [`Ui`] regions.
//!
//! Together with [`Window`] and [`Area`]:s they are
//! the only places where you can put you widgets.
//!
//! The order in which you add panels matter!
//!
//! Add [`CentralPanel`] and [`Window`]:s last.

use crate::*;

// ----------------------------------------------------------------------------

/// A panel that covers the entire left side of the screen.
///
/// `SidePanel`s must be added before adding any [`CentralPanel`] or [`Window`]s.
///
/// ```
/// # let mut ctx = egui::CtxRef::default();
/// # ctx.begin_frame(Default::default());
/// # let ctx = &ctx;
/// egui::SidePanel::left("my_side_panel", 0.0).show(ctx, |ui| {
///    ui.label("Hello World!");
/// });
/// ```
#[must_use = "You should call .show()"]
pub struct SidePanel {
    id: Id,
    max_width: f32,
    frame: Option<Frame>,
}

impl SidePanel {
    /// `id_source`: Something unique, e.g. `"my_side_panel"`.
    /// The given `max_width` is a soft maximum (as always), and the actual panel may be smaller or larger.
    pub fn left(id_source: impl std::hash::Hash, max_width: f32) -> Self {
        Self {
            id: Id::new(id_source),
            max_width,
            frame: None,
        }
    }

    /// Change the background color, margins, etc.
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = Some(frame);
        self
    }
}

impl SidePanel {
    pub fn show<R>(
        self,
        ctx: &CtxRef,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let Self {
            id,
            max_width,
            frame,
        } = self;

        let mut panel_rect = ctx.available_rect();
        panel_rect.max.x = panel_rect.max.x.at_most(panel_rect.min.x + max_width);

        let layer_id = LayerId::background();

        let clip_rect = ctx.input().screen_rect();
        let mut panel_ui = Ui::new(ctx.clone(), layer_id, id, panel_rect, clip_rect);

        let frame = frame.unwrap_or_else(|| Frame::side_top_panel(&ctx.style()));
        let inner_response = frame.show(&mut panel_ui, |ui| {
            ui.set_min_height(ui.max_rect_finite().height()); // Make sure the frame fills the full height
            add_contents(ui)
        });

        // Only inform ctx about what we actually used, so we can shrink the native window to fit.
        ctx.frame_state()
            .allocate_left_panel(inner_response.response.rect);

        inner_response
    }
}

// ----------------------------------------------------------------------------

/// A panel that covers the entire top side of the screen.
///
/// `TopPanel`s must be added before adding any [`CentralPanel`] or [`Window`]s.
///
/// ```
/// # let mut ctx = egui::CtxRef::default();
/// # ctx.begin_frame(Default::default());
/// # let ctx = &ctx;
/// egui::TopPanel::top("my_top_panel").show(ctx, |ui| {
///    ui.label("Hello World!");
/// });
/// ```
#[must_use = "You should call .show()"]
pub struct TopPanel {
    id: Id,
    max_height: Option<f32>,
    frame: Option<Frame>,
}

impl TopPanel {
    /// `id_source`: Something unique, e.g. `"my_top_panel"`.
    /// Default height is that of `interact_size.y` (i.e. a button),
    /// but the panel will expand as needed.
    pub fn top(id_source: impl std::hash::Hash) -> Self {
        Self {
            id: Id::new(id_source),
            max_height: None,
            frame: None,
        }
    }

    /// Change the background color, margins, etc.
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = Some(frame);
        self
    }
}

impl TopPanel {
    pub fn show<R>(
        self,
        ctx: &CtxRef,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let Self {
            id,
            max_height,
            frame,
        } = self;
        let max_height = max_height.unwrap_or_else(|| ctx.style().spacing.interact_size.y);

        let mut panel_rect = ctx.available_rect();
        panel_rect.max.y = panel_rect.max.y.at_most(panel_rect.min.y + max_height);

        let layer_id = LayerId::background();

        let clip_rect = ctx.input().screen_rect();
        let mut panel_ui = Ui::new(ctx.clone(), layer_id, id, panel_rect, clip_rect);

        let frame = frame.unwrap_or_else(|| Frame::side_top_panel(&ctx.style()));
        let inner_response = frame.show(&mut panel_ui, |ui| {
            ui.set_min_width(ui.max_rect_finite().width()); // Make the frame fill full width
            add_contents(ui)
        });

        // Only inform ctx about what we actually used, so we can shrink the native window to fit.
        ctx.frame_state()
            .allocate_top_panel(inner_response.response.rect);

        inner_response
    }
}

// ----------------------------------------------------------------------------

/// A panel that covers the remainder of the screen,
/// i.e. whatever area is left after adding other panels.
///
/// `CentralPanel` must be added after all other panels.
/// Any [`Window`]s and [`Area`]s will cover the `CentralPanel`.
///
/// ```
/// # let mut ctx = egui::CtxRef::default();
/// # ctx.begin_frame(Default::default());
/// # let ctx = &ctx;
/// egui::CentralPanel::default().show(ctx, |ui| {
///    ui.label("Hello World!");
/// });
/// ```
#[must_use = "You should call .show()"]
#[derive(Default)]
pub struct CentralPanel {
    frame: Option<Frame>,
}

impl CentralPanel {
    /// Change the background color, margins, etc.
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = Some(frame);
        self
    }
}

impl CentralPanel {
    pub fn show<R>(
        self,
        ctx: &CtxRef,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let Self { frame } = self;

        let panel_rect = ctx.available_rect();

        let layer_id = LayerId::background();
        let id = Id::new("central_panel");

        let clip_rect = ctx.input().screen_rect();
        let mut panel_ui = Ui::new(ctx.clone(), layer_id, id, panel_rect, clip_rect);

        let frame = frame.unwrap_or_else(|| Frame::central_panel(&ctx.style()));
        let inner_response = frame.show(&mut panel_ui, |ui| {
            ui.expand_to_include_rect(ui.max_rect()); // Expand frame to include it all
            add_contents(ui)
        });

        // Only inform ctx about what we actually used, so we can shrink the native window to fit.
        ctx.frame_state()
            .allocate_central_panel(inner_response.response.rect);

        inner_response
    }
}
