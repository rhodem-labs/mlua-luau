mod find;

fn main() {
    println!("cargo:rerun-if-changed=build");

    find::probe_lua();
}
