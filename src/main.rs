use std::fs;
use std::io;

fn main() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    loop {
        println!("Please input your function");

        let mut invoke_func_name = String::new();
        io::stdin()
            .read_line(&mut invoke_func_name)
            .expect("Failed read line");

        let path = format!("./tmp/{}.js", invoke_func_name.trim());

        let file_content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        // init v8 scope
        let isolate = &mut v8::Isolate::new(Default::default());
        let scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(scope);
        let scope = &mut v8::ContextScope::new(scope, context);

        // excute code
        let code = v8::String::new(scope, &file_content).unwrap();
        let script = v8::Script::compile(scope, code, None).unwrap();
        let result = script.run(scope).unwrap();
        let result = result.to_string(scope).unwrap();
        println!("result: {}", result.to_rust_string_lossy(scope));
    }
}
