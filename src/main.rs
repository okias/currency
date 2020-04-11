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



fn main() {

    // GTK stuff
    let application = Application::new(
        Some("org.ixit.currency"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Currency");
        window.set_default_size(350, 70);

        // my code
        let currency = "USD".to_string();

        let rates = read_rates(&currency);
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
        let conversion_rate = p["rates"]["CZK"].as_f64().unwrap();
        println!("conversion rate: {}", conversion_rate);
        // end of my code    

        let input_field_right = gtk::Entry::new();
        input_field_right.set_text("1"); // recall latest

        let input_field_left = gtk::Entry::new();

        let top_column = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        let row = gtk::Box::new(gtk::Orientation::Vertical, 5);
        
        let label_left = gtk::Label::new(Some(&currency));
        let label_right = gtk::Label::new(Some("CZK")); // FIXME hardcoded

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
            let num_to_convert = buffer.get_text();
            
            let num_to_convert = num_to_convert.trim().parse();

            let num_to_convert: f64 = match num_to_convert {
                Ok(num) => num,
                Err(error) => {
                    println!("Invalid number: {:?}", error);
                    0.0
                },
            };
            let converted = num_to_convert * conversion_rate;
            println!("{} USD is {} CZK", num_to_convert, converted);
            input_field_right.set_text(&converted.to_string());
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

/*fn read_input(&this gtk::Entry) -> f64 {
    let buffer = this.get_buffer();
    let num_to_convert = buffer.get_text();
    
    let num_to_convert = num_to_convert.trim().parse();

    let num_to_convert: f64 = match num_to_convert {
        Ok(num) => num,
        Err(error) => {
            println!("Invalid number: {:?}", error);
            0.0
        },
    };
    let converted = num_to_convert * 25.0; //conversion_rate;
    println!("{} USD is {} CZK", num_to_convert, converted);

    converted
}
*/
/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_rates() {
        let currency: String = "USD".to_string();
        let okay: String = "okay".to_string();

        let rates = read_rates(&currency);
        match rates {
            Ok(v) => assert_eq!(v, okay),
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn test_read_rates_fail() {
        let currency: String = "USDX".to_string();

        let rates = read_rates(&currency);
        match rates {
            Ok(_v) => assert!(false),
            Err(_e) => assert!(true),
        }
    }
}
*/