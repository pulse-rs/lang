use crate::{
    ast::ast::*,
    error::PulseError::{ResolverError, SemanticError},
    lexer::{
        span::TextSpan,
        token::{Token, TokenKind},
    },
};
use anyhow::Result;
use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Void,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    symbol_type: Type,
    is_mutable: bool,
}

#[derive(Debug)]
pub struct Scope {
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            symbols: HashMap::new(),
        }
    }

    fn define(&mut self, name: String, symbol: Symbol) -> Result<()> {
        if self.symbols.contains_key(&name) {
            Err(ResolverError(format!("Duplicate declaration of '{}'", name)).into())
        } else {
            self.symbols.insert(name, symbol);
            Ok(())
        }
    }

    fn resolve(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

#[derive(Debug)]
pub struct Resolver {
    pub scopes: Vec<Scope>,
    pub current_function_return_type: Option<Type>,
}

impl Resolver {
    pub fn new() -> Self {
        let mut resolver = Resolver {
            scopes: vec![Scope::new()],
            current_function_return_type: None,
        };
        resolver
    }

    fn begin_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_symbol(&mut self, name: String, symbol: Symbol, span: TextSpan) -> Result<()> {
        if let Some(scope) = self.scopes.last_mut() {
            scope.define(name.clone(), symbol.clone())?;
        };

        Ok(())
    }

    fn resolve_symbol(&self, name: &str) -> Option<Rc<Symbol>> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.resolve(name) {
                return Some(Rc::new(symbol.clone()));
            }
        }
        None
    }

    pub fn resolve_ast(&mut self, ast: &Ast) -> Result<()> {
        for stmt in &ast.stmts {
            self.resolve_stmt(stmt)?;
        }

        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Use(use_stmt) => {
                self.resolve_use(use_stmt)?;
            }
            Stmt::Fn(fn_stmt) => {
                self.resolve_fn(fn_stmt)?;
            }
            Stmt::Let(let_stmt) => {
                self.resolve_let(let_stmt)?;
            }
            Stmt::If(if_stmt) => {
                self.resolve_if(if_stmt)?;
            }
            Stmt::Return(ret_stmt) => {
                self.resolve_return(ret_stmt)?;
            }
            Stmt::Expr(expr) => {
                self.resolve_expr(expr)?;
            }
            Stmt::Block(block) => {
                self.resolve_block(block)?;
            }
        }

        Ok(())
    }

    // TODO:
    fn resolve_use(&mut self, use_stmt: &Use) -> Result<()> {
        let from_module = use_stmt.from.literal();
        for item in &use_stmt.items {
            let item_name = item.literal();
            let symbol = Symbol {
                name: item_name.clone(),
                symbol_type: Type::Void,
                is_mutable: false,
            };
            self.define_symbol(item_name, symbol, item.span.clone())?;
        }

        Ok(())
    }

    fn resolve_fn(&mut self, fn_stmt: &Fn) -> Result<()> {
        let return_type = if let Some(ftype) = &fn_stmt.return_type {
            self.map_type(&ftype.type_name)
        } else {
            Type::Void
        };

        let fn_name = fn_stmt.name.clone();
        let fn_symbol = Symbol {
            name: fn_name.clone(),
            symbol_type: return_type.clone(),
            is_mutable: false,
        };
        self.define_symbol(fn_name.clone(), fn_symbol, fn_stmt.fn_token.span.clone())?;

        let previous_return_type = self.current_function_return_type.clone();
        self.current_function_return_type = Some(return_type.clone());

        self.begin_scope();

        for param in &fn_stmt.params {
            let param_name = param.ident.literal();
            let param_type = self.map_type(&param.type_annotation.type_name);
            let param_symbol = Symbol {
                name: param_name.clone(),
                symbol_type: param_type.clone(),
                is_mutable: true,
            };
            self.define_symbol(param_name, param_symbol, param.ident.span.clone())?;
        }

        self.resolve_block(&fn_stmt.body)?;

        self.end_scope();

        self.current_function_return_type = previous_return_type;

        Ok(())
    }

    fn resolve_let(&mut self, let_stmt: &Let) -> Result<()> {
        let var_name = let_stmt.ident.literal();
        let var_type = if let Some(type_annotation) = &let_stmt.type_annotation {
            self.map_type(&type_annotation.type_name)
        } else {
            self.infer_expr_type(&let_stmt.initializer)?
        };
        let var_symbol = Symbol {
            name: var_name.clone(),
            symbol_type: var_type.clone(),
            is_mutable: true,
        };
        self.define_symbol(var_name.clone(), var_symbol, let_stmt.ident.span.clone())?;

        self.resolve_expr(&let_stmt.initializer)?;

        Ok(())
    }

    fn resolve_if(&mut self, if_stmt: &If) -> Result<()> {
        self.resolve_expr(&if_stmt.condition)?;

        let cond_type = self.infer_expr_type(&if_stmt.condition)?;
        if cond_type != Type::Bool {
            return Err(SemanticError(
                format!(
                    "Condition in 'if' statement must be of type Bool, found {:?}",
                    cond_type
                ),
                if_stmt.if_token.span.clone(),
            )
            .into());
        }

        self.begin_scope();
        self.resolve_block(&if_stmt.then_block)?;
        self.end_scope();

        for else_if in &if_stmt.else_ifs {
            self.resolve_expr(&else_if.condition)?;

            let else_if_cond_type = self.infer_expr_type(&else_if.condition)?;
            if else_if_cond_type != Type::Bool {
                return Err(SemanticError(
                    format!(
                        "Condition in 'else if' statement must be of type Bool, found {:?}",
                        else_if_cond_type
                    ),
                    else_if.condition.span(),
                )
                .into());
            }

            self.begin_scope();
            self.resolve_block(&else_if.block)?;
            self.end_scope();
        }

        if let Some(else_block) = &if_stmt.else_block {
            self.begin_scope();
            self.resolve_block(&else_block.block)?;
            self.end_scope();
        }

        Ok(())
    }

    fn resolve_return(&mut self, ret_stmt: &Return) -> Result<()> {
        if let Some(expr) = &ret_stmt.expr {
            self.resolve_expr(expr)?;
            let expr_type = self.infer_expr_type(expr)?;
            if let Some(expected_type) = &self.current_function_return_type {
                if &expr_type != expected_type {
                    return Err(SemanticError(
                        format!(
                            "Type mismatch in return statement: expected {:?}, found {:?}",
                            expected_type, expr_type
                        ),
                        ret_stmt.return_token.span.clone(),
                    )
                    .into());
                }
            }
        } else {
            if let Some(expected_type) = &self.current_function_return_type {
                if *expected_type != Type::Void {
                    return Err(SemanticError(
                        format!(
                            "Return statement missing expression: expected return type {:?}",
                            expected_type
                        ),
                        ret_stmt.return_token.span.clone(),
                    )
                    .into());
                }
            }
        }

        Ok(())
    }

    fn resolve_block(&mut self, block: &Block) -> Result<()> {
        self.begin_scope();
        for stmt in &block.stmts {
            self.resolve_stmt(stmt)?;
        }
        self.end_scope();

        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Literal(_) => Ok(()),
            Expr::Variable(var) => {
                self.resolve_variable(var)?;
                Ok(())
            }
            Expr::Binary(bin) => {
                self.resolve_expr(&bin.left)?;
                self.resolve_expr(&bin.right)?;
                self.check_binary_operator(bin)?;
                Ok(())
            }
            Expr::Unary(un) => {
                self.resolve_expr(&un.expr)?;
                self.check_unary_operator(un)?;
                Ok(())
            }
            Expr::Logical(logical) => {
                self.resolve_expr(&logical.left)?;
                self.resolve_expr(&logical.right)?;
                let left_type = self.infer_expr_type(&logical.left)?;
                let right_type = self.infer_expr_type(&logical.right)?;
                if left_type != Type::Bool || right_type != Type::Bool {
                    return Err(SemanticError(
                        "Logical operations require Bool operands".to_string(),
                        logical.token.span.clone(),
                    )
                    .into());
                }

                Ok(())
            }
            Expr::Call(call) => {
                let callee_symbol = self.resolve_symbol(&call.callee);
                if callee_symbol.is_none() {
                    return Err(SemanticError(
                        format!("Undefined function '{}'", call.callee),
                        call.token.span.clone(),
                    )
                    .into());
                }

                for arg in &call.args {
                    self.resolve_expr(arg)?;
                }

                Ok(())
            }
            Expr::Assign(assign) => {
                self.resolve_assign(assign)?;
                Ok(())
            }
            Expr::Parenthesized(paren) => {
                self.resolve_expr(&paren.expr)?;
                Ok(())
            }
        }
    }

    fn resolve_variable(&mut self, var: &Variable) -> Result<()> {
        if self.resolve_symbol(&var.ident).is_none() {
            return Err(SemanticError(
                format!("Undefined variable '{}'", var.ident),
                var.token.span.clone(),
            )
            .into());
        }

        Ok(())
    }

    fn resolve_assign(&mut self, assign: &Assign) -> Result<()> {
        if let Some(symbol) = self.resolve_symbol(&assign.ident.literal()) {
            if !symbol.is_mutable {
                return Err(SemanticError(
                    format!("Cannot assign to immutable variable '{}'", symbol.name),
                    assign.token.span.clone(),
                )
                .into());
            }
            let value_type = self.infer_expr_type(&assign.value)?;
            if &symbol.symbol_type != &value_type {
                return Err(SemanticError(
                    format!(
                        "Type mismatch in assignment to '{}': expected {:?}, found {:?}",
                        symbol.name, symbol.symbol_type, value_type
                    ),
                    assign.token.span.clone(),
                )
                .into());
            }
        } else {
            return Err(SemanticError(
                format!("Undefined variable '{}'", assign.ident.literal()),
                assign.token.span.clone(),
            )
            .into());
        }

        self.resolve_expr(&assign.value)?;

        Ok(())
    }

    fn infer_expr_type(&mut self, expr: &Expr) -> Result<Type> {
        match expr {
            Expr::Literal(lit) => Ok(match lit.value {
                LiteralType::Int(_) => Type::Int,
                LiteralType::Float(_) => Type::Float,
                LiteralType::Bool(_) => Type::Bool,
                LiteralType::String(_) => Type::String,
                LiteralType::Null => Type::Void,
            }),
            Expr::Variable(var) => {
                if let Some(symbol) = self.resolve_symbol(&var.ident) {
                    Ok(symbol.symbol_type.clone())
                } else {
                    Err(SemanticError(
                        format!("Undefined variable '{}'", var.ident).to_string(),
                        var.token.span.clone(),
                    )
                    .into())
                }
            }
            Expr::Binary(bin) => match bin.operator {
                BinOpKind::GreaterThan
                | BinOpKind::LessThan
                | BinOpKind::Equals
                | BinOpKind::NotEquals
                | BinOpKind::LessThanOrEqual
                | BinOpKind::GreaterThanOrEqual => Ok(Type::Bool),
                BinOpKind::And | BinOpKind::Or => Ok(Type::Bool),
                BinOpKind::Plus
                | BinOpKind::Minus
                | BinOpKind::Multiply
                | BinOpKind::Divide
                | BinOpKind::Power
                | BinOpKind::Modulo => {
                    let left_type = self.infer_expr_type(&bin.left)?;
                    let right_type = self.infer_expr_type(&bin.right)?;
                    if left_type == right_type {
                        Ok(left_type)
                    } else {
                        Ok(Type::Void)
                    }
                }
                _ => Ok(Type::Void),
            },
            Expr::Unary(un) => self.infer_expr_type(&un.expr),
            Expr::Logical(_) => Ok(Type::Bool),
            Expr::Call(call) => {
                if let Some(symbol) = self.resolve_symbol(&call.callee) {
                    Ok(symbol.symbol_type.clone())
                } else {
                    Err(SemanticError(
                        format!("Undefined function '{}'", call.callee).to_string(),
                        call.token.span.clone(),
                    )
                    .into())
                }
            }
            Expr::Assign(assign) => self.infer_expr_type(&assign.value),
            Expr::Parenthesized(paren) => self.infer_expr_type(&paren.expr),
        }
    }

    fn map_type(&self, type_token: &Token) -> Type {
        match &type_token.kind {
            TokenKind::Identifier => match type_token.literal().as_str() {
                "int" => Type::Int,
                "float" => Type::Float,
                "bool" => Type::Bool,
                "string" => Type::String,
                "void" => Type::Void,
                _ => Type::Void,
            },
            _ => Type::Void,
        }
    }

    fn check_binary_operator(&mut self, bin: &Binary) -> Result<()> {
        let left_type = self.infer_expr_type(&bin.left)?;
        let right_type = self.infer_expr_type(&bin.right)?;
        match bin.operator {
            BinOpKind::Plus
            | BinOpKind::Minus
            | BinOpKind::Multiply
            | BinOpKind::Divide
            | BinOpKind::Power
            | BinOpKind::Modulo => {
                if left_type != right_type {
                    return Err(SemanticError(
                        format!(
                            "Type mismatch in binary operation: {:?} and {:?}",
                            left_type, right_type
                        ),
                        bin.span(),
                    )
                    .into());
                }
                if left_type != Type::Int && left_type != Type::Float {
                    return Err(SemanticError(
                        format!(
                            "Binary operator '{:?}' not supported for type {:?}",
                            bin.operator, left_type
                        ),
                        bin.span(),
                    )
                    .into());
                }

                Ok(())
            }
            BinOpKind::GreaterThan
            | BinOpKind::LessThan
            | BinOpKind::Equals
            | BinOpKind::NotEquals
            | BinOpKind::LessThanOrEqual
            | BinOpKind::GreaterThanOrEqual => {
                if left_type != right_type {
                    return Err(SemanticError(
                        "Comparison operators require operands of the same type".to_string(),
                        bin.span(),
                    )
                    .into());
                }
                if left_type != Type::Int
                    && left_type != Type::Float
                    && left_type != Type::Bool
                    && left_type != Type::String
                {
                    return Err(SemanticError(
                        format!(
                            "Comparison operator '{:?}' not supported for type {:?}",
                            bin.operator, left_type
                        ),
                        bin.span(),
                    )
                    .into());
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn check_unary_operator(&mut self, un: &Unary) -> Result<()> {
        let expr_type = self.infer_expr_type(&un.expr)?;
        match un.operator.kind {
            UnOpKind::Minus => {
                if expr_type != Type::Int && expr_type != Type::Float {
                    return Err(SemanticError(
                        format!("Unary operator '-' not supported for type {:?}", expr_type),
                        un.operator.token.span.clone(),
                    )
                    .into());
                }
            }
            UnOpKind::BitwiseNot => {
                if expr_type != Type::Int {
                    return Err(SemanticError(
                        format!("Unary operator '~' not supported for type {:?}", expr_type),
                        un.operator.token.span.clone(),
                    )
                    .into());
                }
            }
        }

        Ok(())
    }
}
