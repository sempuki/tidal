using F = int();
int a() { return 0; }
struct Source { operator F&() const { return a; } };
int main() { F& r = Source{}; return r(); }
