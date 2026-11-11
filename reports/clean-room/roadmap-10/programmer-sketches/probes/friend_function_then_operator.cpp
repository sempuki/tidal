// A friend function declaration in place of the friend class.
struct Box {
  int v_ = 7;
  friend int peek(const Box&);
  int operator+(int x) const { return v_ + x; }
};
int peek(const Box& b) { return b.v_; }
int main() {
  Box b;
  return (b + 1 == 8 && peek(b) == 7) ? 0 : 1;
}
