struct A { int a; };
struct B { int b; };

template <class... Bases>
struct all : Bases... {
  all(int v) : Bases{v}... {}
};

int main() {
  all<A, B> x(5);
  return x.a + x.b == 10 ? 0 : 1;
}
