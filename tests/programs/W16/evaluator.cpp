// tidal-rules: TYN-1 TYP-1 TYP-4 CPY-1 CPY-2 CPY-3 CPY-8 CPY-11 TPL-24 CONV-14 DED-8 VIRT-1 VIRT-3 VIRT-7 VIRT-8 VIRT-12 THR-1 THR-4 TRY-1 TRY-2 MUL-9 MUL-10 SW-2 SW-7
// tidal-expect: run
// tidal-note: Write suite W16, program evaluator.cpp (reports/clean-room/W16/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// W16 evaluator: an integer expression evaluator written from the Tidal
// specification alone. No header is included.
//
// Grammar:
//   expr   := term { ('+' | '-') term }
//   term   := unary { ('*' | '/' | '%') unary }
//   unary  := '-' unary | primary
//   primary:= number | '(' expr ')'
//
// Rules used (Tidal IDs):
//   owners and moves        CPY-P2, CPY-P4, CPY-2, CPY-3, CPY-8, NEW-P2
//   converting owner        TPL-24 (member template), CPY-1/CPY-2 (a
//                           template is never a copy or move constructor),
//                           CONV-14 (derived pointer to base pointer)
//   virtual eval            VIRT-1, VIRT-3, VIRT-8, VIRT-7, VIRT-12
//   errors                  THR-1, THR-4, TRY-1, TRY-2, LSUP-8 example
//   arithmetic              MUL-9, MUL-10 (checked before each operation)

using i64 = long long;  // TYN-1: an alias makes no new type

constexpr i64 kMax = 9223372036854775807LL;  // TYP-1: long long has 64 bits
constexpr i64 kMin = -kMax - 1;

// ---------------------------------------------------------------- errors

enum class ErrorKind { Lex, Parse, DivideByZero, Overflow };

struct EvalError {
  int position;
  explicit EvalError(int at) noexcept : position(at) {}
  virtual ErrorKind kind() const noexcept = 0;
  virtual const char* message() const noexcept = 0;
  virtual ~EvalError() = default;
};

struct LexError : EvalError {
  char bad;
  LexError(int at, char c) noexcept : EvalError(at), bad(c) {}
  ErrorKind kind() const noexcept override { return ErrorKind::Lex; }
  const char* message() const noexcept override { return "unexpected character"; }
};

struct ParseError : EvalError {
  const char* what;
  ParseError(int at, const char* w) noexcept : EvalError(at), what(w) {}
  ErrorKind kind() const noexcept override { return ErrorKind::Parse; }
  const char* message() const noexcept override { return what; }
};

struct DivideByZero : EvalError {
  explicit DivideByZero(int at) noexcept : EvalError(at) {}
  ErrorKind kind() const noexcept override { return ErrorKind::DivideByZero; }
  const char* message() const noexcept override { return "division by zero"; }
};

struct Overflow : EvalError {
  explicit Overflow(int at) noexcept : EvalError(at) {}
  ErrorKind kind() const noexcept override { return ErrorKind::Overflow; }
  const char* message() const noexcept override { return "result out of range"; }
};

// ------------------------------------------------------------ owner type

// Owner<T> holds one object made by new and deletes it once.
template <class T>
class Owner {
  T* ptr;

public:
  Owner() noexcept : ptr(nullptr) {}
  explicit Owner(T* p) noexcept : ptr(p) {}

  Owner(const Owner&) = delete;             // one owner (NEW-P2)
  Owner& operator=(const Owner&) = delete;

  Owner(Owner&& other) noexcept : ptr(other.release()) {}  // CPY-2

  // Owner<Derived> -> Owner<Base>. A member template, so it is never the
  // move constructor (CPY-2). U* converts to T* only if T is a base
  // (CONV-14), so other uses do not compile.
  template <class U>
  Owner(Owner<U>&& other) noexcept : ptr(other.release()) {}

  Owner& operator=(Owner&& other) noexcept {  // CPY-3
    if (this != &other) {
      T* old = ptr;
      ptr = other.release();
      delete old;  // NEW-4: null does nothing
    }
    return *this;
  }

  ~Owner() { delete ptr; }  // VIRT-7: the base destructor is virtual

  T* release() noexcept {
    T* p = ptr;
    ptr = nullptr;
    return p;
  }
  T* get() const noexcept { return ptr; }
  T* operator->() const noexcept { return ptr; }
  T& operator*() const noexcept { return *ptr; }
  explicit operator bool() const noexcept { return ptr != nullptr; }
};

// make<T>(args...) is new T(args...) wrapped in an owner. The parameters
// are forwarding references (DED-8), and static_cast<Args&&> passes each
// on with its value category (CPY-P4, LSUP-10 example).
template <class T, class... Args>
Owner<T> make(Args&&... args) {
  return Owner<T>(new T(static_cast<Args&&>(args)...));
}

template <class T>
Owner<T>&& move_from(Owner<T>& o) noexcept {  // std::move by hand (CPY-P4)
  return static_cast<Owner<T>&&>(o);
}

// ------------------------------------------------------- checked arithmetic

i64 checked_add(i64 a, i64 b, int at) {
  if ((b > 0 && a > kMax - b) || (b < 0 && a < kMin - b)) throw Overflow(at);
  return a + b;
}

i64 checked_sub(i64 a, i64 b, int at) {
  if ((b < 0 && a > kMax + b) || (b > 0 && a < kMin + b)) throw Overflow(at);
  return a - b;
}

i64 checked_mul(i64 a, i64 b, int at) {
  if (a == 0 || b == 0) return 0;
  bool bad = false;
  if (a > 0) {
    bad = (b > 0) ? a > kMax / b : b < kMin / a;
  } else {
    bad = (b > 0) ? a < kMin / b : b < kMax / a;
  }
  if (bad) throw Overflow(at);  // MUL-10
  return a * b;
}

i64 checked_div(i64 a, i64 b, int at, bool remainder) {
  if (b == 0) throw DivideByZero(at);          // MUL-9
  if (a == kMin && b == -1) throw Overflow(at);  // MUL-10
  return remainder ? a % b : a / b;           // MUL-P2: toward zero
}

// ------------------------------------------------------------------ tree

struct Node {
  int position;
  explicit Node(int at) : position(at) {}
  virtual i64 eval() const = 0;  // VIRT-8: Node is abstract
  virtual int size() const = 0;  // number of nodes, for the checks
  virtual ~Node() = default;     // VIRT-P3: delete through Node* is fine
};

struct Number : Node {
  i64 value;
  Number(int at, i64 v) : Node(at), value(v) {}
  i64 eval() const override { return value; }
  int size() const override { return 1; }
};

struct Negate : Node {
  Owner<Node> operand;
  Negate(int at, Owner<Node> e) : Node(at), operand(move_from(e)) {}
  i64 eval() const override {
    i64 v = operand->eval();
    if (v == kMin) throw Overflow(position);
    return -v;
  }
  int size() const override { return 1 + operand->size(); }
};

struct Binary : Node {
  char op;
  Owner<Node> left, right;
  Binary(int at, char o, Owner<Node> l, Owner<Node> r)
      : Node(at), op(o), left(move_from(l)), right(move_from(r)) {}
  i64 eval() const override {
    // Left first, then right: two statements, so the order is fixed.
    i64 a = left->eval();
    i64 b = right->eval();
    switch (op) {
      case '+': return checked_add(a, b, position);
      case '-': return checked_sub(a, b, position);
      case '*': return checked_mul(a, b, position);
      case '/': return checked_div(a, b, position, false);
      case '%': return checked_div(a, b, position, true);
    }
    throw ParseError(position, "unknown operator");
  }
  int size() const override { return 1 + left->size() + right->size(); }
};

// ------------------------------------------------------------- tokenizer

enum class Tok { Number, Plus, Minus, Star, Slash, Percent, LParen, RParen, End };

struct Token {
  Tok kind;
  int position;
  i64 value;  // only for Tok::Number
};

class Lexer {
  const char* text;
  int pos = 0;

  static bool is_digit(char c) { return c >= '0' && c <= '9'; }
  static bool is_space(char c) { return c == ' ' || c == '\t' || c == '\n'; }

public:
  explicit Lexer(const char* t) : text(t) {}

  Token next() {
    while (is_space(text[pos])) ++pos;
    int start = pos;
    char c = text[pos];
    if (c == '\0') return Token{Tok::End, start, 0};
    if (is_digit(c)) {
      i64 v = 0;
      while (is_digit(text[pos])) {
        int d = text[pos] - '0';
        if (v > (kMax - d) / 10) throw Overflow(start);
        v = v * 10 + d;
        ++pos;
      }
      return Token{Tok::Number, start, v};
    }
    ++pos;
    switch (c) {
      case '+': return Token{Tok::Plus, start, 0};
      case '-': return Token{Tok::Minus, start, 0};
      case '*': return Token{Tok::Star, start, 0};
      case '/': return Token{Tok::Slash, start, 0};
      case '%': return Token{Tok::Percent, start, 0};
      case '(': return Token{Tok::LParen, start, 0};
      case ')': return Token{Tok::RParen, start, 0};
      default: throw LexError(start, c);
    }
  }
};

// ---------------------------------------------------------------- parser

class Parser {
  Lexer lexer;
  Token current;

  void advance() { current = lexer.next(); }

  static char op_char(Tok t) {
    switch (t) {
      case Tok::Plus: return '+';
      case Tok::Minus: return '-';
      case Tok::Star: return '*';
      case Tok::Slash: return '/';
      case Tok::Percent: return '%';
      default: return '?';
    }
  }

  Owner<Node> expr() {
    Owner<Node> left = term();
    while (current.kind == Tok::Plus || current.kind == Tok::Minus) {
      int at = current.position;
      char op = op_char(current.kind);
      advance();
      Owner<Node> right = term();
      left = make<Binary>(at, op, move_from(left), move_from(right));
    }
    return left;  // CPY-P4: a returned local is treated as an rvalue
  }

  Owner<Node> term() {
    Owner<Node> left = unary();
    while (current.kind == Tok::Star || current.kind == Tok::Slash ||
           current.kind == Tok::Percent) {
      int at = current.position;
      char op = op_char(current.kind);
      advance();
      Owner<Node> right = unary();
      left = make<Binary>(at, op, move_from(left), move_from(right));
    }
    return left;
  }

  Owner<Node> unary() {
    if (current.kind == Tok::Minus) {
      int at = current.position;
      advance();
      return make<Negate>(at, unary());  // Owner<Negate> -> Owner<Node>
    }
    return primary();
  }

  Owner<Node> primary() {
    if (current.kind == Tok::Number) {
      Owner<Number> n = make<Number>(current.position, current.value);
      advance();
      return n;  // an lvalue Owner<Number>; see the report
    }
    if (current.kind == Tok::LParen) {
      advance();
      Owner<Node> inner = expr();
      if (current.kind != Tok::RParen) throw ParseError(current.position, "expected )");
      advance();
      return inner;
    }
    throw ParseError(current.position, "expected a number or (");
  }

public:
  explicit Parser(const char* text) : lexer(text), current{Tok::End, 0, 0} {
    advance();
  }

  Owner<Node> parse() {
    Owner<Node> tree = expr();
    if (current.kind != Tok::End) throw ParseError(current.position, "extra input");
    return tree;
  }
};

// ------------------------------------------------------------ the checks

struct Outcome {
  bool ok;
  i64 value;
  ErrorKind error;
  int position;
  int nodes;
};

Outcome run(const char* text) {
  try {
    Parser parser(text);
    Owner<Node> tree = parser.parse();
    i64 v = tree->eval();
    return Outcome{true, v, ErrorKind::Lex, -1, tree->size()};
  } catch (const EvalError& e) {  // TRY-2: a public base matches
    return Outcome{false, 0, e.kind(), e.position, 0};
  }
}

struct Case {
  const char* text;
  bool ok;
  i64 value;        // when ok
  ErrorKind error;  // when not ok
  int position;     // of the error, or -1
  int nodes;        // when ok
};

constexpr Case cases[] = {
    {"1", true, 1, ErrorKind::Lex, -1, 1},
    {"1 + 2 * 3", true, 7, ErrorKind::Lex, -1, 5},
    {"(1 + 2) * 3", true, 9, ErrorKind::Lex, -1, 5},
    {"10 - 4 - 3", true, 3, ErrorKind::Lex, -1, 5},
    {"100 / 7", true, 14, ErrorKind::Lex, -1, 3},
    {"-7 / 2", true, -3, ErrorKind::Lex, -1, 4},
    {"-7 % 2", true, -1, ErrorKind::Lex, -1, 4},
    {"7 % -2", true, 1, ErrorKind::Lex, -1, 4},
    {"--5", true, 5, ErrorKind::Lex, -1, 3},
    {"2 * (3 + 4) * (5 - 1)", true, 56, ErrorKind::Lex, -1, 9},
    {"((((42))))", true, 42, ErrorKind::Lex, -1, 1},
    {"  8\t*\n 8 ", true, 64, ErrorKind::Lex, -1, 3},
    {"9223372036854775807", true, kMax, ErrorKind::Lex, -1, 1},
    {"-9223372036854775807 - 1", true, kMin, ErrorKind::Lex, -1, 4},
    {"1 / 0", false, 0, ErrorKind::DivideByZero, 2, 0},
    {"5 % (3 - 3)", false, 0, ErrorKind::DivideByZero, 2, 0},
    {"9223372036854775807 + 1", false, 0, ErrorKind::Overflow, 20, 0},
    {"9223372036854775808", false, 0, ErrorKind::Overflow, 0, 0},
    {"-9223372036854775807 - 1 - 1", false, 0, ErrorKind::Overflow, 25, 0},
    {"(-9223372036854775807 - 1) / -1", false, 0, ErrorKind::Overflow, 27, 0},
    {"-(-9223372036854775807 - 1)", false, 0, ErrorKind::Overflow, 0, 0},
    {"3037000500 * 3037000500", false, 0, ErrorKind::Overflow, 11, 0},
    {"1 + ", false, 0, ErrorKind::Parse, 4, 0},
    {"(1 + 2", false, 0, ErrorKind::Parse, 6, 0},
    {"1 2", false, 0, ErrorKind::Parse, 2, 0},
    {")", false, 0, ErrorKind::Parse, 0, 0},
    {"4 $ 4", false, 0, ErrorKind::Lex, 2, 0},
};

// A check on the owner itself: moves leave the source empty, and a
// converted owner deletes through the base.
int destroyed = 0;

struct Probe : Node {
  explicit Probe(int at) : Node(at) {}
  i64 eval() const override { return 0; }
  int size() const override { return 1; }
  ~Probe() override { ++destroyed; }
};

bool owner_checks() {
  bool good = true;
  {
    Owner<Probe> a = make<Probe>(0);
    Owner<Node> b = move_from(a);  // converts Owner<Probe> -> Owner<Node>
    good = good && !a && b && destroyed == 0;
    Owner<Node> c;
    c = move_from(b);              // move assignment
    good = good && !b && c;
    c = move_from(c);              // self move keeps the object
    good = good && c && destroyed == 0;
  }
  good = good && destroyed == 1;   // deleted once, through Node*
  return good;
}

int main() {
  int failures = 0;
  for (const Case& c : cases) {
    Outcome r = run(c.text);
    bool match = r.ok == c.ok;
    if (match && c.ok) match = r.value == c.value && r.nodes == c.nodes;
    if (match && !c.ok) match = r.error == c.error && r.position == c.position;
    if (!match) ++failures;
  }
  if (!owner_checks()) ++failures;
  return failures == 0 ? 0 : 1;
}
