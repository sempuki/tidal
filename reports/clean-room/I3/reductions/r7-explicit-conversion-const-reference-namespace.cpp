int value = 0;
struct Source { explicit operator int&() const { return value; } };
Source s;
const int& r(s);
int main() { return &r == &value ? 0 : 1; }
