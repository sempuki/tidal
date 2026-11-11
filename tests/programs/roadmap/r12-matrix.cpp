// tidal-rules: DCMP-1 OPR-3 CLS-13
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: constant value
// tidal-note: Roadmap round 12, programmer sketch matrix.cpp (reports/clean-room/roadmap-12/programmer.md, section 1.4). The first try builds clean on GCC and Clang and is the last version (decision 0061). Q-0403. The program evaluates a defaulted == over a member array of arrays in a static_assert. GCC and Clang accept. EDG rejects the constant evaluation. The rules are those of the defaulted ==, the subscript with two arguments, and the explicit object parameter that the report names for the program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Matrix<T, R, C> with expression templates: a concept Expr, lazy Sum,
// Scaled, and Product nodes that hold their operands by reference or by
// value as each needs, a C++23 subscript with two arguments, a row view
// through an explicit object parameter, transpose, identity, and checks
// of products and powers in static_assert and at run time.
using size_t = decltype(sizeof 0);

template <class T> struct remove_cvref { using type = T; };
template <class T> struct remove_cvref<const T> { using type = T; };
template <class T> struct remove_cvref<T&> : remove_cvref<T> {};
template <class T> struct remove_cvref<T&&> : remove_cvref<T> {};
template <class T> using remove_cvref_t = typename remove_cvref<T>::type;

template <class E>
concept Expr = requires(const E& e) {
  typename E::value_type;
  E::rows + E::cols;
  e[0, 0];
};

template <class T, size_t R, size_t C>
struct Matrix {
  using value_type = T;
  static constexpr size_t rows = R, cols = C;
  T a[R][C]{};

  constexpr Matrix() = default;
  template <Expr E>
    requires(E::rows == R && E::cols == C)
  constexpr Matrix(const E& e) {
    for (size_t i = 0; i < R; ++i)
      for (size_t j = 0; j < C; ++j) a[i][j] = e[i, j];
  }
  constexpr T& operator[](size_t i, size_t j) { return a[i][j]; }
  constexpr const T& operator[](size_t i, size_t j) const { return a[i][j]; }

  // A row as a span-like view; const or not as the object is.
  template <class Self>
  constexpr auto row(this Self&& self, size_t i) {
    struct Row {
      decltype(&self.a[0][0]) p;
      constexpr auto& operator[](size_t j) const { return p[j]; }
      constexpr auto begin() const { return p; }
      constexpr auto end() const { return p + C; }
    };
    return Row{self.a[i]};
  }

  static constexpr Matrix identity() requires(R == C) {
    Matrix m;
    for (size_t i = 0; i < R; ++i) m[i, i] = T(1);
    return m;
  }
  friend constexpr bool operator==(const Matrix&, const Matrix&) = default;
};

// Leaf operands are held by reference, nodes by value.
template <class E> using hold_t = const E&;

template <Expr L, Expr Rt>
  requires(L::rows == Rt::rows && L::cols == Rt::cols)
struct Sum {
  using value_type = typename L::value_type;
  static constexpr size_t rows = L::rows, cols = L::cols;
  hold_t<L> l;
  hold_t<Rt> r;
  constexpr value_type operator[](size_t i, size_t j) const { return l[i, j] + r[i, j]; }
};

template <Expr E>
struct Scaled {
  using value_type = typename E::value_type;
  static constexpr size_t rows = E::rows, cols = E::cols;
  value_type k;
  hold_t<E> e;
  constexpr value_type operator[](size_t i, size_t j) const { return k * e[i, j]; }
};

template <Expr L, Expr Rt>
  requires(L::cols == Rt::rows)
struct Product {
  using value_type = typename L::value_type;
  static constexpr size_t rows = L::rows, cols = Rt::cols;
  hold_t<L> l;
  hold_t<Rt> r;
  constexpr value_type operator[](size_t i, size_t j) const {
    value_type s{};
    for (size_t k = 0; k < L::cols; ++k) s += l[i, k] * r[k, j];
    return s;
  }
};

template <Expr E>
struct Transposed {
  using value_type = typename E::value_type;
  static constexpr size_t rows = E::cols, cols = E::rows;
  hold_t<E> e;
  constexpr value_type operator[](size_t i, size_t j) const { return e[j, i]; }
};

template <Expr L, Expr Rt> constexpr auto operator+(const L& l, const Rt& r) { return Sum<L, Rt>{l, r}; }
template <Expr L, Expr Rt> constexpr auto operator*(const L& l, const Rt& r) { return Product<L, Rt>{l, r}; }
template <Expr E> constexpr auto operator*(typename E::value_type k, const E& e) { return Scaled<E>{k, e}; }
template <Expr E> constexpr auto transpose(const E& e) { return Transposed<E>{e}; }

template <Expr E>
constexpr Matrix<typename E::value_type, E::rows, E::cols> eval(const E& e) { return e; }

template <class T, size_t N>
constexpr Matrix<T, N, N> power(Matrix<T, N, N> m, unsigned k) {
  auto r = Matrix<T, N, N>::identity();
  while (k) {
    if (k & 1) r = r * m;
    m = m * m;
    k >>= 1;
  }
  return r;
}

constexpr Matrix<long, 2, 2> fibm = [] {
  Matrix<long, 2, 2> m;
  m[0, 0] = 1; m[0, 1] = 1; m[1, 0] = 1;
  return m;
}();
static_assert(power(fibm, 10)[0, 1] == 55);
static_assert(power(fibm, 50)[0, 1] == 12586269025L);

constexpr bool check() {
  Matrix<int, 2, 3> a;
  int v = 1;
  for (size_t i = 0; i < 2; ++i)
    for (auto& x : a.row(i)) x = v++;  // 1 2 3 / 4 5 6
  Matrix<int, 3, 2> t = transpose(a);
  Matrix<int, 2, 2> p = a * t;  // 14 32 / 32 77
  if (p[0, 0] != 14 || p[0, 1] != 32 || p[1, 1] != 77) return false;
  Matrix<int, 2, 3> s = a + 2 * a;
  if (s[1, 2] != 18) return false;
  const Matrix<int, 2, 3>& ca = a;
  int sum = 0;
  for (int x : ca.row(1)) sum += x;
  return sum == 15 && eval(a + a) == eval(2 * a);
}
static_assert(check());

int main() {
  int fails = 0;
  if (!check()) ++fails;
  Matrix<double, 3, 3> m = Matrix<double, 3, 3>::identity();
  m[0, 2] = 0.5;
  Matrix<double, 3, 3> q = m * m;
  if (q[0, 2] != 1.0 || q[1, 1] != 1.0) ++fails;
  // A sum of a product keeps the product node by value inside the sum.
  Matrix<double, 3, 3> w = m * m + m;
  if (w[0, 2] != 1.5 || w[2, 2] != 2.0) ++fails;
  return fails;
}
