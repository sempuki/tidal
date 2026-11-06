int value = 0;
struct ToInt { operator int&() { return value; } };
int main() { ToInt s; int& r = static_cast<int&>(s); return r; }
