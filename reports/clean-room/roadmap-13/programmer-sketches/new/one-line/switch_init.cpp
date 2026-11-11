// A switch with an init statement.
constexpr int digits(int n) {
  switch (int d = n < 10 ? 1 : n < 100 ? 2 : 3; d) {
    case 1: return 1;
    case 2: return 2;
    default: return d;
  }
}
int main() { return digits(5) + digits(50) + digits(500) - 6; }
