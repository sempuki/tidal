// tidal-rules: SINIT-3 SINIT-2
// tidal-expect: run
// tidal-sources: across-units-class-member.tu.cpp
// tidal-note: table is a class-type variable of the other unit. A read of table.size before its constructor runs is undefined (OBJ-2, Q-0175), so the reader first checks the bool table_ready, which holds false or true by SINIT-3. slots is 0 or 28, depending on the order of the units.
int table_slots();
int slots = table_slots();
int main() { return slots == 0 || slots == 28 ? 0 : 1; }
