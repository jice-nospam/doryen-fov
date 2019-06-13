extern crate doryen_rs;
extern crate rand;


use doryen_fov::{FovAlgorithm, FovRecursiveShadowCasting, FovRestrictive, MapData};
use doryen_rs::{App, AppOptions, Color, DoryenApi, Engine, TextAlign};
use rand::Rng;
const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;
const FOV_NAMES: [&str; 2] = ["recursive shadowcasting", "mrpas (restrictive)"];

const DARK_WALL: Color = (0, 0, 100, 255);
const LIGHT_WALL: Color = (130, 110, 50, 255);
const DARK_GROUND: Color = (50, 50, 150, 255);
const LIGHT_GROUND: Color = (200, 180, 50, 255);
const STARTING_RADIUS: usize = 20;

struct MyRoguelike {
    player_pos: (i32, i32),
    fov: Vec<Box<FovAlgorithm>>,
    map: MapData,
    fov_num: usize,
    radius: usize,
}

impl Engine for MyRoguelike {
    fn init(&mut self, _api: &mut DoryenApi) {}
    fn update(&mut self, api: &mut DoryenApi) {
        let input = api.input();
        if input.key("ArrowLeft") {
            self.player_pos.0 = (self.player_pos.0 - 1).max(1);
        } else if input.key("ArrowRight") {
            self.player_pos.0 = (self.player_pos.0 + 1).min(CONSOLE_WIDTH as i32 - 2);
        }
        if input.key("ArrowUp") {
            self.player_pos.1 = (self.player_pos.1 - 1).max(1);
        } else if input.key("ArrowDown") {
            self.player_pos.1 = (self.player_pos.1 + 1).min(CONSOLE_HEIGHT as i32 - 2);
        }
        if input.key("Equal") || input.key("NumpadAdd") {
            self.radius += 1;
        } else if (input.key("Minus") || input.key("NumpadSubtract")) && self.radius > 0 {
            self.radius -= 1;
        }
        if input.key_pressed("PageUp") {
            self.fov_num += self.fov.len() - 1;
        } else if input.key_pressed("PageDown") {
            self.fov_num += 1;
        }
        self.fov_num %= self.fov.len();
        self.map.clear_fov();
        self.fov[self.fov_num].compute_fov(
            &mut self.map,
            self.player_pos.0 as usize,
            self.player_pos.1 as usize,
            self.radius,
            false,
        );
    }
    fn render(&mut self, api: &mut DoryenApi) {
        let con = api.con();
        con.rectangle(
            0,
            0,
            CONSOLE_WIDTH,
            CONSOLE_HEIGHT,
            Some((128, 128, 128, 255)),
            None,
            Some(' ' as u16),
        );
        for y in 0..CONSOLE_HEIGHT as i32 {
            let off = (y * CONSOLE_WIDTH as i32) as usize;
            for x in 0..CONSOLE_WIDTH as i32 {
                let off = off + x as usize;
                let wall = !self.map.transparent[off];
                let in_fov = self.map.fov[off];
                let back_col = if wall {
                    if in_fov {
                        LIGHT_WALL
                    } else {
                        DARK_WALL
                    }
                } else if in_fov {
                    LIGHT_GROUND
                } else {
                    DARK_GROUND
                };
                con.back(x, y, back_col);
            }
        }
        con.ascii(self.player_pos.0, self.player_pos.1, '@' as u16);
        con.fore(self.player_pos.0, self.player_pos.1, (255, 255, 255, 255));
        con.print(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT - 1) as i32,
            "arrows : move | +/- : change radius | PageUp/PageDown : change algorithm",
            TextAlign::Center,
            Some((255, 255, 255, 255)),
            Some(DARK_WALL),
        );
        con.print(
            (CONSOLE_WIDTH / 2) as i32,
            0,
            FOV_NAMES[self.fov_num],
            TextAlign::Center,
            Some((255, 255, 255, 255)),
            Some(DARK_WALL),
        );
    }
    fn resize(&mut self, _api: &mut DoryenApi) {}
}

impl MyRoguelike {
    pub fn new() -> Self {
        let mut map = MapData::new(CONSOLE_WIDTH as usize, CONSOLE_HEIGHT as usize);
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            map.transparent[rng.gen_range(0, CONSOLE_WIDTH as usize * CONSOLE_HEIGHT as usize)] =
                false;
        }
        let mut fov: Vec<Box<FovAlgorithm>> = Vec::new();
        fov.push(Box::new(FovRecursiveShadowCasting::new()));
        fov.push(Box::new(FovRestrictive::new()));
        let mut me = Self {
            player_pos: ((CONSOLE_WIDTH / 2) as i32, (CONSOLE_HEIGHT / 2) as i32),
            fov,
            map,
            radius: STARTING_RADIUS,
            fov_num: 0,
        };
        me.fov[me.fov_num].compute_fov(
            &mut me.map,
            me.player_pos.0 as usize,
            me.player_pos.1 as usize,
            10,
            false,
        );
        me
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 8,
        screen_height: CONSOLE_HEIGHT * 8,
        window_title: "fov test".to_owned(),
        font_path: "terminal_8x8.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
    });
    app.set_engine(Box::new(MyRoguelike::new()));
    app.run();
}