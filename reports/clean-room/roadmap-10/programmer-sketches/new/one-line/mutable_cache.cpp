// A mutable member that a const member function fills as a cache.
struct Poly {
  int coef[3] = {1, 2, 3};
  mutable bool cached = false;
  mutable int sum = 0;
  int total() const { if (!cached) { sum = coef[0] + coef[1] + coef[2]; cached = true; } return sum; }
};
int main() {
  const Poly p;
  return p.total() == 6 && p.cached && p.total() == 6 ? 0 : 1;
}
