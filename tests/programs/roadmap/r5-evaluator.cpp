// tidal-rules: CPY-3 CTOR-10 TPL-24 CVF-3 VIRT-1 VIRT-7 THR-1 THR-4 TRY-1 LSUP-7
// tidal-expect: run
// tidal-note: Roadmap round 5, programmer sketch evaluator.cpp (reports/clean-room/roadmap-5/programmer.md, section 1.5). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// An expression evaluator: tokenizer, recursive-descent parser, a tree of
// owned nodes with virtual evaluation, and errors as exceptions.
// Written as a programmer would, with no #include.

inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }

template <class T> class Owner {
  T* p = nullptr;
public:
  Owner() = default;
  explicit Owner(T* q) : p(q) {}
  Owner(const Owner&) = delete;
  Owner& operator=(const Owner&) = delete;
  Owner(Owner&& o) noexcept : p(o.p) { o.p = nullptr; }
  Owner& operator=(Owner&& o) noexcept {
    if (this != &o) { delete p; p = o.p; o.p = nullptr; }
    return *this;
  }
  ~Owner() { delete p; }
  T* operator->() const { return p; }
  T& operator*() const { return *p; }
  explicit operator bool() const { return p != nullptr; }
  template <class U> Owner(Owner<U>&& o) noexcept : p(o.release()) {}
  T* release() { T* q = p; p = nullptr; return q; }
};

template <class T, class... A> Owner<T> make(A&&... a) {
  return Owner<T>(new T(static_cast<A&&>(a)...));
}

struct Error {
  int position;
  virtual const char* what() const noexcept = 0;
  virtual ~Error() = default;
protected:
  explicit Error(int p) : position(p) {}
};
struct SyntaxError : Error {
  explicit SyntaxError(int p) : Error(p) {}
  const char* what() const noexcept override { return "syntax"; }
};
struct DivideByZero : Error {
  explicit DivideByZero(int p) : Error(p) {}
  const char* what() const noexcept override { return "divide"; }
};

struct Node {
  virtual long eval() const = 0;
  virtual ~Node() = default;
};
struct Number final : Node {
  long value;
  explicit Number(long v) : value(v) {}
  long eval() const override { return value; }
};
struct Negate final : Node {
  Owner<Node> operand;
  explicit Negate(Owner<Node> o) : operand(static_cast<Owner<Node>&&>(o)) {}
  long eval() const override { return -operand->eval(); }
};
struct Binary final : Node {
  char op;
  int at;
  Owner<Node> left, right;
  Binary(char o, int a, Owner<Node> l, Owner<Node> r)
      : op(o), at(a), left(static_cast<Owner<Node>&&>(l)),
        right(static_cast<Owner<Node>&&>(r)) {}
  long eval() const override {
    long a = left->eval();
    long b = right->eval();
    switch (op) {
      case '+': return a + b;
      case '-': return a - b;
      case '*': return a * b;
      case '/':
        if (b == 0) throw DivideByZero(at);
        return a / b;
    }
    return 0;
  }
};

class Parser {
  const char* text;
  int pos = 0;
  char peek() {
    while (text[pos] == ' ') ++pos;
    return text[pos];
  }
  Owner<Node> primary() {
    char c = peek();
    if (c == '(') {
      ++pos;
      Owner<Node> e = sum();
      if (peek() != ')') throw SyntaxError(pos);
      ++pos;
      return e;
    }
    if (c == '-') { ++pos; return make<Negate>(primary()); }
    if (c < '0' || c > '9') throw SyntaxError(pos);
    long v = 0;
    while (text[pos] >= '0' && text[pos] <= '9') v = v * 10 + (text[pos++] - '0');
    return make<Number>(v);
  }
  Owner<Node> product() {
    Owner<Node> e = primary();
    for (char c = peek(); c == '*' || c == '/'; c = peek()) {
      int at = pos++;
      e = make<Binary>(c, at, static_cast<Owner<Node>&&>(e), primary());
    }
    return e;
  }
  Owner<Node> sum() {
    Owner<Node> e = product();
    for (char c = peek(); c == '+' || c == '-'; c = peek()) {
      int at = pos++;
      e = make<Binary>(c, at, static_cast<Owner<Node>&&>(e), product());
    }
    return e;
  }
public:
  explicit Parser(const char* t) : text(t) {}
  Owner<Node> parse() {
    Owner<Node> e = sum();
    if (peek() != '\0') throw SyntaxError(pos);
    return e;
  }
};

long run(const char* s, int& error_at, char& kind) {
  error_at = -1;
  kind = 0;
  try {
    return Parser(s).parse()->eval();
  } catch (const Error& e) {
    error_at = e.position;
    kind = e.what()[0];
    return 0;
  }
}

int main() {
  int at; char kind;
  if (run("1 + 2 * 3", at, kind) != 7) return 1;
  if (run("(1 + 2) * 3", at, kind) != 9) return 2;
  if (run("-(4 - 10) / 2", at, kind) != 3) return 3;
  run("8 / (2 - 2)", at, kind);
  if (kind != 'd' || at != 2) return 4;
  run("1 + * 2", at, kind);
  if (kind != 's' || at != 4) return 5;
  run("(1 + 2", at, kind);
  if (kind != 's') return 6;
  return 0;
}
