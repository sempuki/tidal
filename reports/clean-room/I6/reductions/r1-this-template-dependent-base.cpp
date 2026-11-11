// Reduction of programs/p02-this-template-member.cpp (I6-11). g++ and clang++ accept; tidal-fe stops:
// unsupported at line 4: check: parse: expected an expression, found ')'
template <class T> struct Derived : T {
  int run() { return this->template get<3>(); }
};
int main() { return 0; }
