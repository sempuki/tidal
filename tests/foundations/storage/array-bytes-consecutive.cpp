// tidal-rules: STG-2
// tidal-expect: run
// tidal-note: An array of a standard-layout class. The array occupies sizeof bytes from its address, and each element follows the one before.
struct Cell { char tag; short value; };
int main() {
  Cell cells[4] = {};
  const unsigned char* start = reinterpret_cast<const unsigned char*>(&cells);
  const unsigned char* last = reinterpret_cast<const unsigned char*>(&cells[3]);
  bool at_start = reinterpret_cast<const unsigned char*>(&cells[0]) == start;
  bool fills = last + sizeof(Cell) == start + sizeof cells;
  bool sizes = sizeof cells == 4 * sizeof(Cell);
  return at_start && fills && sizes ? 0 : 1;
}
