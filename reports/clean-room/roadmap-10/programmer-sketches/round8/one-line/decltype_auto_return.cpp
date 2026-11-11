int g = 3;
decltype(auto) ref() { return (g); }
int main() { ref() = 7; return g == 7 ? 0 : 1; }
