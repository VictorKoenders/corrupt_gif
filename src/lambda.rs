use lambda_runtime::{error::HandlerError, lambda, Context};
use simple_error::bail;
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}


fn main() {
        lambda!(my_handler);
}

fn my_handler(e: CustomEvent, _c: Context) -> Result<CustomOutput, HandlerError> {
    if e.first_name == "" {
        bail!("Empty first name");
    }

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}


