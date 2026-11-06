// matrix.cpp: a fixed-size matrix with a two-argument operator[] (OPR-3),
// multiplication, and a transpose. Checked in constant evaluation
// (SA-1, CE-P1) and at run time (TR-2).
namespace std { using size_t = decltype(sizeof 0); }  // LSUP example

template <class T, std::size_t R, std::size_t C>
struct Matrix {
  T cell[R][C] = {};  // every scalar part has a value (CE-21)

  constexpr T& operator[](std::size_t i, std::size_t j) { return cell[i][j]; }
  constexpr const T& operator[](std::size_t i, std::size_t j) const { return cell[i][j]; }

  friend constexpr bool operator==(const Matrix&, const Matrix&) = default;  // DCMP-1
};

template <class T, std::size_t R, std::size_t K, std::size_t C>
constexpr Matrix<T, R, C> operator*(const Matrix<T, R, K>& a, const Matrix<T, K, C>& b) {
  Matrix<T, R, C> out;
  for (std::size_t i = 0; i < R; ++i)
    for (std::size_t j = 0; j < C; ++j) {
      T sum = 0;
      for (std::size_t k = 0; k < K; ++k) sum += a[i, k] * b[k, j];
      out[i, j] = sum;
    }
  return out;
}

template <class T, std::size_t R, std::size_t C>
constexpr Matrix<T, C, R> transpose(const Matrix<T, R, C>& m) {
  Matrix<T, C, R> out;
  for (std::size_t i = 0; i < R; ++i)
    for (std::size_t j = 0; j < C; ++j) out[j, i] = m[i, j];
  return out;
}

// Build a matrix from a generator, as Haskell's listArray / generate would.
template <class T, std::size_t R, std::size_t C, class F>
constexpr Matrix<T, R, C> generate(F f) {
  Matrix<T, R, C> out;
  for (std::size_t i = 0; i < R; ++i)
    for (std::size_t j = 0; j < C; ++j) out[i, j] = f(i, j);
  return out;
}

using M23 = Matrix<int, 2, 3>;
using M32 = Matrix<int, 3, 2>;
using M22 = Matrix<int, 2, 2>;

constexpr M23 a{{{1, 2, 3}, {4, 5, 6}}};
constexpr M32 b{{{7, 8}, {9, 10}, {11, 12}}};

static_assert(a[1, 2] == 6);
static_assert((a * b) == M22{{{58, 64}, {139, 154}}});
static_assert(transpose(a) == M32{{{1, 4}, {2, 5}, {3, 6}}});
static_assert(transpose(transpose(a)) == a);
// (A B)^T == B^T A^T
static_assert(transpose(a * b) == transpose(b) * transpose(a));

constexpr auto ident = generate<int, 3, 3>([](std::size_t i, std::size_t j) { return i == j ? 1 : 0; });
static_assert(a * ident == a);

int main(int argc, char**) {
  // Run time: the values depend on argc, so no constant evaluation occurs.
  int s = argc;  // 1 when run with no arguments
  M23 x = generate<int, 2, 3>([s](std::size_t i, std::size_t j) { return int(i * 3 + j) + s; });
  M32 y = transpose(x);
  M22 p = x * y;
  // x = [[1,2,3],[4,5,6]] when s == 1, so p = [[14,32],[32,77]]
  if (s == 1 && !(p == M22{{{14, 32}, {32, 77}}})) return 1;
  if (!(transpose(transpose(x)) == x)) return 2;
  if (!(transpose(x * y) == transpose(y) * transpose(x))) return 3;
  p[0, 1] = 0;
  if (p[0, 1] != 0) return 4;
  return 0;
}
