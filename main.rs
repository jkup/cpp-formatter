use rusty_v8 as v8;
use std::fs;

fn main() {
    // Initialize the V8 platform once, before any isolates are created.
    let platform = v8::new_default_platform(0, false); // 0 threads, inspector disabled
    v8::V8::initialize_platform(platform.into());
    v8::V8::initialize();

    // Run the JavaScript code within its own scope to ensure proper cleanup.
    run_js_script();

    // Properly shut down the V8 platform after all isolates have been disposed of.
    unsafe {
        v8::V8::dispose();
    }
    v8::V8::shutdown_platform();
}

fn run_js_script() {
    // Create a new isolate with default parameters.
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let handle_scope = &mut v8::HandleScope::new(&mut isolate);
    let context = v8::Context::new(handle_scope);
    let context_scope = &mut v8::ContextScope::new(handle_scope, context);

    // Read and compile the JavaScript from a file.
    let code = fs::read_to_string("input.js").expect("Unable to read file");
    let code_string = v8::String::new(context_scope, &code).unwrap();
    let script = v8::Script::compile(context_scope, code_string, None).unwrap();

    // Execute the script and handle the result.
    let result = script.run(context_scope).unwrap();
    let result_string = result.to_string(context_scope).unwrap();
    let rust_string = result_string.to_rust_string_lossy(context_scope);

    // Output the result to the console.
    println!("{}", rust_string);
}

