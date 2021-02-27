use peslib::env::JsysCleanEnv;
use peslib::BaseEnv;

fn main() {
    let clean_env = JsysCleanEnv::new();
    for env in clean_env.base_env() {
        println!("{:?}", env);
    }
}