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
                let cmd = Command::new("docker")
                    .arg("start")
                    .arg(image_name)
                    .status()
                    .expect("Can't start server!");

                match cmd.success() {
                    true => {
                        rouille::Response::text("success").with_status_code(200)
                    }
                    false => {
                        rouille::Response::text("error").with_status_code(500)
                    }
                }
            },

            (GET) (/down/{image_name: String}) => {
                let cmd = Command::new("docker")
                    .arg("stop")
                    .arg(image_name)
                    .status()
                    .expect("Can't stop server!");

                match cmd.success() {
                    true => {
                        rouille::Response::text("success").with_status_code(200)
                    }
                    false => {
                        rouille::Response::text("error").with_status_code(500)
                    }
                }
            },

            _ => rouille::Response::empty_404()
        )
    });
}
