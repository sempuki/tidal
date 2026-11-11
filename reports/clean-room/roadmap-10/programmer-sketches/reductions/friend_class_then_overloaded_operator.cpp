// A class that names a friend class. Each overloaded operator on it stops
// the front end; without the friend declaration, the program runs.
class Other;
class Box {
  int v_ = 7;
  friend class Other;
public:
  int& operator*() { return v_; }
  int operator+(int x) const { return v_ + x; }
  explicit operator bool() const { return v_ != 0; }
};
int main() {
  Box b;
  int fails = 0;
  if (*b != 7) ++fails;
  if (b + 1 != 8) ++fails;
  if (!b) ++fails;
  return fails;
}
