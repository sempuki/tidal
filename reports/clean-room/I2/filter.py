import sys,re
skip=False
for line in open(sys.argv[1]):
    if re.match(r'^## ', line):
        skip = line.strip().startswith('## Evidence')
    if skip: continue
    if line.lstrip().startswith(':tests:'): continue
    sys.stdout.write(line)
