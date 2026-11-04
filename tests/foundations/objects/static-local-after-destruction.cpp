// tidal-rules: OBJ-12
// tidal-expect: accept
// tidal-note: late is constructed before main and count's object at the first call, so the object ends first. The destructor of late then passes through the declaration of the ended object. No tool in the matrix stops the run, so the test checks only that the program compiles.
struct Counter { int n = 0; ~Counter() {} };
Counter& count() { static Counter c; return c; }
struct Late { ~Late() { ++count().n; } } late;
int main() { return count().n; }
