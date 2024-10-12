use piston_window::{rectangle,Context,G2d};
use piston_window::types::Color;

const BLOCK_SIZE:f64=25.0;

pub fn coordination(game_cord:i32)->f64{
    (game_cord as f64) * BLOCK_SIZE
}
pub fn coordination_u32(game_cord:i32)->u32{
    coordination(game_cord) as u32
}
pub fn draw_block(color:Color,x:i32,y:i32,con:&Context,g:&mut G2d){
    let gui_x=coordination(x);
    let gui_y=coordination(y);
    rectangle(
        color,[gui_x,gui_y,BLOCK_SIZE,BLOCK_SIZE],con.transform,g,
    )
}
pub fn draw_rectangle(color:Color,x:i32,y:i32,width:i32,height:i32,con:&Context,g:&mut G2d){
    let gui_x=coordination(x);
    let gui_y=coordination(y);
    rectangle(
        color,[gui_x,gui_y,BLOCK_SIZE*(width as f64),BLOCK_SIZE*(height as f64)],con.transform,g,
    )
}