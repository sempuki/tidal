// tidal-rules: LAM-16
// tidal-expect: run
// tidal-note: The recursion runs at run time, and the lambda has a capture.
int main() {
  int base = 1;
  auto sum = [base](this auto self, int n) -> int {
    return n == 0 ? base : n + self(n - 1);
  };
  return sum(4) == 11 ? 0 : 1;
}
