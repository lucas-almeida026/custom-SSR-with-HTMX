use std::{io::BufRead, ops::Deref};

use crate::ErrorMsg;
use swc_common as Swc;
use swc_ecma_ast as Ast;
use swc_ecma_parser as Parser;
use Swc::source_map as Sm;

pub enum Tag {
	Native(String),
	Component(String),
}

impl std::string::ToString for Tag {
	fn to_string(&self) -> String {
		match self {
			Tag::Native(s) => s.to_string(),
			Tag::Component(s) => s.to_string(),
		}
	}
}

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

pub fn get_fn_params(fn_expr: Result<Ast::FnExpr, ErrorMsg>) -> Result<Vec<Ast::Param>, ErrorMsg> {
	match fn_expr {
		Ok(fn_expr) => Ok(fn_expr.function.params),
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

fn with_tabs(str: String, tab: i32) -> String {
	let mut buffer = String::from("");
	for _ in 0..tab {
		buffer.push(' ');
	}
	buffer.push_str(&str);
	return buffer;
}

pub fn traverse_jsx_tree(e: Ast::JSXElement, tab: i32) -> String {
	let tag = traverse_opening(e.opening, tab);
	let mut buffer = String::new();
	match tag {
		Tag::Native(t) => {
			buffer.push_str(&t);
			let closing = traverse_closing(e.closing);
			if closing.is_none() {
				buffer.push('/');
				buffer.push('>');
			} else {
				buffer.push('>');
			}
			for (idx, child) in e.children.iter().enumerate() {
				match child {
					Ast::JSXElementChild::JSXElement(e) => {
						if idx == 1 {
							buffer.push('\n');
						}
						buffer.push_str(&traverse_jsx_tree(e.deref().clone(), tab + 2));
					},
					Ast::JSXElementChild::JSXText(e) => buffer.push_str(&e.value.trim()),
					Ast::JSXElementChild::JSXExprContainer(e) => buffer.push_str(&parse_expr_container(e.clone())),
					Ast::JSXElementChild::JSXSpreadChild(_) => buffer.push_str("unhandled jsx spread child"),
					Ast::JSXElementChild::JSXFragment(_) => buffer.push_str("unhandled jsx fragment"),
				}
			}
			buffer.push_str(&closing.clone().unwrap_or("".to_string()));
			buffer.push('\n');
		},
		Tag::Component(c) => {
			buffer.push_str(&as_indexed_child(c));
			buffer.push('\n');
		}
	} 
	
	return with_tabs(buffer, tab);
}

fn as_indexed_child(name: String) -> String {
	return format!("{{#[{name}]}}");
}

fn as_named_param(name: String) -> String {
	return format!("{{{{{name}}}}}");
}

pub fn traverse_opening(e: Ast::JSXOpeningElement, tab: i32) -> Tag {
	let name = parse_name(e.name);
	if is_html_tag(&name) {
		let mut buffer = String::from(&with_tabs("<".to_string(), tab));
		buffer.push_str(&name);
		for attr in e.attrs {
			buffer.push_str(&parse_attr(attr));
		}
		return Tag::Native(buffer);
	} else {
		return Tag::Component(name);
	}
}

pub fn traverse_closing(e: Option<Ast::JSXClosingElement>) -> Option<String> {
	match e {
		Some(e) => {
			let mut buffer = "</".to_string();
			buffer.push_str(&parse_name(e.name));
			buffer.push('>');
			return Some(buffer);
		},
		None => return None,
	}
}

pub fn parse_expr_container(e: Ast::JSXExprContainer) -> String {
	match e.expr {
		Ast::JSXExpr::JSXEmptyExpr(_) => "".to_string(),
		Ast::JSXExpr::Expr(expr) => parse_expr(*expr),
	}
}

pub fn parse_expr(e: Ast::Expr) -> String {
	match e {
		Ast::Expr::Ident(ident) => as_named_param(ident.sym.to_string()),
		_ => "unknown expr".to_string(),
	}
}

pub fn parse_name(e: Ast::JSXElementName) -> String {
	match e {
		Ast::JSXElementName::Ident(ident) => ident.sym.to_string(),
		_ => "unknown opening name".to_string(), //Pages.index = MemberExpr
	}
}

pub fn parse_attr(e: Ast::JSXAttrOrSpread) -> String {
	let mut buffer = String::from(" ");
	match e {
		Ast::JSXAttrOrSpread::JSXAttr(attr) => {
			buffer.push_str(&parse_attr_name(attr.name));
			if attr.value.is_some() {
				buffer.push_str("=");
			}
			buffer.push_str(&parse_attr_value(attr.value));
		},
		_ => buffer.push_str("\"unhandled attr of type spread\""),
	}
	return buffer;
}

pub fn parse_attr_name(e: Ast::JSXAttrName) -> String {
	match e {
		Ast::JSXAttrName::Ident(ident) => ident.sym.to_string(),
		_ => "unknown attr name".to_string(),
	}
}

pub fn parse_attr_value(e: Option<Ast::JSXAttrValue>) -> String {
	match e {
		Some(value) => match value {
			Ast::JSXAttrValue::Lit(lit) => parse_lit(lit),
			Ast::JSXAttrValue::JSXFragment(_) => "JSXFragment".to_string(),
			Ast::JSXAttrValue::JSXExprContainer(_) => "JSXExprContainer".to_string(),
			Ast::JSXAttrValue::JSXElement(_) => "JSXElement".to_string(),
		},
		_ => "".to_string(),
	}
}

pub fn parse_lit(e: Ast::Lit) -> String {
	match e {
		Ast::Lit::Str(str) => format!("{:?}", str.value.to_string()),
		_ => "unknown lit".to_string(),
	}
}

pub fn is_html_tag(tag_name: &str) -> bool {
	let html_tags = [
		"html", "head", "title", "body", //basic
		"meta", "link", "base", //metadata
		"h1", "h2", "h3", "h4", "h5", "h6", //headings
		"p", "br", "hr", "strong", "em", "b", "i", "u", "s", "small", "mark", "bdi", "bdo", "cite", "del", "pre", "sub", "sup", //text
		"ul", "ol", "li", "dl", "dt", "dd", "datalist", "menu", //lists
		"a", "nav", //links
		"img", "figure", "figcaption", "audio", "video", "source", "track", "area", "map", "picture", //media
		"table", "tr", "td", "th", "thead", "tbody", "tfoot", "caption", //tables
		"form", "input", "textarea", "button", "select", "option", "label", "fieldset", "optgroup", "legend", "time", //forms
		"article", "section", "aside", "header", "footer", "main", "nav", "figure", "figcaption", "hgroup", //semantic
		"div", "span", "iframe", "embed", "object", "param", "details", "summary", "abbr", "q", "blockquote", "code", "dialog", "progress", //common
		"col", "colgroup", "data", "ins", "kbd", "meter", "noscript", "output", "samp", "rp", "rt", "ruby", "template", "var", "dfn", "wbr" // others
	];
	return html_tags.contains(&tag_name);
}

pub fn get_sample_code() -> String {
	let code1 = r#"
export default function MyText({ text }) {
	return (
		<Page.index>{text}</Page.index>
	)
}	
"#;
    let code3 = r#"
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
    return code1.to_string();
}
