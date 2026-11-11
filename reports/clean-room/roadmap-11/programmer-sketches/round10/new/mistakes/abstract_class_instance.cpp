// Planted mistake: an object of an abstract class.
struct Shape { virtual int area() const = 0; };
int main() { Shape s; return 0; }
