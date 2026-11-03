int main() {
  auto c = [k = 0]() mutable { return ++k; };
  auto d = c;          // copy construction
  d = c;               // assignment: LAM-26
  return d();
}
