use std::env;

fn main()
{
    env_logger::init();
    //
    if env::var("INTERACTIVE").is_ok() {
        lab_05::dbscan::run_interactive();
    } else if env::var("COMPARISON").is_ok() {
        lab_05::dbscan::run_comparison();
    }
}
