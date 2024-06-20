use crate::app::Damus;
use crate::ui;
use crate::ui::{Preview, PreviewConfig, View};
use egui::widgets::text_edit::TextEdit;
use nostrdb::Transaction;
use tracing::info;

pub struct PostView<'app, 'p> {
    app: &'app mut Damus,
    /// account index
    poster: usize,
    replying_to: &'p [u8; 32],
}

impl<'app, 'p> PostView<'app, 'p> {
    pub fn new(app: &'app mut Damus, poster: usize, replying_to: &'p [u8; 32]) -> Self {
        PostView {
            app,
            poster,
            replying_to,
        }
    }

    fn editbox(&mut self, txn: &nostrdb::Transaction, ui: &mut egui::Ui) {
        ui.spacing_mut().item_spacing.x = 12.0;

        let pfp_size = 24.0;

        let poster_pubkey = self
            .app
            .account_manager
            .get_account(self.poster)
            .map(|acc| acc.pubkey.bytes())
            .unwrap_or(crate::test_data::test_pubkey());

        // TODO: refactor pfp control to do all of this for us
        let poster_pfp = self
            .app
            .ndb
            .get_profile_by_pubkey(txn, poster_pubkey)
            .as_ref()
            .ok()
            .and_then(|p| {
                Some(ui::ProfilePic::from_profile(&mut self.app.img_cache, p)?.size(pfp_size))
            });

        if let Some(pfp) = poster_pfp {
            ui.add(pfp);
        } else {
            ui.add(
                ui::ProfilePic::new(&mut self.app.img_cache, ui::ProfilePic::no_pfp_url())
                    .size(pfp_size),
            );
        }

        let draft = self
            .app
            .drafts
            .entry(enostr::NoteId::new(*self.replying_to))
            .or_default();

        ui.add(TextEdit::multiline(&mut draft.buffer).frame(false));
    }

    pub fn ui(&mut self, txn: &nostrdb::Transaction, ui: &mut egui::Ui) {
        egui::Frame::default()
            .inner_margin(egui::Margin::same(12.0))
            .outer_margin(egui::Margin::same(12.0))
            .fill(ui.visuals().extreme_bg_color)
            .stroke(ui.visuals().noninteractive().bg_stroke)
            .rounding(12.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        self.editbox(txn, ui);
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui
                            .add_sized([91.0, 32.0], egui::Button::new("Post now"))
                            .clicked()
                        {
                            info!("Post clicked");
                        }
                    });
                });
            });
    }
}

mod preview {
    use super::*;
    use crate::test_data;

    pub struct PostPreview {
        app: Damus,
    }

    impl PostPreview {
        fn new(is_mobile: bool) -> Self {
            PostPreview {
                app: test_data::test_app(is_mobile),
            }
        }
    }

    impl View for PostPreview {
        fn ui(&mut self, ui: &mut egui::Ui) {
            let test_note_id = test_data::test_pubkey();
            let txn = Transaction::new(&self.app.ndb).unwrap();
            PostView::new(&mut self.app, 0, test_note_id).ui(&txn, ui);
        }
    }

    impl<'app, 'p> Preview for PostView<'app, 'p> {
        type Prev = PostPreview;

        fn preview(cfg: PreviewConfig) -> Self::Prev {
            PostPreview::new(cfg.is_mobile)
        }
    }
}