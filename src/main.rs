extern crate cursive;

use cursive::{
    views::{
        TextView, 
        Dialog,
        Checkbox,
        EditView,
        ListView
    },
    traits::Identifiable,
    Cursive
};

// Form fields
struct CatsayOptions<'a> {
    message: &'a str,
    dead: bool
}

// Layout definition for first form
fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please, fill out the form for the cat")
            .content(
                ListView::new()
                    .child(
                        "Message:",
                        EditView::new().with_name("message")
                    )
                    .child(
                        "Dead?", 
                        Checkbox::new().with_name("dead")
                    )
            ).button("OK", |s| {
                let message = s
                    .call_on_name(
                        "message",
                        |t: &mut EditView| t.get_content()
                    ).unwrap();
                
                let is_dead = s
                    .call_on_name(
                        "dead",
                        |t: &mut Checkbox| t.is_checked()
                    ).unwrap();

                let options = CatsayOptions {
                    message: &message,
                    dead: is_dead
                };

                result_step(s, &options);
            })
    );
}

// Cat printing layout
fn result_step(siv: &mut Cursive, options: &CatsayOptions) {
    let eye = if options.dead { "x" } else { "o" };
    let cat_text = format!(
        "{msg}
        \\
         \\
          /\\_/\\
         ({eye} {eye} )
        =( I )=",
        msg = options.message,
        eye = eye
    );

    siv.pop_layer();
    siv.add_layer(
        Dialog::around(TextView::new(cat_text))
            .title("The cat says...")
            .button("OK", |s| s.quit())
    );
}

fn main() {
    let mut siv = cursive::default();
    
    // Call layout flow
    input_step(&mut siv);

    // Event loop start
    siv.run();
}
