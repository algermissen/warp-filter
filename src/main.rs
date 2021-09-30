use std::convert::Infallible;
use warp::Filter;

#[derive(Clone,Debug)]
struct Validator;

fn _validate(vld: Validator, val: i32) -> Result<String, Infallible> {
    Ok(format!("Validator '{:?}' says '{}' is valid.", vld, val))
}

async fn async_validate(vld: Validator, val: i32) -> Result<String, Infallible> {
    Ok(format!("Validator '{:?}' says '{}' is valid.", vld, val))
}



#[tokio::main]
async fn main() {
    let validator = Validator;

    let route = warp::path!("test" / i32)
        .and_then(move |v| {
            async_validate(validator.clone(),v)
        })
        .map(|value| format!("Result: {}", value));

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
