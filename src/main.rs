#[macro_use]
extern crate rouille;

use std::process::Command;


fn main() {
    println!("Now listening on localhost:8000");

    rouille::start_server("localhost:8000", move |req| {
        router!(req,
            (GET) (/) => {
                rouille::Response::text("hello world")
            },

            (GET) (/up/{image_name: String}) => {
                let res_text = format!("Hello {}", image_name);
                Command::new("docker")
                    .arg("start")
                    .arg(image_name)
                    .spawn()
                    .expect("Can't run ur image lolz!");
                rouille::Response::text(res_text)
            },

            (GET) (/down/{image_name: String}) => {
                let res_text = format!("Hello {}", image_name);
                Command::new("docker")
                    .arg("stop")
                    .arg(image_name)
                    .spawn()
                    .expect("Can't run ur image lolz!");
                rouille::Response::text(res_text)
            },

            _ => rouille::Response::empty_404()
        )
    });
}
