// W9: a visitor over an expression tree.
// Rules used: VIRT-1, VIRT-3, VIRT-8, DER-3, DER-14, ACC-4, ACC-9, CE-1.

struct Num;
struct Add;
struct Mul;

// The visitor interface. The destructor is protected and not virtual,
// so no code can delete a visitor through this base (DER-14, ACC-4).
struct Visitor {
  constexpr virtual void visit(const Num&) = 0;
  constexpr virtual void visit(const Add&) = 0;
  constexpr virtual void visit(const Mul&) = 0;
protected:
  ~Visitor() = default;
};

struct Expr {
  virtual ~Expr() = default;
  constexpr virtual void accept(Visitor& v) const = 0;
};

struct Num final : Expr {
  int value;
  constexpr explicit Num(int v) : value(v) {}
  constexpr void accept(Visitor& v) const override { v.visit(*this); }
};

struct Add final : Expr {
  const Expr& left;
  const Expr& right;
  constexpr Add(const Expr& l, const Expr& r) : left(l), right(r) {}
  constexpr void accept(Visitor& v) const override { v.visit(*this); }
};

struct Mul final : Expr {
  const Expr& left;
  const Expr& right;
  constexpr Mul(const Expr& l, const Expr& r) : left(l), right(r) {}
  constexpr void accept(Visitor& v) const override { v.visit(*this); }
};

// Evaluates the tree.
struct Eval final : Visitor {
  int result = 0;
  constexpr void visit(const Num& n) override { result = n.value; }
  constexpr void visit(const Add& a) override {
    a.left.accept(*this);
    int l = result;
    a.right.accept(*this);
    result = l + result;
  }
  constexpr void visit(const Mul& m) override {
    m.left.accept(*this);
    int l = result;
    m.right.accept(*this);
    result = l * result;
  }
};

// Walks every node and does nothing. Derived visitors override only
// the cases they care about.
struct Walker : Visitor {
  constexpr void visit(const Num&) override {}
  constexpr void visit(const Add& a) override {
    a.left.accept(*this);
    a.right.accept(*this);
  }
  constexpr void visit(const Mul& m) override {
    m.left.accept(*this);
    m.right.accept(*this);
  }
};

// Counts leaves. The using-declaration keeps the other visit overloads
// visible in this class (DER-3).
struct CountLeaves final : Walker {
  int leaves = 0;
  using Walker::visit;
  constexpr void visit(const Num&) override { ++leaves; }
};

constexpr int eval(const Expr& e) {
  Eval v;
  e.accept(v);
  return v.result;
}

constexpr int leaves(const Expr& e) {
  CountLeaves c;
  e.accept(c);
  return c.leaves;
}

// (2 + 3) * (4 + 1) * 2
constexpr int check() {
  Num two{2}, three{3}, four{4}, one{1};
  Add a{two, three};
  Add b{four, one};
  Mul ab{a, b};
  Mul all{ab, two};
  if (eval(all) != 50) return 1;
  if (leaves(all) != 5) return 2;
  CountLeaves c;
  c.visit(a);                       // Walker::visit(const Add&), via using
  if (c.leaves != 2) return 3;
  return 0;
}

static_assert(check() == 0);

int main() {
  return check();
}
