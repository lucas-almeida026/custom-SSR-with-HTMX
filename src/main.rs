use std::rc::Rc;
use hotwatch as hw;
use serde_json::{self, json};
use std::fs;
use std::io::Error;
use std::path;
use std::string::String;
use std::time::Duration;
use swc_common as Swc;
use swc_ecma_ast as Ast;
use swc_ecma_parser as Parser;
use swc_ecma_codegen as Gen;
use swc_ecma_visit as Visit;
use Swc::source_map as Sm;
use handlebars::Handlebars;

mod discover;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorMsg {
	pub message: String,
}
impl core::fmt::Display for ErrorMsg {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}\n", self.message)
	}
}
impl ErrorMsg {
	pub fn new(str: &str) -> Self {
		Self { message: String::from(str) }
	}
	pub fn panic(&self) {
		panic!("{}", self.message);
	}
}


fn setup_watchers(watcher: &mut hw::blocking::Hotwatch) -> Result<(), Error> {
    let jsx_file_paths = fs::read_dir("./")?;
    for entry in jsx_file_paths {
        if let Ok(p) = entry {
            if let Some(ext) = path::Path::new(&p.path()).extension() {
                if ext == "jsx" {
                    watcher
                        .watch(&p.path(), |event| {
                            if let hw::EventKind::Modify(_) = event.kind {
                                println!("{:?} changed!", event.paths[0]);
                                hw::blocking::Flow::Continue
                            } else {
                                hw::blocking::Flow::Continue
                            }
                        })
                        .expect("Failed to watch");
                }
            }
        }
    }
    return Ok(());
}
fn main() {
    let mut watcher = hw::blocking::Hotwatch::new_with_custom_delay(Duration::from_millis(100))
        .expect("Failed to initialize");
    if let Err(e) = setup_watchers(&mut watcher) {
        println!("Error setting up watchers: {}", e);
    }

    //let _cm: Swc::sync::Lrc<Swc::SourceMap> = Swc::sync::Lrc::new(Swc::SourceMap::default());
    let code = discover::get_sample_code();
	// parse
    let program = discover::build_ast(code);
    if let Ok(ast) = program {
        let json_string = serde_json::to_string_pretty(&ast).unwrap();
        fs::write(path::Path::new("./ast.json"), json_string).expect("Failed to write to file");
		// verify & separate
		if !ast.is_module() {
			panic!("Expecting at least 1 module statement of type default export");
		}
		let (modules, stmts) = discover::filter_module_decl_variants(&ast);
		let default_export = discover::get_default_export_decl(modules);
		let fn_expr = discover::get_default_export_fn(default_export);
		let fn_body = discover::get_fn_body(fn_expr);
		let fn_return = discover::get_fn_return_stmt(fn_body);
		let jsx_expr = discover::get_jsx_expr(fn_return);
		if jsx_expr.is_err() {
			panic!("{}", jsx_expr.unwrap_err().message);
		} else {
			let parsed_html = discover::traverse_jsx_tree(jsx_expr.unwrap(), 0);
			println!("{}", parsed_html.clone());
			// let mut reg = Handlebars::new();
			// println!("{}", reg.render_template(&parsed_html, &json!({"text": "some text"})).unwrap());
			fs::write("./component.html", parsed_html).expect("Failed to write to file");
		}
		
		//emit js
		let mut conf = Gen::Config::default();
		conf.target = Ast::EsVersion::Es2022;
		conf.minify = false;
		let cm = Rc::new(Sm::SourceMap::default());
		
		let mut buf = std::io::Cursor::new(vec![]);
		let mut emitter = Gen::Emitter {
			cfg: conf,
			comments: None,
			cm: cm.clone(),
			wr: Box::new(Gen::text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
		};
		// emitter.emit_module(&ast.as_module().unwrap()).unwrap();
		let script = Ast::Script {
			body: stmts,
			shebang: None,
			span: Sm::Span::new(Sm::BytePos(0), Sm::BytePos(0))
		};
		emitter.emit_script(&script);
		let src_code = String::from_utf8(buf.into_inner()).unwrap();
		fs::write(path::Path::new("./component.js"), src_code).expect("Failed to write to file");
		
    } else {
        println!("Failed to parse program");
        println!("{:#?}", program);
    }
    //watcher.run().expect("Failed to run");
}
