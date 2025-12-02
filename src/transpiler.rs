use crate::compiletask::compiletobinary;

use crate::parser::{
    Program,
    Stmt,
    Expr,
    InfixOp,
    PrefixOp,
    Type,
    Block,
    BlockOrIf,
};

use std::fs;
use std::fmt::Write as FmtWrite;

fn indent(out: &mut String, level: usize) {
    for _ in 0..level {
        out.push_str("    "); // 4 spaces
    }
}

fn type_to_c(t: &Type) -> &'static str {
    match t {
        Type::Int       => "int64_t",
        Type::Float     => "double",
        Type::Bool      => "bool",
        Type::String    => "char*",
        Type::Nil       => "void",
        Type::Custom(_) => "void*", // later: map to struct name etc.
    }
}

fn emit_expr(out: &mut String, expr: &Expr) {
    match expr {
        Expr::Int(v) => {
            write!(out, "{v}").unwrap();
        }

        Expr::Float(v) => {
            write!(out, "{v}").unwrap();
        }

        Expr::Bool(true) => out.push_str("1"),
        Expr::Bool(false) => out.push_str("0"),

        Expr::String(s) => {
            out.push('"');
            out.push_str(s);
            out.push('"');
        }

        Expr::Nil => {
            out.push_str("0");
        }

        Expr::Ident(name) => {
            out.push_str(name);
        }

        Expr::Prefix { op, rhs } => {
            match op {
                PrefixOp::Neg => {
                    out.push('-');
                    emit_expr(out, rhs);
                }
                PrefixOp::Not => {
                    out.push('!');
                    emit_expr(out, rhs);
                }
            }
        }

        Expr::Infix { op, lhs, rhs } => {
            out.push('(');
            emit_expr(out, lhs);
            out.push(' ');

            let op_str = match op {
                InfixOp::Add => "+",
                InfixOp::Sub => "-",
                InfixOp::Mul => "*",
                InfixOp::Div => "/",
                InfixOp::Mod => "%",

                InfixOp::Eq => "==",
                InfixOp::Ne => "!=",
                InfixOp::Lt => "<",
                InfixOp::Le => "<=",
                InfixOp::Gt => ">",
                InfixOp::Ge => ">=",

                InfixOp::And => "&&",
                InfixOp::Or  => "||",

                InfixOp::Assign    => "=",
                InfixOp::AddAssign => "+=",
                InfixOp::SubAssign => "-=",
                InfixOp::MulAssign => "*=",
                InfixOp::DivAssign => "/=",
            };

            out.push_str(op_str);
            out.push(' ');
            emit_expr(out, rhs);
            out.push(')');
        }

        Expr::Call { callee, args } => {
            emit_expr(out, callee);
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_expr(out, arg);
            }
            out.push(')');
        }

        Expr::Index { .. }
        | Expr::StructLiteral { .. }
        | Expr::FieldAccess { .. } => {
            // TODO: arrays, structs, fields
            out.push_str("/* TODO complex expr */");
        }

        Expr::Group(inner) => {
            out.push('(');
            emit_expr(out, inner);
            out.push(')');
        }
    }
}

// =======================
// ==== Stmt → C code ====
// =======================
fn emit_stmt(out: &mut String, stmt: &Stmt, indent_level: usize) {
    match stmt {
        Stmt::Let { name, mutable: _, value } => {
            indent(out, indent_level);

            // for now, assume locals are int64_t
            out.push_str("int64_t ");
            out.push_str(name);

            if let Some(expr) = value {
                out.push_str(" = ");
                emit_expr(out, expr);
            }

            out.push_str(";\n");
        }

        Stmt::ExprStmt(expr) => {
            indent(out, indent_level);
            emit_expr(out, expr);
            out.push_str(";\n");
        }

        Stmt::Return(expr_opt) => {
            indent(out, indent_level);
            out.push_str("return");
            if let Some(expr) = expr_opt {
                out.push(' ');
                emit_expr(out, expr);
            }
            out.push_str(";\n");
        }

        Stmt::While { cond, body } => {
            indent(out, indent_level);
            out.push_str("while (");
            emit_expr(out, cond);
            out.push_str(") {\n");
            emit_block(out, body, indent_level + 1);
            indent(out, indent_level);
            out.push_str("}\n");
        }

        Stmt::If { cond, then_branch, else_branch } => {
            indent(out, indent_level);
            out.push_str("if (");
            emit_expr(out, cond);
            out.push_str(") {\n");
            emit_block(out, then_branch, indent_level + 1);
            indent(out, indent_level);
            out.push_str("}");

            if let Some(else_part) = else_branch {
                match else_part {
                    BlockOrIf::Block(block) => {
                        out.push_str(" else {\n");
                        emit_block(out, block, indent_level + 1);
                        indent(out, indent_level);
                        out.push_str("}\n");
                    }
                    BlockOrIf::If(if_stmt) => {
                        out.push_str(" else ");
                        emit_stmt(out, if_stmt, indent_level); // else if
                    }
                }
            } else {
                out.push('\n');
            }
        }

        Stmt::Block(block) => {
            indent(out, indent_level);
            out.push_str("{\n");
            emit_block(out, block, indent_level + 1);
            indent(out, indent_level);
            out.push_str("}\n");
        }

        Stmt::Struct { .. } => {
            // TODO: map to C struct decl
            indent(out, indent_level);
            out.push_str("/* TODO struct decl */\n");
        }

        Stmt::Out => {
            indent(out, indent_level);
            out.push_str("break;\n");
        }

        Stmt::Skip => {
            indent(out, indent_level);
            out.push_str("continue;\n");
        }

        Stmt::Func { .. } => {
            // Shouldn't appear here; functions handled at top-level
            indent(out, indent_level);
            out.push_str("/* nested func? TODO */\n");
        }
    }
}

fn emit_block(out: &mut String, body: &Block, indent_level: usize) {
    for stmt in body {
        emit_stmt(out, stmt, indent_level);
    }
}

fn emit_function(
    out: &mut String,
    name: &str,
    params: &[(String, Type)],
    return_type: &Type,
    body: &Block,
) {
    let ret_ty = type_to_c(return_type);

    // function signature
    indent(out, 0);
    write!(out, "{} {}(", ret_ty, name).unwrap();

    for (i, (pname, pty)) in params.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        let c_ty = type_to_c(pty);
        write!(out, "{} {}", c_ty, pname).unwrap();
    }

    out.push_str(") {\n");

    // body with one level of indent
    emit_block(out, body, 1);

    // closing brace
    indent(out, 0);
    out.push_str("}\n\n");
}

pub fn transpile(program: Program, name: &str) {
    let mut genned_code = String::new();

    // headers
    genned_code.push_str("#include <stdint.h>\n");
    genned_code.push_str("#include <stdbool.h>\n");
    genned_code.push_str("#include <stdio.h>\n\n");


    for stmt in &program.stmts {
        match stmt {
            Stmt::Func { name, params, returntype, body } => {
                emit_function(&mut genned_code, name, params, returntype, body);
            }

            _ => {
                // TODO: handle top-level non-function stmts
            }
        }
    }

    fs::write(format!("{name}.c"), genned_code).expect("failed to write result.c");

    compiletobinary(name);
}
