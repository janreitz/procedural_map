use nannou::prelude::*;
use nannou::noise::*;
//use nannou::color::*;
use nannou::ui::prelude::*;

struct Tile {
    position: Vector2<f32>,
    size: (f32, f32),
    height: f32,
    under_water: bool,
}

impl Tile {
    fn draw(&self, draw: &Draw) {
        // let col1 = hsv(0.0,0.0,1.0);
        // let col2 = hsv(1.0,1.0,1.0);
        // let col = col1.mix(&col2, self.height);

        draw.rect()
            .x_y(self.position.x, self.position.y)
            .w_h(self.size.0, self.size.1)
            .color( 
                if self.under_water { BLUE }
                else{ GREEN }
            );
    } 
}

struct Model {
    tiles: Vec<Tile>,
    scale: f64,
    offset: Vector2<f64>,
    sea_level: f32,
    ui: Ui,
    ids: Ids,
}

widget_ids! {
    struct Ids {
        scale,
        offset_x,
        offset_y,
        sea_level,
    }
}

fn color_tiles(scale: f64, offset: Vector2<f64>, sea_level: f32) -> Vec<Tile> { 
    let mut tiles = Vec::new();
    let noise = Perlin::new();
    for i in 0..100 {
        for j in 0..100 {
            let pos = vec2(i as f32 * 10.0 - 500.0, j as f32 * 10.0 - 500.0);
            let height = noise.get([
                scale * (pos.x as f64 + offset.x), 
                scale * (pos.y as f64 + offset.y)
                ]) as f32;

            tiles.push(Tile {
                position: pos,
                size: (10.0, 10.0),
                height: height,
                under_water: height < sea_level,
            })
        }
    }
    tiles
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1000,1000).view(view).build().unwrap();

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    // Init our variables
    let scale = 0.01;
    let offset = vec2(0.0, 0.0);
    let sea_level = 0.5;

    Model { 
        tiles: color_tiles(scale, offset, sea_level),
        scale: scale,
        offset: offset,
        sea_level: sea_level,
        ui: ui,
        ids: ids,
    }
}

fn slider_f64(val: f64, min: f64, max: f64) -> widget::Slider<'static, f64> {
    widget::Slider::new(val, min, max)
        .w_h(300.0, 40.0)
        .label_font_size(30)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(0.0, 0.0, 0.0)
        .border(1.0)
}

fn slider_f32(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    widget::Slider::new(val, min, max)
        .w_h(300.0, 40.0)
        .label_font_size(30)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(0.0, 0.0, 0.0)
        .border(1.0)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();

    for value in slider_f64(model.scale, 0.0, 0.01)
        .top_left_with_margin(20.0)
        .label("Scale")
        .set(model.ids.scale, ui)
    {
        model.scale = value;
    }

    for value in slider_f64(model.offset.x, 0.0, 1000.0)
        .down_from(model.ids.scale,20.0)
        .label("Offset X")
        .set(model.ids.offset_x, ui)
        {
            model.offset.x = value;
        }
        
    for value in slider_f64(model.offset.y, 0.0, 1000.0)
        .down_from(model.ids.offset_x,20.0)
        .label("Offset Y")
        .set(model.ids.offset_y, ui)
    {
        model.offset.y = value;
    }

    for value in slider_f32(model.sea_level, -1.0, 1.0)
        .down_from(model.ids.offset_y,20.0)
        .label("Sea Level")
        .set(model.ids.sea_level, ui)
    {
        model.sea_level = value;
    }

    model.tiles = color_tiles(model.scale, model.offset, model.sea_level);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    for tile in model.tiles.iter() {
        tile.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}