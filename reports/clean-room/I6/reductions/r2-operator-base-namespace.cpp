// Reduction of programs/p05-operator-adl-base-namespace.cpp (I6-12). No template is needed.
// g++ and clang++ run it and exit 0; tidal-fe: rejected [ADD-3] at line 5: no operator '+' for class operand
namespace geo { struct Shape {}; int operator+(Shape, Shape) { return 0; } }
struct Circle : geo::Shape {};
int main() { return Circle{} + Circle{}; }
