// tidal-rules: CONV-16
// tidal-expect: accept
// tidal-note: The conversion happens for a function argument, with an array of double.
double values[4];
bool take(double (*)[]) { return true; }
bool used = take(&values);
