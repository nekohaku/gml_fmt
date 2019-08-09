use super::lex_token::*;
use super::statements::DelimitedLines;
pub type ExprBox<'a> = Box<(Expr<'a>, CommentsAndNewlines<'a>)>;
pub type CommentsAndNewlines<'a> = Vec<Token<'a>>;

pub type DSAccess<'a> = Vec<(CommentsAndNewlines<'a>, ExprBox<'a>)>;

#[derive(Debug)]
pub enum Expr<'a> {
    Call {
        procedure_name: ExprBox<'a>,
        comments_and_newlines_after_lparen: CommentsAndNewlines<'a>,
        arguments: DelimitedLines<'a>,
    },
    Binary {
        left: ExprBox<'a>,
        operator: Token<'a>,
        comments_and_newlines_between_op_and_r: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Grouping {
        comments_and_newlines_after_lparen: CommentsAndNewlines<'a>,
        expressions: Vec<ExprBox<'a>>,
        comments_and_newlines_after_rparen: CommentsAndNewlines<'a>,
    },
    ArrayLiteral {
        comments_and_newlines_after_lbracket: CommentsAndNewlines<'a>,
        arguments: DelimitedLines<'a>,
    },
    Literal {
        literal_token: Token<'a>,
        comments: CommentsAndNewlines<'a>,
    },
    NumberStartDot {
        literal_token: Token<'a>,
        comments: CommentsAndNewlines<'a>,
    },
    NumberEndDot {
        literal_token: Token<'a>,
        comments: CommentsAndNewlines<'a>,
    },
    Unary {
        operator: Token<'a>,
        comments_and_newlines_between: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Postfix {
        operator: Token<'a>,
        comments_and_newlines_between: CommentsAndNewlines<'a>,
        expr: ExprBox<'a>,
    },
    Assign {
        left: ExprBox<'a>,
        operator: Token<'a>,
        comments_and_newlines_between_op_and_r: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Identifier {
        name: Token<'a>,
        comments: CommentsAndNewlines<'a>,
    },
    DotAccess {
        object_name: ExprBox<'a>,
        instance_variable: ExprBox<'a>,
    },
    DataStructureAccess {
        ds_name: ExprBox<'a>,
        access_type: Token<'a>,
        access_exprs: DSAccess<'a>,
    },
    // x ? y : z;
    Ternary {
        conditional: ExprBox<'a>,
        comments_and_newlines_after_q: CommentsAndNewlines<'a>,
        left: ExprBox<'a>,
        comments_and_newlines_after_colon: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },

    Newline,
    UnidentifiedAsLiteral {
        literal_token: Token<'a>,
    },
    UnexpectedEnd,
}
