// Probe from matrix.cpp: a defaulted == over a member int[2][3] in a
// constant expression (DCMP-1, Q-0403; the test eq-array-of-arrays-member-constant.cpp holds it). GCC, Clang, and tidal-fe accept; EDG
// reports an access one past the end of an array of 3.
struct M {
  int a[2][3]{};
  friend constexpr bool operator==(const M&, const M&) = default;
};
static_assert(M{} == M{});
int main() { return M{} == M{} ? 0 : 1; }
