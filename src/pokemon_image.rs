use eframe::egui::{Image, Response, Sense, TextureId, Ui, Vec2, Widget};

pub struct PokemonImage {
    image: Option<Image>,
    sense: Sense,
}

impl PokemonImage {
    pub fn new(texture_id: Option<TextureId>, size: impl Into<Vec2>) -> Self {
        let image = if let Some(texture_id) = texture_id {
            Some(Image::new(texture_id, size))
        } else {
            None
        };
        Self {
            image,
            sense: Sense::hover(),
        }
    }
}

impl Widget for PokemonImage {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self { image, sense } = self;

        let rect_size = Vec2::new(50.0, 50.0);
        let (rect, response) = ui.allocate_exact_size(rect_size, sense);

        let visuals = ui.style().noninteractive();

        let mut stroke = visuals.fg_stroke;
        stroke.width = 0.2;

        if ui.is_rect_visible(rect) {
            ui.painter().rect(rect, 0.0, visuals.bg_fill, stroke);
            if let Some(image) = image {
                let image_rect = ui.layout().align_size_within_rect(image.size(), rect);
                image.paint_at(ui, image_rect);
            }
        }

        response
    }
}
