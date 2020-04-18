/* SPDX-License-Identifier: GPL-3.0
 * Author: David Heidelberg <david@ixit.cz>
 */

extern crate reqwest;
extern crate serde_json;

use std::io;
use std::io::Read;
use std::fs::File;

//use serde_json::Result as JsonResult;
use serde_json::Value as JsonValue;

// GTK
extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow};
// GTK end

#[derive(Copy, Clone)]
struct CurrencyPair {
    left_type: &'static str,
    left_val: f64,
    conv_rate: f64,
    right_type: &'static str,
    right_val: f64,
}

impl CurrencyPair {
    fn new(left: &'static str, right: &'static str) -> CurrencyPair {
        CurrencyPair {
            left_type: left,
            left_val: 1.0,
            conv_rate: 0.0,
            right_type: right,
            right_val: 0.0,
        }
    }
}

fn main() {

    // GTK stuff
    let application = Application::new(
        Some("org.ixit.currency"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        let mut curr_pair = CurrencyPair::new("USD", "CZK");

        window.set_title("Currency");
        window.set_default_size(350, 70);

        // my code
        let rates = read_rates(&curr_pair.left_type.to_string()); // read filename directly
        let rates = match rates {
            Ok(file) => file,
            Err(error) => {
                panic!("Cannot open file: {:?}", error);
            },
        };
    
        let json = serde_json::from_str(&rates);
        let json = match json {
            Ok(data) => data,
            Err(_) => {
                panic!("Cannot parse JSON");
            },
        };
    
        let p: JsonValue = json;
       println!("The data is based on {} and downloaded {}", p["base"], p["date"]);
        curr_pair.conv_rate = p["rates"]["CZK"].as_f64().unwrap();
        println!("conversion rate: {}", curr_pair.conv_rate);
        // end of my code    

        let input_field_right = gtk::Entry::new();
        input_field_right.set_text("1"); // recall latest

        let input_field_left = gtk::Entry::new();

        let top_column = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        let row = gtk::Box::new(gtk::Orientation::Vertical, 5);
        
        let label_left = gtk::Label::new(Some(&curr_pair.left_type));
        let label_right = gtk::Label::new(Some(&curr_pair.right_type));

        top_column.add(&label_left);
        top_column.add(&input_field_left);
        top_column.add(&label_right);
        top_column.add(&input_field_right);

        row.add(&top_column);
        window.add(&row);

        window.show_all();

        input_field_right.connect_activate(|_| {
            println!("Right!");
        });

        input_field_left.connect_changed(move |this| {
            let buffer = this.get_buffer();
            let input_left = buffer.get_text();
            
            let input_left = input_left.trim().parse();
            // FIXME don't want to work with copy
            let mut working_pair = curr_pair;
            working_pair.left_val = match input_left {
                Ok(num) => num,
                Err(error) => {
                    println!("Invalid number: {:?}", error);
                    0.0
                },
            };

            working_pair.right_val = working_pair.left_val * working_pair.conv_rate;
            println!("{} {} is {} {}", working_pair.left_val, working_pair.left_type, working_pair.right_val, working_pair.right_type);
            input_field_right.set_text(&working_pair.right_val.to_string());
        });

    });

    application.run(&[]);
    // GTK window stuff end}
}

fn read_rates(currency: &String) -> Result<String, io::Error> {
    let f = File::open(currency);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    Ok(content)
}