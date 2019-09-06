#[macro_use]
extern crate gotham_derive;
#[macro_use]
extern crate serde_derive;

// use futures::{Future, Stream};

use gotham::{
    router::{
        builder::*,
        Router,
    },
};

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringExtractor {
    name: Option<String>,
}

mod handlers {
    use hyper::{Body, Response, StatusCode};
    use gotham::{
        helpers::http::response::create_response,
        state::{FromState, State},
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
        use crate::QueryStringExtractor;
        use super::*;

        pub fn index(mut state: State) -> (State, Response<Body>) {
            let res = {
                let query_param = QueryStringExtractor::take_from(&mut state);

                let msg = match query_param.name {
                    Some(name) => { format!("Hero: {}", name) },
                    None => { format!("List All Heroes!") },
                };

                create_response(
                    &state,
                    StatusCode::OK,
                    mime::TEXT_PLAIN,
                    msg,
                )
            };

            (state, res)
        }
    }
}

/// Create a `Router`
///
/// Results in a tree of routes that looks like:
///
/// /           --> GET, HEAD
/// /heroes     --> GET, HEAD
///
/// If no match for a request is found a 404 will be returned. Both the HTTP verb and the request
/// path are considered when determining if the request matches a defined route.
fn router() -> Router {
    use self::handlers::*;

    build_simple_router(|route| {
        route.get_or_head("/").to(index);
        route
            .get_or_head("/heroes")
            .with_query_string_extractor::<QueryStringExtractor>()
            .to(heroes::index);
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