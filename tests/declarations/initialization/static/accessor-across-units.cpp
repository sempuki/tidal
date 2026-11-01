// tidal-rules: SINIT-3 STO-2
// tidal-expect: run
// tidal-sources: accessor-across-units.tu.cpp
int& table_size();
extern constinit int table_limit;
int slots = table_size() * 2;
int cap = table_limit;
int main() {
  if (slots != 14) return 1;
  if (cap != 64) return 2;
  return 0;
}
