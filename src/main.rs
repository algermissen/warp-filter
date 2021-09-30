use std::convert::Infallible;
use warp::{Filter, Rejection};

#[derive(Clone,Debug)]
struct Validator;

async fn async_validate(vld: Validator, val: i32) -> Result<String, Infallible> {
    Ok(format!("Validator '{:?}' says '{}' is valid.", vld, val))
}

fn validated_thing(validator:Validator) -> impl Filter<Extract = (String,), Error = Rejection> + Clone{
    warp::any().map(move || validator.clone()).and(warp::path::param()).and_then(move |vld:Validator, v:i32| {
            async_validate(vld,v)
        })
}


#[tokio::main]
async fn main() {
    let validator = Validator;

    let route = warp::path("test").and(validated_thing(validator))
        .map(|value| format!("Result: {}", value));

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
