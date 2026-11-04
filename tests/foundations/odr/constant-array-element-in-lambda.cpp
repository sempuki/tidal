// tidal-rules: ODR-2 ODR-3
// tidal-expect: run
// tidal-expect-gcc: reject
// tidal-error-gcc: .table. is not captured
// tidal-note: Q-0367. The subscript reads the value of one element of a constant array, so the lambda does not use table. Clang and EDG accept. GCC rejects.
int main() {
  constexpr int table[] = {1, 2, 3};
  auto second = [] { return table[1]; };
  return second() - 2;
}
