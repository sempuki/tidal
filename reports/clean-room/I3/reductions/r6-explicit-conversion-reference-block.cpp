int value = 0;
struct Source { explicit operator int&() const { return value; } };
int main() { Source s; int& r(s); return &r == &value ? 0 : 1; }
