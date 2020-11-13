extern crate cursive;

use cursive::views::TextView;

fn main() {
    let mut siv = cursive::default();
    let cat_text = "Meow!
    \\  
    \\    
      /\\_/\\   
     ( o o )   
     =( I )=";
    
    // App layout declaration
    siv.add_layer(TextView::new(cat_text));

    // Event loop start
    siv.run();
}
