// A double narrows to int in a braced initializer.
struct Point { int x, y; };
int main() { double d = 1.5; Point p{d, 2}; return p.y - 2; }
