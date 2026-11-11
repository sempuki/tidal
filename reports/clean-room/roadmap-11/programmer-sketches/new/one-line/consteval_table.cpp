// A consteval function that builds a table used at run time.
struct Table { int v[8]; };
consteval Table squares() { Table t{}; for (int i = 0; i < 8; ++i) t.v[i] = i * i; return t; }
constexpr Table table = squares();
int main(int argc, char**) { return table.v[argc + 2] == 9 ? 0 : 1; }
