extern crate rand;
extern crate orbtk;
extern crate orbimage;
extern crate orbclient;
extern crate orbtk_simple_modal;

#[macro_use]
extern crate lazy_static;

mod board;
mod gamemove;
mod graphics;
mod error;

use orbtk_simple_modal::Modal;

use orbtk::{Window, Rect, Image, Color, Point, Menu, Action};
use orbtk::traits::{Click, Place, Text};

use std::rc::Rc;
use std::cell::RefCell;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() {
    //Set up menu bar
    let menu = Menu::new("Menu");
    menu.position(10, 0).size(32, 16);

    //Create representation of game board
    let board = Rc::new(RefCell::new(board::Board::new()));

    //Track prior click
    let last: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(None));

    //Create game window
    let mut window = Window::new(Rect::new(100, 100, 615, 420), "Charles Village Solitaire");
    let bg =
        Image::from_image(orbimage::parse_png(include_bytes!("../assets/bg.png")).unwrap());
    let canvas = Image::from_color(595, 430, Color::rgba(255, 255, 255, 0));

    //Render initial game state
    graphics::render(&mut *canvas.image.borrow_mut(), &*board.borrow());

    //Main game logic
    {
        let last = last.clone();
        let board = board.clone();   
        canvas.position(10, 16).on_click(move |canvas: &Image, point: Point| {
            let mut last_maybe = last.borrow_mut();                
            let mut board = board.borrow_mut();
            if let Ok(v) = board.get_valid(
                gamemove::Move{
                    src: last_maybe.and_then(|l|gamemove::ClickTarget::from_coord(l.x, l.y)),
                    dst: gamemove::ClickTarget::from_coord(point.x, point.y),
                }) {
                board.make_move(v);
                board.sweep_free();
                *last_maybe = None;
            } else if last_maybe.is_none() {
                *last_maybe = Some(point);
            } 
            else { *last_maybe = None; }
            let canvas = &mut *canvas.image.borrow_mut();
            graphics::render(canvas, &board);
            graphics::render_cursor(canvas, (*last_maybe).as_ref());
        });
    }

    // Modal widgets
    let about_box = Modal::new();
    about_box.text(include_str!("../assets/about.txt"))
        .position(5, 10)
        .size(605, 395);

    let rules_box = Modal::new();
    rules_box.text(include_str!("../assets/rules.txt"))
        .position(5, 10)
        .size(605, 395);
    
    //Menu logic
    {
        let about_box = about_box.clone();
        let rules_box = rules_box.clone();
        let board = board.clone();
        let canvas = canvas.clone();
        let last_ng = last.clone();
        let last_abt = last.clone();
        let last_rls = last.clone();
        
        let new_game = Action::new("New Game");
        new_game.on_click(move |_action: &Action, _point: Point| {
            let mut board = board.borrow_mut();
            *board = board::Board::new();
            graphics::render(&mut *canvas.image.borrow_mut(), &board);
            *last_ng.borrow_mut() = None;
        });
        menu.add(&new_game);

        let about = Action::new("About");
        about.on_click(move |_action: &Action, _point: Point| {
            about_box.visible.set(true);
            *last_abt.borrow_mut() = None;
        });
        menu.add(&about);

        let rules = Action::new("Rules");
        rules.on_click(move |_action: &Action, _point: Point| {
            rules_box.visible.set(true);
            *last_rls.borrow_mut() = None;
        });
        menu.add(&rules);

        let quit = Action::new("Quit");
        quit.on_click(move |_action: &Action, _point: Point| {
            std::process::exit(0);
        });
        menu.add(&quit);
    }

    window.add(&bg);
    window.add(&canvas);
    window.add(&menu);
    window.add(&rules_box);
    window.add(&about_box);
    window.exec();
}
