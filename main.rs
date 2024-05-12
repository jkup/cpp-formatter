use rusty_v8 as v8;
use std::fs;

fn main() {
    // Initialize the V8 platform and runtime.
    let platform = v8::new_default_platform(0, false); // 0 threads, inspector disabled
    v8::V8::initialize_platform(platform.into());
    v8::V8::initialize();

    // Create a new isolate (i.e., a V8 execution environment).
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());

    // Read the JavaScript code from the input file.
    let code = fs::read_to_string("input.js").expect("Unable to read file");

    // Create a handle scope to manage the memory for V8 handles.
    let handle_scope = &mut v8::HandleScope::new(&mut isolate);

    // Create a new context (i.e., the global object) for the JavaScript code.
    let context = v8::Context::new(handle_scope);
    let context_scope = &mut v8::ContextScope::new(handle_scope, context);

    // Convert the Rust string to a V8 string value.
    let code_string = v8::String::new(context_scope, &code).unwrap();

    // Compile the JavaScript code.
    let script = v8::Script::compile(context_scope, code_string, None).unwrap();

    // Run the script and get the result.
    let result = script.run(context_scope).unwrap();

    // Convert the result to a string.
    let result_string = result.to_string(context_scope).unwrap();
    let rust_string = result_string.to_rust_string_lossy(context_scope);

    // Print the result.
    println!("{}", rust_string);

    // Dispose the V8 instance and shutdown the platform properly.
    unsafe {
        v8::V8::dispose();
    }
    v8::V8::shutdown_platform();
}
