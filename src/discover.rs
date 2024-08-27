use std::ops::Deref;

use crate::ErrorMsg;
use swc_common as Swc;
use swc_ecma_ast as Ast;
use swc_ecma_parser as Parser;
use Swc::source_map as Sm;

pub fn build_ast(code: String) -> Result<Ast::Program, ErrorMsg> {
    let syntax = Parser::Syntax::Es(Parser::EsSyntax {
        jsx: true,
        ..Default::default()
    });
    let target = Ast::EsVersion::Es2022;
    let input = Parser::StringInput::new(
        code.as_str(),
        Sm::SmallPos::from_usize(0),
        Sm::SmallPos::from_usize(0),
    );
    let lexer = Parser::lexer::Lexer::new(syntax, target, input, None);
    let mut parser = Parser::Parser::new_from(lexer);
    let program = parser.parse_program();
    match program {
        Ok(ast) => Ok(ast),
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(ErrorMsg::new("Error parsing JSX file"));
        }
    }
}

pub fn filter_module_decl_variants(ast: &Ast::Program) -> (Vec<Ast::ModuleItem>, Vec<Ast::Stmt>) {
	let mut modules: Vec<Ast::ModuleItem> = Vec::new();
	let mut stmts: Vec<Ast::Stmt> = Vec::new();
	for node in ast.clone().module().unwrap().body {
		if node.is_module_decl() {
			modules.push(node);
		} else {
			stmts.push(node.as_stmt().unwrap().clone());
		}
	}
	return (modules, stmts);
}

pub fn get_default_export_decl(modules: Vec<Ast::ModuleItem>) -> Result<Ast::ExportDefaultDecl, ErrorMsg> {
	if modules.len() != 1 {
		return Err(ErrorMsg::new("Expecting 1 and only 1 module statement of type default export"));
	};
	let module = modules.get(0).unwrap().clone();
	if !module.is_module_decl() {
		return Err(ErrorMsg::new("Expecting 1 and only 1 module statement of type default export"));
	};
	match module.module_decl().unwrap() {
		Ast::ModuleDecl::ExportDefaultDecl(decl) => Ok(decl),
		_ => Err(ErrorMsg::new("Expecting 1 and only 1 module statement of type default export")),
	}
}

pub fn get_default_export_fn(decl: Result<Ast::ExportDefaultDecl, ErrorMsg>) -> Result<Ast::FnExpr, ErrorMsg> {
	match decl {
		Ok(decl) => match decl.decl {
			Ast::DefaultDecl::Fn(fn_decl) => Ok(fn_decl),
			_ => Err(ErrorMsg::new("Expecting a function to be exported as default")),
		},
		Err(e) => Err(e),
	}
}

pub fn get_fn_body(fn_expr: Result<Ast::FnExpr, ErrorMsg>) -> Result<Ast::BlockStmt, ErrorMsg> {
	match fn_expr {
		Ok(fn_expr) => match fn_expr.function.body {
			Some(body) => Ok(body),
			None => Err(ErrorMsg::new("Expeting exported function to have a body")),
		},
		Err(e) => Err(e),
	}
}

pub fn get_fn_return_stmt(fn_body: Result<Ast::BlockStmt, ErrorMsg>) -> Result<Ast::ReturnStmt, ErrorMsg> {
	match fn_body {
		Ok (fn_body) => match fn_body.stmts.len() {
			1 => match fn_body.stmts.get(0).unwrap() {
				Ast::Stmt::Return(return_stmt) => Ok(return_stmt.clone()),
				_ => Err(ErrorMsg::new("Expeting exported function to have a single return statement")),
			},
			_ => Err(ErrorMsg::new("Expeting exported function to have a single return statement")),
		},
		Err(e) => Err(e),
	}
}

pub fn get_jsx_expr(stmt: Result<Ast::ReturnStmt, ErrorMsg>) -> Result<Ast::JSXElement, ErrorMsg> {
	match stmt {
		Ok(stmt) => match stmt.arg {
			Some(arg) => match *arg {
				Ast::Expr::JSXElement(element) => {
					println!("Warning: use parenthesis to wrap JSX");
					return Ok(element.deref().clone());
				},
				Ast::Expr::Paren(paren) => match *paren.expr {
					Ast::Expr::JSXElement(element) => Ok(element.deref().clone()),
					_ => Err(ErrorMsg::new("Expeting exported function return JSX")),
				}
				_ => Err(ErrorMsg::new("Expeting exported function return JSX")),
			},
			None => Err(ErrorMsg::new("Expeting exported function return JSX")),
		},
		Err(e) => Err(e),
	}
}

pub fn get_sample_code() -> String {
    let code = r#"
const num = reactive(0);
const increase = () => {
	num.set(x => x + 1);
}
const decrease = () => {
	if (num.get() > 0) {
		num.set(x => x - 1);
	}
}
export default function MyComponent({ text }) {
	return (
		<div className="d-flex flex-col" disabled >
			<h1>This is my counter!</h1>
			<ControllBtn text="-" onClick={decrease} />
			<p>{num.get()}</p>
			<ControllBtn text="+" onClick={increase} />
			<p>{text}</p>
			<span>{SOME_VAL}</span>
		</div>
	)	
}
"#;
    return code.to_string();
}
