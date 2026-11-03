// tidal-rules: TYN-3
// tidal-expect: run
// tidal-sources: typedef-unnamed-across-units.tu.cpp
// tidal-note: Both units name the unnamed class with the same typedef. The function that takes the class links across the units.
typedef struct { int v; int w; } Cell;
int product(Cell c);
int main() { return product(Cell{3, 4}) == 12 ? 0 : 1; }
