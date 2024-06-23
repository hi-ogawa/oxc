#![allow(clippy::print_stdout)]
use base64::{prelude::BASE64_STANDARD, Engine};
use oxc_allocator::Allocator;
use oxc_codegen::{CodeGenerator, CodegenReturn};
use oxc_parser::Parser;
use oxc_span::SourceType;

// cargo run -p oxc_codegen --example repro

// current
// https://evanw.github.io/source-map-visualization/#MTI2AGV4cG9ydCBmdW5jdGlvbiBnKCkgewoJbGV0IGEgPSAzOwoJbGV0IGIgPSA0OwoJcmV0dXJuIGEgKyBiOwp9CmV4cG9ydCBmdW5jdGlvbiBmKCkgewoJbGV0IHggPSAxOwoJbGV0IHkgPSAyOwoJcmV0dXJuIHggKyB5Owp9CjMwOAB7InZlcnNpb24iOjMsIm5hbWVzIjpbXSwic291cmNlcyI6WyJ0ZXN0LmpzIl0sInNvdXJjZXNDb250ZW50IjpbIlxuZXhwb3J0IGZ1bmN0aW9uIGYoKSB7XG4gICAgbGV0IHggPSAxO1xuICAgIGxldCB5ID0gMjtcbiAgICByZXR1cm4geCArIHk7XG59XG5leHBvcnQgZnVuY3Rpb24gZygpIHtcbiAgICBsZXQgYSA9IDM7XG4gICAgbGV0IGIgPSA0O1xuICAgIHJldHVybiBhICsgYjtcbn1cbiJdLCJtYXBwaW5ncyI6IkFBTUEsT0FBTyxTQUFTLElBQUk7Q0FDaEIsSUFBSSxJQUFJO0NBQ1IsSUFBSSxJQUFJO0FBQ1IsUUFBTyxJQUFJO0FBQ2QifQ==

// after commenting out `last_position` check
// https://evanw.github.io/source-map-visualization/#MTI2AGV4cG9ydCBmdW5jdGlvbiBnKCkgewoJbGV0IGEgPSAzOwoJbGV0IGIgPSA0OwoJcmV0dXJuIGEgKyBiOwp9CmV4cG9ydCBmdW5jdGlvbiBmKCkgewoJbGV0IHggPSAxOwoJbGV0IHkgPSAyOwoJcmV0dXJuIHggKyB5Owp9CjM3OQB7InZlcnNpb24iOjMsIm5hbWVzIjpbXSwic291cmNlcyI6WyJ0ZXN0LmpzIl0sInNvdXJjZXNDb250ZW50IjpbIlxuZXhwb3J0IGZ1bmN0aW9uIGYoKSB7XG4gICAgbGV0IHggPSAxO1xuICAgIGxldCB5ID0gMjtcbiAgICByZXR1cm4geCArIHk7XG59XG5leHBvcnQgZnVuY3Rpb24gZygpIHtcbiAgICBsZXQgYSA9IDM7XG4gICAgbGV0IGIgPSA0O1xuICAgIHJldHVybiBhICsgYjtcbn1cbiJdLCJtYXBwaW5ncyI6IkFBTUEsT0FBTyxTQUFTLElBQUk7Q0FDaEIsSUFBSSxJQUFJO0NBQ1IsSUFBSSxJQUFJO0FBQ1IsUUFBTyxJQUFJO0FBQ2Q7QUFURCxPQUFPLFNBQVMsSUFBSTtDQUNoQixJQUFJLElBQUk7Q0FDUixJQUFJLElBQUk7QUFDUixRQUFPLElBQUk7QUFDZCJ9

fn main() -> std::io::Result<()> {
    let source_text = r#"
export function f() {
    let x = 1;
    let y = 2;
    return x + y;
}
export function g() {
    let a = 3;
    let b = 4;
    return a + b;
}
"#;
    let source_type = SourceType::default().with_module(true);
    let allocator = Allocator::default();
    let mut ret = Parser::new(&allocator, &source_text, source_type).parse();
    assert!(ret.errors.len() == 0);

    // swap two statements
    ret.program.body.swap(0, 1);

    let CodegenReturn { source_text, source_map } =
        CodeGenerator::new().enable_source_map("test.js", &source_text).build(&ret.program);
    let result = source_map.unwrap().to_json_string().unwrap();
    let hash = BASE64_STANDARD.encode(format!(
        "{}\0{}{}\0{}",
        source_text.len(),
        source_text,
        result.len(),
        result
    ));
    println!("https://evanw.github.io/source-map-visualization/#{hash}");
    Ok(())
}
