use crate::ast::ast::{Ast, Block, FnParam, FunctionType, Stmt, TypeAnnotation};
use crate::lexer::token::{Token, TokenKind};
use anyhow::Result;
use crate::error::PulseError::ExpectedToken;
use crate::lexer::token::TokenKind::Boolean;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<Ast> {
        let mut ast = Ast::new();

        while !self.is_eof() {
            let stmt = self.parse_stmt()?;

            ast.stmts.push(stmt);
        }

        Ok(ast)
    }

    pub fn consume(&mut self) -> Token {
        if !self.is_eof() {
            self.current += 1;
        }

        self.previous()
    }


    pub fn previous(&self) -> Token {
        assert!(self.current > 0);

        self.tokens[self.current - 1].clone()
    }


    pub fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    pub fn is_eof(&self) -> bool {
        self.current >= self.tokens.len() || self.peek().kind == TokenKind::EOF
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Token> {
        let token = self.peek();

        if token.kind == kind {
            Ok(self.consume())
        } else {
            Err(ExpectedToken(
                kind.to_string(),
                format!("Expected token of kind: {}", kind),
                token.span.clone(),
            ).into())
        }
    }
}

impl Parser {
    pub fn parse_stmt(&mut self) -> Result<Stmt> {
        let token = self.peek();

        let stmt = match token.kind {
            TokenKind::Fn | TokenKind::Export => self.parse_fn(),
            _ => unimplemented!("Token kind: {:?}", token.kind),
        };

        log::debug!("{:#?}", stmt);

        stmt
    }

    pub fn parse_type_annotation(&mut self) -> Result<TypeAnnotation> {
        let colon = self.expect(TokenKind::Colon)?;
        let type_name = self.expect(TokenKind::Identifier)?;

        Ok(TypeAnnotation { colon, type_name })
    }

    pub fn parse_return_type(&mut self) -> Result<Option<FunctionType>> {
        if self.peek().kind == TokenKind::Identifier {
            Err(ExpectedToken(
                "arrow".to_string(),
                "Expected arrow".to_string(),
                self.peek().span.clone(),
            ).into())
        } else {
            let arrow = self.consume();
            let type_name = self.expect(TokenKind::Identifier)?;

            Ok(Some(FunctionType { arrow, type_name }))
        }
    }

    pub fn parse_block(&mut self) -> Result<Block> {
        let mut stmts = vec![];

        while self.peek().kind != TokenKind::RightBrace && !self.is_eof() {
            let stmt = self.parse_stmt()?;

            stmts.push(stmt);
        }

        Ok(Block { stmts })
    }

    pub fn parse_fn(&mut self) -> Result<Stmt> {
        let mut exported = false;
        let fn_token = if self.peek().kind == TokenKind::Export {
            self.consume();

            if self.peek().kind == TokenKind::Fn {
                exported = true;

                self.consume()
            } else {
                return Err(ExpectedToken(
                    "function".to_string(),
                    "You can only export functions".to_string(),
                    self.peek().span.clone(),
                ).into());
            }
        } else {
            self.consume()
        };
        let name = self.expect(TokenKind::Identifier)?;

        self.expect(TokenKind::LeftParen)?;
        let mut params = vec![];

        if self.peek().kind != TokenKind::RightParen {
            while self.peek().kind != TokenKind::RightParen && !self.is_eof() {
                let param = self.consume();

                params.push(FnParam {
                    type_annotation: self.parse_type_annotation()?,
                    ident: param,
                });
            }
        }

        self.expect(TokenKind::RightParen)?;

        let return_type = self.parse_return_type()?;

        self.expect(TokenKind::LeftBrace)?;

        let body = self.parse_block()?;

        self.expect(TokenKind::RightBrace)?;

        Ok(Stmt::new_fn(
            fn_token,
            name.literal(),
            params,
            body,
            exported,
            return_type,
        ))
    }
}