use piston_window::{Context, G2d, Key};
use rand::{thread_rng, Rng};

use crate::{draw::{draw_block, draw_rectangle}, snake::{Direction, Snake}};
const FOOD_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BORDER_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GAMEOVER_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 0.5];
const MOVING_PERIOD: f64 = 0.12;
const RESTART_TIME: f64 = 1.0;
pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
    score: i32, 
}
impl Game{

    pub fn new(width:i32,height:i32)->Game{
        
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 1.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            score: 0,
        }
    }
    fn check_eating(&mut self){
        let (head_x,head_y)=self.snake.head_position().unwrap();
        if self.food_exists && self.food_x==head_x && self.food_y==head_y {
            self.food_exists=false;
            self.snake.restore_tail();
            self.score+=1;
        }
    }
    pub fn draw(&self,con:&Context,g:&mut G2d){
        self.snake.draw(con,g);
        if self.food_exists{    
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }

    }
    pub fn key_pressend(&mut self,key:Key){
        if self.game_over {
            return;
        }
        let dir=match key {
            Key::Up=>Some(Direction::Up),
            Key::Down=>Some(Direction::Down),
            Key::Left=>Some(Direction::Left),
            Key::Right=>Some(Direction::Right),
            _=>Some(self.snake.head_direction()),
        };
        if let Some(dir) = dir {
            if dir == self.snake.head_direction().opposite() {
                return;
            }
        }
       self.update_snake(dir);
        
    }
    pub fn update(&mut self,time:f64){
        self.waiting_time+=time;
        if self.game_over{
            if self.waiting_time>=RESTART_TIME{
                self.restart()
            }
        return;
        }
        if !self.food_exists{
            self.add_food();
        }

        if self.waiting_time>MOVING_PERIOD {
            self.update_snake(None)
        }
    }
    fn check_if_alive(&self,dir:Option<Direction>)->bool{
        let (x,y)=self.snake.next_head(dir);
        if self.snake.overlap_tail(x, y){
            return false;
        }
        x > 0 && y > 0 && x < self.width - 1 && y < self.height - 1
    }
    fn add_food(&mut self){
        let mut rng=thread_rng();
        let mut new_x=rng.gen_range(1..self.width-1);
        let mut new_y=rng.gen_range(1..self.height-1);
        while self.snake.overlap_tail(new_x, new_y) {
             new_x=rng.gen_range(1..self.width-1);
             new_y=rng.gen_range(1..self.height-1);   
        }
        self.food_x=new_x;
        self.food_y=new_y;
        self.food_exists=true;
    }
    fn update_snake(&mut self,dir:Option<Direction>){
        if self.check_if_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        }else{
            self.game_over=true;
        }
        self.waiting_time=0.0;
    }
    fn restart(&mut self){
        self.snake=Snake::new(2,2);
        self.waiting_time=0.0;
        self.food_exists=true;
        self.food_x= 6;
        self.food_y= 4;
        self.game_over=false;
    }
}
