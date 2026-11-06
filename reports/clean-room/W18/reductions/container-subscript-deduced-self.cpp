struct V {
  int items[2]{};
  template <class Self>
  auto&& operator[](this Self&& self, int i) { return static_cast<Self&&>(self).items[i]; }
};
int main() { V v; return v[1]; }
