// The same as friend_enclosing_class_in_nested_class_of_template.cpp
// with no template. tidal-fe gives TYN-7 (an incomplete class).
class Ring {
  int v[2] = {1, 0};
 public:
  struct Iter {
    friend class Ring;
    const Ring* r;
    int get() const { return r->v[1]; }
  };
};
int main() {
  Ring r;
  Ring::Iter it{&r};
  return it.get();
}
