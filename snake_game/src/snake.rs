use std::collections::LinkedList;

use piston_window::{types::Color, Context, G2d};

use crate::draw::draw_block;

// const SNAKE_SPEED: f64 = 10.0;
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];
#[derive(Clone,Debug)]
struct Block {
    x: i32,
    y: i32,
}
#[derive(Clone,Copy,PartialEq,Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x, y });
        body.push_back(Block { x:x+1, y });
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }
    pub fn head_position(&self) -> Option<(i32, i32)> {
        let head = self.body.front();
        match head {
            Some(block) => {
                let x = block.x;
                let y = block.y;
                Some((x, y))
            }
            None => None,
        }
    }
    pub fn move_forward(&mut self,dir:Option<Direction>){
        match dir{
            Some(d)=>self.direction=d,
            None=>(),
        }
        let (x,y)=self.head_position().unwrap();
        let new_block=match self.direction {
            Direction::Up=>Block{
                x,
                y:y-1
            },
            Direction::Down=>Block{
                x,
                y:y+1
            },Direction::Left=>Block{
                x:x-1,
                y
            },Direction::Right=>Block{
                x:x+1,
                y
            }
        };
        self.body.push_front(new_block);
        let removed_block=self.body.pop_back().unwrap();
        self.tail=Some(removed_block)
        
    }
    pub fn head_direction(&self)->Direction{
        println!("CALLED IN HEAD_DIRECTION FUNCTION {:?}",self.direction);
        self.direction
    }
    pub fn next_head(&self,dir:Option<Direction>)->(i32,i32){
        let (x,y)=self.head_position().unwrap();
        let mut moving=self.direction;
        match dir{
            Some(d)=>moving=d,
            None=>(),
        }
        match moving {
            Direction::Down=>(x,y+1),
            Direction::Left=>(x-1,y),
            Direction::Right=>(x+1,y),
            Direction::Up=>(x,y-1)
        }
    }
    pub fn restore_tail(&mut self){
        let add=self.tail.clone().unwrap();
        self.body.push_back(add);
    }
    pub fn overlap_tail(&self,x: i32,y: i32)->bool{
            let mut ch=0;
            for block in &self.body  {
                if x==block.x && y==block.y{
                    return true;
                }
                ch+=1;
                if ch==self.body.len()-1 {
                    break;
                }
            }
            return false;
    }


}
