extern crate sdl2;
extern crate rand;


use rand::prelude::*;
use rand::rngs::StdRng;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::*;
use sdl2::keyboard::Keycode;
use std::time::Duration;




const SIZE:i32=300;
const PSIZE:u32=2;
const USIZE:usize = SIZE as usize;
const WIDTH:u32 = PSIZE * (SIZE as u32);

// this is the grid where the game takes place. It's a 2D array of bools.
// The semicolon means "repeat", so this is USIZE bools, USIZE times.
struct Grid {
    grid: [[bool;USIZE];USIZE],
}

// the implementation of the grid - Rust puts struct (class) methods
// outside the struct.
impl Grid {
    // the constructor, which just makes a grid and fills it with
    // false. Again, the ; means repeat so this is false USIZE times,
    // USIZE times.
    fn new() -> Grid {
        // this function is just a single expression (no semicolon at
        // the end) - the expression Type {k:v, k:v,} is how you
        // instantiate a struct.
        Grid { 
            grid: [[false;USIZE];USIZE],
        }
    }
    
    // count neighbours. This takes a non-mutable reference to self,
    // because it just counts. The count variable has to be mutable.
    fn neighbours(&self,x:i32,y:i32) -> u32 {
        let mut ct:u32 = 0;
        // The a..=b notation is an inclusive range; exclusive would
        // be a..b
        for xx in x-1..=x+1 {
            for yy in y-1..=y+1 {
                if (xx!=x || yy!=y) && self.get(xx,yy){ ct+=1; }
            }
        }
        // the function ends with an expression (note the lack of a
        // semicolon) - means we return the value
        ct
    }
    
    // this is straightforward, with multidimensional arrays being
    // accessed as in C. Note the heavy typecasting with "as": Rust
    // is very strictly typed.
    fn get(&self,x:i32,y:i32)->bool {
        self.grid[((x+SIZE)%SIZE) as usize][((y+SIZE)%SIZE) as usize]
    }
    fn set(&mut self,x:i32,y:i32,v:bool) {
        self.grid[((x+SIZE)%SIZE) as usize][((y+SIZE)%SIZE) as usize] = v;
    }
    
    // a simple printing show.
    fn render(&self, canvas: &mut Canvas<Window>){
        canvas.set_draw_color(Color::RGB(255,255,0));
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.get(x,y){
                    canvas.fill_rect(Rect::new(x*PSIZE as i32,
                                               y*PSIZE as i32,
                                               PSIZE,PSIZE)).ok();
                }
            }
        }
    }
    
}

// run a generation of Life, using neighbours() (which reads the front
// buffer) and set() (which writes the back buffer). Note the heavy
// use of match expressions.

fn gen(g: Grid) -> Grid {
    let mut ng = Grid::new();
    for x in 0..SIZE {
        for y in 0..SIZE {
            let n = g.neighbours(x,y); // get neighbour count of cell
            if g.get(x,y) { // if it's alive
                ng.set(x,y, // set to..
                         match n { 
                         2|3 => true, // for 2/3 cells, alive
                         _ => false // otherwise dead
                     });
            } else {
                ng.set(x,y, // if it's dead, set to..
                         match n {
                         3 => true, // for 3 cells, alive
                         _ => false // otherwise dead
                     });
            }
        }
    }
    ng
}

fn main() {
    let mut g = Grid::new();
    // create a new standard PRNG, using OS entropy to get a 
    // new seed every time
    let mut r = StdRng::from_entropy();
    
    let sdl_c = sdl2::init().unwrap();
    let video = sdl_c.video().unwrap();
    let window = video.window("Foo",WIDTH,WIDTH)
          .position_centered()
          .build().unwrap();
    let mut canvas = window.into_canvas()
          .target_texture()
          .present_vsync()
          .build().unwrap();
    
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    canvas.present();
    
    let mut event_pump = sdl_c.event_pump().unwrap();
    
    
    for _ in 0..10000 {
        // get random number in range. We have to specify the type
        // so we know which implementation of gen_range() to use.
        let x:i32 = r.gen_range(0,SIZE);
        let y:i32 = r.gen_range(0,SIZE);
        g.set(x,y,true);
    }
    
    // run some generations - swap, gen, show.
    'mainloop: loop {
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape),..}|
                      Event::KeyDown {keycode: Some(Keycode::Q),..}
                => {
                    break 'mainloop;
                },
                _ => {}
             }
        }

        g = gen(g);
        g.render(&mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }    
    
}


