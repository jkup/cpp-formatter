use rusty_v8 as v8;
use std::fs;

fn main() {
    // Initialize the V8 platform and runtime.
    let platform = v8::new_default_platform(0, false); // 0 threads, inspector disabled
    v8::V8::initialize_platform(platform.into());
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    // Read the input file.
    let code = fs::read_to_string("input.js").expect("Unable to read file");

    // Compile and run the script.
    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);
    let code = v8::String::new(scope, &code).unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();

    // Convert the result to a string and print it.
    let result = result.to_string(scope).unwrap();
    println!("{}", result.to_rust_string_lossy(scope));
    
    unsafe {
        v8::V8::dispose();
    }
    
    v8::V8::shutdown_platform();
}