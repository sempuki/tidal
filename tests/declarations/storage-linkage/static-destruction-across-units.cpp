// tidal-rules: STO-10
// tidal-expect: run
// tidal-sources: static-destruction-across-units.tu.cpp
// tidal-note: a and b are defined in two units, so either can be constructed first (SINIT-3). Each constructor records the order. The destructors run in the reverse of that order, and the second destructor checks it. The throw from a destructor ends the program only when the order is wrong.
struct Note { char c; explicit Note(char k); ~Note(); };
char built[2];
int nbuilt = 0;
char gone[2];
int ngone = 0;
Note::Note(char k) : c(k) { built[nbuilt++] = k; }
Note::~Note() {
  gone[ngone++] = c;
  if (ngone == 2 && !(gone[0] == built[1] && gone[1] == built[0])) throw 1;
}
Note a('a');
int main() { return 0; }
