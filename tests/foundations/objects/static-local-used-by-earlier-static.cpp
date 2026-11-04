// tidal-rules: OBJ-12
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: first's object is constructed before log's object, so log's object ends first. The destructor of first's object then calls log, and control passes through the declaration of the ended object. No tool in the matrix stops the run, so the test checks only that the program compiles. The front end's interpreter must stop the run with OBJ-12 (decision 0053).
struct Log { int lines = 0; ~Log() {} };
Log& log() { static Log l; return l; }
struct First { ~First() { ++log().lines; } };
First& first() { static First f; return f; }
int main() {
  first();
  return log().lines;
}
