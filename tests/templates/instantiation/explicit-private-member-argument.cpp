// tidal-rules: INST-34
// tidal-expect: accept
// tidal-note: The explicit instantiations name the private type Host::Secret and the address of the private member Host::value. Access is not checked there.
class Host { struct Secret {}; static int value; };
int Host::value = 0;
template <int* P> int read() { return *P; }
template <class T> struct Box { void run() {} };
template int read<&Host::value>();
template void Box<Host::Secret>::run();
