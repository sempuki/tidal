// calc.cpp: an evaluator for integer + - * / over a fixed array of tokens.
// Nodes are a tagged union. Operators are called through function pointers.
// Spec rules used: UNI-P1, UNI-4, UNI-7, CE-6 (union), DCL-1, PFX-7
// (function pointers), MUL-9, MUL-10, ADD-9 (checks), THR-1, TRY-2, CE-8.

enum class Tok { Num, Plus, Minus, Star, Slash, End };

struct Token { Tok kind; int value; };   // value only for Tok::Num
struct CalcError { int code; };   // 1 division by zero, 2 overflow, 3 syntax

// The range of int. unsigned has the width of int (outside knowledge).
constexpr int int_max = static_cast<int>(~0u >> 1);
constexpr int int_min = -int_max - 1;

// Each operator checks its operands before it computes, because the
// overflow or the division by zero would itself be undefined.
constexpr int add(int a, int b) {
  if (b > 0 ? a > int_max - b : a < int_min - b) throw CalcError{2};
  return a + b;
}
constexpr int sub(int a, int b) {
  if (b < 0 ? a > int_max + b : a < int_min + b) throw CalcError{2};
  return a - b;
}
constexpr int mul(int a, int b) {
  if (a == 0 || b == 0) return 0;
  bool over = a > 0 ? (b > 0 ? a > int_max / b : b < int_min / a)
                     : (b > 0 ? a < int_min / b : a < int_max / b);
  if (over) throw CalcError{2};
  return a * b;
}
constexpr int divide(int a, int b) {
  if (b == 0) throw CalcError{1};                  // MUL-9
  if (a == int_min && b == -1) throw CalcError{2}; // MUL-10
  return a / b;
}

using BinaryFn = int (*)(int, int);   // pointer to function (int, int)
enum class NodeKind { Leaf, Binary };
struct BinaryNode { BinaryFn fn; int left; int right; };   // pool indexes

struct Node {
  NodeKind kind;
  union {
    int value;          // active when kind is Leaf
    BinaryNode op;      // active when kind is Binary
  };
};

constexpr int max_nodes = 32;

class Parser {
public:
  constexpr explicit Parser(const Token* tokens) : toks_(tokens) {}
  // expr := term (('+' | '-') term)*
  constexpr int expr() {
    int left = term();
    while (peek() == Tok::Plus || peek() == Tok::Minus) {
      BinaryFn fn = next().kind == Tok::Plus ? add : sub;
      left = binary(fn, left, term());
    }
    return left;
  }
  constexpr int eval(int i) const {
    const Node& n = nodes_[i];
    switch (n.kind) {
      case NodeKind::Leaf: return n.value;
      case NodeKind::Binary: return n.op.fn(eval(n.op.left), eval(n.op.right));
    }
    throw CalcError{3};
  }
  constexpr bool at_end() const { return toks_[pos_].kind == Tok::End; }
private:
  // term := number (('*' | '/') number)*
  constexpr int term() {
    int left = leaf();
    while (peek() == Tok::Star || peek() == Tok::Slash) {
      BinaryFn fn = next().kind == Tok::Star ? mul : divide;
      left = binary(fn, left, leaf());
    }
    return left;
  }
  constexpr int leaf() {
    if (peek() != Tok::Num) throw CalcError{3};
    Node& n = alloc();
    n.kind = NodeKind::Leaf;
    n.value = next().value;              // makes value the active member
    return count_ - 1;
  }
  constexpr int binary(BinaryFn fn, int left, int right) {
    Node& n = alloc();
    n.kind = NodeKind::Binary;
    n.op = BinaryNode{fn, left, right};  // makes op the active member
    return count_ - 1;
  }
  constexpr Node& alloc() {
    if (count_ == max_nodes) throw CalcError{3};
    return nodes_[count_++];
  }
  constexpr Tok peek() const { return toks_[pos_].kind; }
  constexpr const Token& next() { return toks_[pos_++]; }
  const Token* toks_;
  int pos_ = 0;
  Node nodes_[max_nodes] = {};
  int count_ = 0;
};

constexpr int evaluate(const Token* tokens) {
  Parser p(tokens);
  int root = p.expr();
  if (!p.at_end()) throw CalcError{3};
  return p.eval(root);
}

// Returns minus the error code instead of the value if evaluation throws.
int evaluate_or_code(const Token* tokens) {
  try { return evaluate(tokens); }
  catch (const CalcError& e) { return -e.code; }   // TRY-2
}

constexpr Token N(int v) { return Token{Tok::Num, v}; }
constexpr Token P{Tok::Plus, 0}, M{Tok::Minus, 0}, S{Tok::Star, 0},
    D{Tok::Slash, 0}, E{Tok::End, 0};

constexpr Token sample[] = {N(2), P, N(3), S, N(4), M, N(8), D, N(2), E};
static_assert(evaluate(sample) == 10);               // 2 + 12 - 4
constexpr Token left_assoc[] = {N(20), M, N(5), M, N(3), E};
static_assert(evaluate(left_assoc) == 12);           // (20 - 5) - 3

struct Case { Token tokens[6]; int expect; };

int main() {
  const Case cases[] = {
      {{N(7), D, N(0), E}, -1},                      // division by zero
      {{N(-9), D, N(2), E}, -4},                     // toward zero (MUL-P2)
      {{N(int_max), P, N(1), E}, -2},                // overflow
      {{N(int_min), D, N(-1), E}, -2},               // overflow (MUL-10)
      {{N(65536), S, N(-32768), E}, int_min},        // fits exactly
      {{N(65536), S, N(32768), E}, -2},              // one too big
      {{N(1), P, E}, -3},                            // syntax error
  };
  if (evaluate_or_code(sample) != 10) return 1;
  int n = 2;
  for (const Case& c : cases) {
    if (evaluate_or_code(c.tokens) != c.expect) return n;
    ++n;
  }
  return 0;
}
