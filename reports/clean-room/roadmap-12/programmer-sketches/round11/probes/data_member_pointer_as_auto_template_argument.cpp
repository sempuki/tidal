struct Counter { int n = 0; };
template <auto Mem> int get(Counter& obj) { return obj.*Mem; }
int main() {
  Counter c{2};
  return get<&Counter::n>(c) == 2 ? 0 : 1;
}
