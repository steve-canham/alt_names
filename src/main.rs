
use alt_names::error_defs::AppError;
use alt_names::run;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AppError> {

    let args: Vec<_> = env::args_os().collect();
    run(args).await
    
}
