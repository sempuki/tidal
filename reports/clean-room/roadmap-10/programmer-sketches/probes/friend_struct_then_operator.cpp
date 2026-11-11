// As friend_class_then_overloaded_operator.cpp, with a friend struct
// declared in place of class, and no forward declaration.
struct Box {
  int v_ = 7;
  friend struct Other;
  int operator+(int x) const { return v_ + x; }
};
int main() {
  Box b;
  return b + 1 == 8 ? 0 : 1;
}
