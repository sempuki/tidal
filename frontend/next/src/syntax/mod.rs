//! The syntax stage: the cursor over the tokens, the syn node, the
//! extension point for proposals, and the stop that TR-12 names. The
//! productions themselves sit in the `syntax.rs` part of each page
//! module, one function each, named after the production.
//!
//! The parser asks the `Names` oracle at the points where the grammar
//! needs to know what a name is, and reports nothing until a declaration
//! fails. A failed alternative returns `Fail`, and the caller tries the
//! next one; no diagnostic is muted, because none was made.

pub mod names;
#[cfg(test)]
pub mod testing;
#[cfg(test)]
mod tests;

use crate::lex::{Token, TK};
use crate::source::Span;
use names::{DeclCtx, Found, Names, Qual};

/// One use of a production. `prod` names the production, and `ext`
/// names the proposal whose alternative made the node (`syn::Ext`).
/// The levels of the expression chain make a node only when their
/// operator is present, so `x` is a `primary` and not seventeen nodes.
#[derive(Clone, Debug)]
pub struct Node {
    pub prod: &'static str,
    pub ext: Option<&'static str>,
    pub span: Span,
    pub kids: Vec<Kid>,
}

#[derive(Clone, Debug)]
pub enum Kid {
    /// A token, by its index.
    Tok(usize),
    Node(Node),
    /// A member function body kept as a token range until its class is
    /// complete (NAM-6), by its index in `Parser::deferred`.
    Deferred(usize),
}

impl Node {
    /// The first child node with the production `prod`.
    pub fn child(&self, prod: &str) -> Option<&Node> {
        self.kids.iter().find_map(|k| match k {
            Kid::Node(n) if n.prod == prod => Some(n),
            _ => None,
        })
    }
    pub fn children<'a>(&'a self, prod: &'a str) -> impl Iterator<Item = &'a Node> + 'a {
        self.kids.iter().filter_map(move |k| match k {
            Kid::Node(n) if n.prod == prod => Some(n),
            _ => None,
        })
    }
    /// The first node with the production `prod` in this subtree,
    /// depth first, this node included.
    pub fn find(&self, prod: &str) -> Option<&Node> {
        if self.prod == prod {
            return Some(self);
        }
        self.kids.iter().find_map(|k| match k {
            Kid::Node(n) => n.find(prod),
            _ => None,
        })
    }
    /// The token indexes of this subtree, in order.
    pub fn tokens(&self, out: &mut Vec<usize>) {
        for k in &self.kids {
            match k {
                Kid::Tok(t) => out.push(*t),
                Kid::Node(n) => n.tokens(out),
                Kid::Deferred(_) => {}
            }
        }
    }
    /// The texts of the direct token children.
    pub fn words<'t>(&self, toks: &'t [Token]) -> Vec<&'t str> {
        self.kids
            .iter()
            .filter_map(|k| match k {
                Kid::Tok(t) => Some(toks[*t].text.as_str()),
                _ => None,
            })
            .collect()
    }
    /// The tree as an S-expression, for `tidal-next dump` and the tests.
    pub fn sexp(&self, toks: &[Token]) -> String {
        let mut s = format!("({}", self.prod);
        if let Some(e) = self.ext {
            s.push_str(&format!(" ext:{}", e));
        }
        for k in &self.kids {
            s.push(' ');
            match k {
                Kid::Tok(t) => s.push_str(&tok_text(&toks[*t])),
                Kid::Node(n) => s.push_str(&n.sexp(toks)),
                Kid::Deferred(i) => s.push_str(&format!("<deferred {}>", i)),
            }
        }
        s.push(')');
        s
    }
}

fn tok_text(t: &Token) -> String {
    match t.kind {
        TK::Str if t.text.is_empty() => "\"..\"".into(),
        TK::Char if t.text.is_empty() => "'.'".into(),
        _ => t.text.clone(),
    }
}

/// A failed alternative. The parser keeps the furthest failure for the
/// TR-12 diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fail;
pub type PResult<T> = Result<T, Fail>;

/// The nonterminal point of `DESIGN-rewrite.md` section 7: a proposal's
/// alternative for a nonterminal, which the parser tries at the start of
/// that nonterminal.
#[derive(Clone, Copy)]
pub struct Alt {
    pub nonterminal: &'static str,
    pub proposal: &'static str,
    pub parse: fn(&mut Parser) -> PResult<Node>,
}

/// The place in the tokens: a token index and, inside a `>>`, `>=`, or
/// `>>=` that closes a template argument list, the characters used up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mark {
    pos: usize,
    sub: usize,
    depth: usize,
}

/// A member function body kept until its class is complete.
pub struct Deferred {
    pub start: usize,
    pub scopes: Vec<u32>,
    pub declarator: Node,
    pub qual: Qual,
}

/// Where the parse stopped: TR-12, with the token and what the grammar
/// wanted there.
#[derive(Debug, Clone)]
pub struct Stop {
    pub tok: usize,
    pub want: &'static str,
}

pub struct Parser<'a> {
    pub toks: &'a [Token],
    pos: usize,
    sub: usize,
    pub names: &'a mut dyn Names,
    alts: &'a [Alt],
    /// `>` is an operator here; false inside a template argument list
    /// until a bracket opens.
    pub gt_ok: bool,
    /// A position of DEP-20, where a name with a dependent qualifier is a
    /// type without `typename`.
    pub type_only: bool,
    /// The next declaration follows a template head.
    pub template: bool,
    /// A template parameter was named since the flag was cleared.
    pub saw_param: bool,
    /// Inside `decl_specifiers`, where the class's own name before `(`
    /// begins a constructor's declarator.
    pub ctor_check: bool,
    /// The qualifier of the last `declarator_id`.
    pub decl_qual: Qual,
    /// How many class bodies are open; deferred bodies are parsed when
    /// the outermost closes.
    pub class_depth: u32,
    pub deferred: Vec<Deferred>,
    pub done: Vec<Option<Node>>,
    far: usize,
    far_want: &'static str,
    /// A failure inside a deferred body, which no outer alternative can
    /// repair.
    pub stop: Option<Stop>,
}

impl<'a> Parser<'a> {
    pub fn new(toks: &'a [Token], names: &'a mut dyn Names, alts: &'a [Alt]) -> Parser<'a> {
        Parser {
            toks,
            pos: 0,
            sub: 0,
            names,
            alts,
            gt_ok: true,
            type_only: false,
            template: false,
            saw_param: false,
            ctor_check: false,
            decl_qual: Qual::None,
            class_depth: 0,
            deferred: Vec::new(),
            done: Vec::new(),
            far: 0,
            far_want: "a declaration",
            stop: None,
        }
    }

    // -----------------------------------------------------------------
    // The cursor

    pub fn pos(&self) -> usize {
        self.pos
    }
    pub fn tok(&self) -> &'a Token {
        &self.toks[self.pos.min(self.toks.len() - 1)]
    }
    /// The text of the current token as the grammar sees it: for a name
    /// or a punctuator its spelling, for a literal or the end nothing.
    pub fn peek(&self) -> &'a str {
        let t = self.tok();
        match t.kind {
            TK::Ident | TK::Punct => &t.text[self.sub..],
            _ => "",
        }
    }
    pub fn kind(&self) -> TK {
        self.tok().kind
    }
    /// The text of the token `k` places ahead (no half tokens).
    pub fn ahead(&self, k: usize) -> &'a str {
        match self.toks.get(self.pos + k) {
            Some(t) if matches!(t.kind, TK::Ident | TK::Punct) => &t.text,
            _ => "",
        }
    }
    pub fn ahead_kind(&self, k: usize) -> TK {
        self.toks.get(self.pos + k).map_or(TK::Eof, |t| t.kind)
    }
    pub fn at(&self, s: &str) -> bool {
        self.peek() == s
    }
    pub fn at_any(&self, set: &[&str]) -> bool {
        set.contains(&self.peek())
    }
    pub fn at_eof(&self) -> bool {
        self.kind() == TK::Eof
    }
    pub fn span(&self) -> Span {
        self.tok().span
    }
    pub fn mark(&self) -> Mark {
        Mark { pos: self.pos, sub: self.sub, depth: self.names.depth() }
    }
    pub fn reset(&mut self, m: Mark) {
        self.pos = m.pos;
        self.sub = m.sub;
        if self.names.depth() > m.depth {
            let mut s = self.names.stack();
            s.truncate(m.depth);
            self.names.set_stack(s);
        }
    }
    /// Use up the current token.
    pub fn bump(&mut self) -> Kid {
        let k = Kid::Tok(self.pos);
        if !self.at_eof() {
            self.pos += 1;
        }
        self.sub = 0;
        k
    }
    pub fn eat(&mut self, s: &str) -> Option<Kid> {
        self.at(s).then(|| self.bump())
    }
    pub fn expect(&mut self, s: &'static str) -> PResult<Kid> {
        self.eat(s).ok_or_else(|| self.fail(s))
    }
    /// A `>` that closes a template argument or parameter list, taken
    /// from the front of `>>`, `>=`, or `>>=` when need be.
    pub fn close_angle(&mut self) -> PResult<Kid> {
        let p = self.peek();
        if p == ">" {
            return Ok(self.bump());
        }
        if p.starts_with('>') && p.len() > 1 {
            let k = Kid::Tok(self.pos);
            self.sub += 1;
            return Ok(k);
        }
        Err(self.fail("'>'"))
    }
    /// Two `[` tokens in a row, which begin an attribute.
    pub fn at_attr(&self) -> bool {
        self.sub == 0 && self.ahead(0) == "[" && self.ahead(1) == "["
    }
    /// Record a failure here and return it.
    pub fn fail(&mut self, want: &'static str) -> Fail {
        if self.pos > self.far || (self.pos == self.far && self.far_want.is_empty()) {
            self.far = self.pos;
            self.far_want = want;
        }
        Fail
    }
    pub fn furthest(&self) -> Stop {
        self.stop.clone().unwrap_or(Stop { tok: self.far, want: self.far_want })
    }
    /// Try `f`; on failure, go back to where it started.
    pub fn attempt<T>(&mut self, f: impl FnOnce(&mut Self) -> PResult<T>) -> PResult<T> {
        let m = self.mark();
        let r = f(self);
        if r.is_err() {
            self.reset(m);
        }
        r
    }
    /// Try `f` and give `None` when it fails.
    pub fn opt<T>(&mut self, f: impl FnOnce(&mut Self) -> PResult<T>) -> Option<T> {
        self.attempt(f).ok()
    }
    pub fn node(&self, prod: &'static str, span: Span, kids: Vec<Kid>) -> Node {
        Node { prod, ext: None, span, kids }
    }
    /// The extension point: a proposal's alternative for `nonterminal`.
    pub fn ext(&mut self, nonterminal: &'static str) -> Option<Node> {
        if self.alts.is_empty() {
            return None;
        }
        let alts: Vec<Alt> = self.alts.iter().filter(|a| a.nonterminal == nonterminal).copied().collect();
        for a in alts {
            if let Ok(mut n) = self.attempt(|p| (a.parse)(p)) {
                n.ext = Some(a.proposal);
                return Some(n);
            }
        }
        None
    }
    /// Run `f` with `gt_ok` and `type_only` set as given.
    pub fn with<T>(&mut self, gt_ok: bool, type_only: bool, f: impl FnOnce(&mut Self) -> T) -> T {
        let (g, t) = (self.gt_ok, self.type_only);
        self.gt_ok = gt_ok;
        self.type_only = type_only;
        let r = f(self);
        self.gt_ok = g;
        self.type_only = t;
        r
    }
    /// Run `f` inside a bracket, where `>` is an operator again.
    pub fn bracketed<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        let t = self.type_only;
        self.with(true, t, f)
    }

    // -----------------------------------------------------------------
    // The oracle

    pub fn find(&mut self, q: Qual, name: &str) -> Found {
        let f = self.names.find(q, name);
        if f.param {
            self.saw_param = true;
        }
        f
    }
    /// Whether a name under the qualifier is a type. Under a dependent
    /// qualifier, it is a type only in a position of DEP-20.
    pub fn names_type(&mut self, q: Qual, name: &str) -> bool {
        match q {
            Qual::Dependent => self.type_only,
            _ => {
                let f = self.find(q, name);
                f.is_type() || (f.kind == names::Kind::Unknown && q != Qual::None && self.type_only)
            }
        }
    }
    pub fn declared(&mut self, node: &Node, ctx: DeclCtx) -> Vec<u32> {
        self.names.declared(node, self.toks, ctx)
    }
    /// Take the template flag for the declaration being read.
    pub fn take_template(&mut self) -> bool {
        std::mem::replace(&mut self.template, false)
    }

    // -----------------------------------------------------------------
    // Deferred bodies (NAM-6)

    /// Skip a function body that starts here and keep its tokens.
    pub fn defer_body(&mut self, declarator: &Node) -> PResult<Kid> {
        let start = self.pos;
        if self.at("try") {
            self.bump();
        }
        if self.at(":") {
            // ctor_init: up to the `{` that follows `)`, `}`, or `...`.
            self.bump();
            loop {
                if self.at_eof() || self.at(";") {
                    return Err(self.fail("a function body"));
                }
                if self.at("{") && matches!(self.toks[self.pos - 1].text.as_str(), ")" | "}" | "...") {
                    break;
                }
                if self.at_any(&["(", "[", "{"]) {
                    self.skip_balanced()?;
                } else {
                    self.bump();
                }
            }
        }
        if !self.at("{") {
            return Err(self.fail("'{'"));
        }
        self.skip_balanced()?;
        while self.at("catch") {
            self.bump();
            if !self.at("(") {
                return Err(self.fail("'('"));
            }
            self.skip_balanced()?;
            if !self.at("{") {
                return Err(self.fail("'{'"));
            }
            self.skip_balanced()?;
        }
        let i = self.deferred.len();
        self.deferred.push(Deferred { start, scopes: self.names.stack(), declarator: declarator.clone(), qual: self.decl_qual });
        self.done.push(None);
        Ok(Kid::Deferred(i))
    }

    /// Skip a bracketed token range that starts here.
    pub fn skip_balanced(&mut self) -> PResult<()> {
        let mut depth = 0i32;
        loop {
            match self.peek() {
                "(" | "[" | "{" => depth += 1,
                ")" | "]" | "}" => depth -= 1,
                _ if self.at_eof() => return Err(self.fail("a closing bracket")),
                _ => {}
            }
            self.bump();
            if depth <= 0 {
                return Ok(());
            }
        }
    }

    /// Parse the bodies deferred since `from`, with the class complete.
    /// `body` parses one `function_body` with the parameters declared.
    pub fn parse_deferred(&mut self, from: usize, body: fn(&mut Parser, &Node, Qual) -> PResult<Node>) -> PResult<()> {
        let mut i = from;
        while i < self.deferred.len() {
            if self.done[i].is_none() {
                let (start, scopes, decl, qual) = {
                    let d = &self.deferred[i];
                    (d.start, d.scopes.clone(), d.declarator.clone(), d.qual)
                };
                let (pos, sub, saved) = (self.pos, self.sub, self.names.stack());
                let (depth, tmpl) = (self.class_depth, self.take_template());
                self.pos = start;
                self.sub = 0;
                self.class_depth = 0;
                self.names.set_stack(scopes);
                let (far, want) = (self.far, self.far_want);
                self.far = start;
                let r = self.with(true, false, |p| body(p, &decl, qual));
                self.class_depth = depth;
                self.template = tmpl;
                self.names.set_stack(saved);
                match r {
                    Ok(n) => {
                        self.done[i] = Some(n);
                        self.pos = pos;
                        self.sub = sub;
                        self.far = far;
                        self.far_want = want;
                    }
                    Err(f) => {
                        if self.stop.is_none() {
                            self.stop = Some(Stop { tok: self.far, want: self.far_want });
                        }
                        self.pos = pos;
                        self.sub = sub;
                        return Err(f);
                    }
                }
            }
            i += 1;
        }
        Ok(())
    }

    /// Put the deferred bodies into the tree.
    pub fn fill(&mut self, n: &mut Node) {
        for k in n.kids.iter_mut() {
            match k {
                Kid::Node(c) => self.fill(c),
                Kid::Deferred(i) => {
                    if let Some(mut body) = self.done[*i].take() {
                        self.fill(&mut body);
                        *k = Kid::Node(body);
                    }
                }
                Kid::Tok(_) => {}
            }
        }
    }
}

/// The result of parsing one translation unit: the trees of its
/// declarations, or the stop.
pub fn parse_unit(toks: &[Token], names: &mut dyn Names, alts: &[Alt]) -> Result<Vec<Node>, Stop> {
    let mut p = Parser::new(toks, names, alts);
    let mut out = Vec::new();
    while !p.at_eof() {
        match crate::declarations::declaration(&mut p) {
            Ok(mut n) => {
                p.fill(&mut n);
                out.push(n);
            }
            Err(_) => return Err(p.furthest()),
        }
    }
    Ok(out)
}

/// The TR-12 message for a stop.
pub fn stop_message(toks: &[Token], s: &Stop) -> String {
    let t = &toks[s.tok.min(toks.len() - 1)];
    let found = match t.kind {
        TK::Eof => "the end of the file".to_string(),
        TK::Ident | TK::Punct => format!("'{}'", t.text),
        _ => "a literal".to_string(),
    };
    format!("the grammar derives no declaration here: expected {}, found {}", s.want, found)
}

/// Every place that takes a proposal's alternative passes through
/// `Parser::ext` with the nonterminal's name; this is the list of the
/// nonterminals the parser offers.
pub fn token_text(toks: &[Token], k: &Kid) -> Option<String> {
    match k {
        Kid::Tok(t) => Some(toks[*t].text.clone()),
        _ => None,
    }
}
