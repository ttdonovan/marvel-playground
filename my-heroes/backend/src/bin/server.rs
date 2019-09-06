use futures::{Future, Stream};
use gotham::{
    helpers::http::response::create_response,
    state::State,
};
use hyper::{Body, Response, StatusCode};

fn say_hello(state: State) -> (State, Response<Body>) {
    let res = create_response(
        &state,
        StatusCode::OK,
        mime::TEXT_PLAIN,
        String::from("My Hereos!"),
    );

    (state, res)
}

fn main() {
    let addr = "127.0.0.1:7878";

    let server = gotham::init_server(addr, || Ok(say_hello));

    // Future to wait for Ctrl+C
    let signal = tokio_signal::ctrl_c()
        .flatten_stream()
        .map_err(|error| panic!("Error listening for signal: {}", error))
        .take(1)
        .for_each(|()| {
            println!("Ctrl+C pressed - Goodbye!");
            Ok(())
        });

    let serve_until = signal
        .select(server)
        .map(|(res, _)| res)
        .map_err(|(error, _)| error);

    tokio::run(serve_until);

    println!("Shutting down gracefully.");
}