// An array of a class that has a default constructor and a constructor
// template, as a type-erased callable has. One object of the class runs.
struct F {
  int* p = nullptr;
  F() = default;
  template <class G> F(G) {}
};
int main() { F h[2]; return h[1].p ? 1 : 0; }
