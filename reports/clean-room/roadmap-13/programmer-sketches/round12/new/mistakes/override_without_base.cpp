// Planted mistake: override on a function that overrides nothing (a parameter type differs).
struct Shape { virtual double area(int scale) const { return scale; } virtual ~Shape() = default; };
struct Sq : Shape { double area(long scale) const override { return scale; } };
int main() { Sq s; return 0; }
