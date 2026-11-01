// tidal-rules: CX-2 CX-1
// tidal-expect: reject
// tidal-error-clang: must be initialized by a constant expression
// tidal-error-gcc: is not a constant expression
// tidal-error-edg: must have a constant value
struct Holder { const int& n; };
int main() {
  constexpr Holder local{4};
  return local.n;
}
