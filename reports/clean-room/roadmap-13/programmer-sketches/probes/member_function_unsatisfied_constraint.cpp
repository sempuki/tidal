template <bool B> struct Q {
  int value = 0;
  constexpr int get() const requires B { return value; }
};
int main() { Q<false> q; return q.get(); }
