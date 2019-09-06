// use futures::{Future, Stream};
use gotham::router::{
    builder::*,
    Router,
};

pub mod handlers {
    use hyper::{Body, Response, StatusCode};
    use gotham::{
        helpers::http::response::create_response,
        state::State,
    };

    pub fn index(state: State) -> (State, Response<Body>) {
        let res = create_response(
            &state,
            StatusCode::OK,
            mime::TEXT_PLAIN,
            String::from("My Heroes!"),
        );

        (state, res)
    }

    pub mod heroes {
        use super::*;

        pub fn index(state: State) -> (State, Response<Body>) {
            let res = create_response(
                &state,
                StatusCode::OK,
                mime::TEXT_PLAIN,
                String::from("List Heroes!"),
            );

            (state, res)
        }
    }
}

fn router() -> Router {
    use handlers::*;

    build_simple_router(|route| {
        route.get_or_head("/").to(index);
        route.get_or_head("/heroes").to(heroes::index);
    })
}

fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);

    gotham::start(addr, router())

    // let server = gotham::init_server(addr, router());

    // // Future to wait for Ctrl+C
    // let signal = tokio_signal::ctrl_c()
    //     .flatten_stream()
    //     .map_err(|error| panic!("Error listening for signal: {}", error))
    //     .take(1)
    //     .for_each(|()| {
    //         println!("Ctrl+C pressed - Goodbye!");
    //         Ok(())
    //     });

    // let serve_until = signal
    //     .select(server)
    //     .map(|(res, _)| res)
    //     .map_err(|(error, _)| error);

    // tokio::run(serve_until);

    // println!("Shutting down gracefully.");
}