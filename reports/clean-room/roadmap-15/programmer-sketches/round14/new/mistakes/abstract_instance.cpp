struct Shape { virtual int sides() const = 0; virtual ~Shape() = default; };
struct Tri : Shape {};
int main() { Tri t; return t.sides() - 3; }
