use rand::RngExt;
use std::io::{stdout,Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};
use crossterm::{execute,cursor,terminal::{Clear,ClearType}};
// Just a location on the board
#[derive(PartialEq,Copy,Clone)]
struct Point    {
    x: u16,
    y: u16,
}
// set of points where the snake is
struct Snake    {
    body: Vec<Point>,
}
// point where the food is
struct Food {
    location: Point,
}
// cardinal directions to update state of game

#[derive(PartialEq,Copy,Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
// The game
struct Game {
    snake:Snake,
    food:Food,
    direction:Direction ,
    width:u16,
    height:u16,
    score:u16,
    gameover:bool

}



fn main() {
    // init the terminal
    crossterm::terminal::enable_raw_mode().unwrap();
    // clear terminal before drawing the game 
    execute!(
        stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0,0))
        .unwrap();
   // println!("Hello, world!");
   let mut game= Game::new_game(); 
   //Main game loop
   while !game.gameover {
        game.handle_input();
        game.game_update();
        game.draw();
        sleep(Duration::from_millis(130));
   }
   println!("Game Over! Your Score Was {}", game.score);
   // game.game_over_screen();
}

// Function to calculate the next step for the snake.
fn calc_next_step(direction: Direction, snake_head:Point,width:u16,height:u16) ->Point{
   // learning the match format for direction enum 
    match direction {
    Direction::Down => {
    if snake_head.y == height - 1{
        Point {
            x:snake_head.x,
            y:0,
        }
    } else {
        Point {
            x:snake_head.x,
            y: snake_head.y + 1 ,
        }
    }

    }
    Direction::Up => {
        if snake_head.y == 0 {
            Point {
                x:snake_head.x,
                y:height - 1 ,
            }
        } else {
            Point {
            x:snake_head.x,
            y:snake_head.y - 1,
            }
        }
    }
    Direction::Left =>{
        if snake_head.x == 0 {
            Point {
                x:width-1,
                y:snake_head.y,
            }
        } else{
            Point{
                x:snake_head.x-1,
                y:snake_head.y,
            }
        }
    }
    Direction::Right =>{
    if snake_head.x == width - 1 {
        Point {
            x:0,
            y:snake_head.y,
        }
    } else {
        Point{
            x:snake_head.x+1,
            y:snake_head.y,
        }
    }

    }

    }
}
//checks if the snake is at the food's location
fn iseating(new_pos:Point,food:&Food) -> bool {
    new_pos == food.location
}
// checks if pos is anywhere inside the snake
fn iscannibal(pos:&Point,snake:&Snake) -> bool {
    //let mut collide = false;
    snake.body.iter().any(|p| p == pos)
    
}
// implicit definition on food: gen_food places a random snack
impl Food {
    fn gen_food(&mut self,snake:&Snake,width:u16,height:u16) {
        let mut rng = rand::rng();
        let mut new_pos = Point{
            x:rng.random_range(0..width),
            y:rng.random_range(0..height),
        };
        while iscannibal(&new_pos, snake) {
            // shoulda called it something else but iscannibal is fun
            new_pos = Point {
                x: rng.random_range(0..width),
                y: rng.random_range(0..height),
            };
        }
        self.location = new_pos;
    }
}
// implicit def on snake updates the snake, inserts new pos at 0 and if not eating pops the tail
// out.
impl Snake {
    fn update_snake(&mut self, new_pos: Point,eating:bool) {

        self.body.insert(0,new_pos);
        if !eating {
            self.body.pop();
        }
    }
}


impl Game {
    // Main game cycle
    fn game_update(&mut self){//-> bool {
        let new_pos = calc_next_step(self.direction,self.snake.body[0],self.width,self.height);//newpos
                                                                                        
        if iscannibal(&new_pos,&self.snake) {//check if eating self 
            self.gameover = true;
            return;
        }

            let eating = iseating(new_pos,&self.food);//check if on food 
            self.snake.update_snake(new_pos,eating);//update pos 
            if eating {//update score if eating 
                self.score += 1;
                self.food.gen_food(&self.snake,self.width,self.height);
            }

    }
    // complicated, learning events/keyinputs
    fn handle_input(&mut self) {
       if event::poll(std::time::Duration::from_millis(0)).unwrap(){
        let event = event::read().unwrap();
            match event {
                Event::Key(key_event) => {
                    match key_event.code{
                    KeyCode::Up => {
                        if self.direction != Direction::Down {
                            self.direction = Direction::Up;
                        }
                    }
                    KeyCode::Down => {
                        if self.direction != Direction::Up {
                            self.direction = Direction::Down;
                        }
                    }
                    KeyCode::Left => {
                        if self.direction != Direction::Right {
                            self.direction = Direction::Left;
                        }
                    }
                    KeyCode::Right => {
                        if self.direction != Direction::Left {
                            self.direction= Direction::Right;
                        }
                    }
                    KeyCode::Esc => {
                        self.gameover = true; 
                    }
                    _ => {}
                    }
                }
                _ => {}
            }

       }

    }
    // creates a new game 
    fn new_game() -> Game {
        let width = 40;
        let height = 25;
        let newsnake = Snake{body:(vec![Point{x:width/2,y:height/2,},
        Point{x:width/2, y:height/2+1, },
        Point{x:width/2, y:height/2+2,},
        ])} ;
        let mut food = Food{ location: Point{ x:0 , y:0,}};
        food.gen_food(&newsnake,width,height);
        let direction = Direction::Up;
        Game {
            snake:newsnake,
            food,
            direction,
            width,
            height,
            score:0,
            gameover:false,
    }
    }
    // drawing function 
    fn draw(&self) {
    // crossterm to move cursor
    let mut stdout = stdout();
    execute!(
        stdout,
        cursor::MoveTo(0,0),
    )//Clear(ClearType::All)) // don't need to clear terminal each time 
        .unwrap();
    //
    for y in 0..self.height {
        for x in 0..self.width{
            let p = Point {x,y,};
            if self.snake.body.contains(&p){
                print!("O");
            } else if p == self.food.location {
                print!("*");
            } else if p.x == 0 || p.x == self.width-1 || p.y == 0 || p.y == self.height - 1 {
                print!("#")
            } else {
                print!(" ");
            }
        }
    
        print!("\r\n");//makes sure returns to the start of the line on newline 
        }
    stdout.flush().unwrap();// makes sure buffer returns 
    }
}
