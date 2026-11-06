// OVL-15 with ICS-45: direct initialization of const int& takes the explicit operator int&().
int value = 5;
struct Source { explicit operator int&() const { return value; } };
Source s;
const int& r(s);
int main() { return &r == &value ? 0 : 1; }
