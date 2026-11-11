"""States of both ledgers and the commit rows whose Record names no commit hash."""
import re
for f in ['reports/clean-room/rust-frontend/ledger.md', 'reports/clean-room/ledger.md']:
    rows = [l for l in open('/home/ryanm/Code/tidal/' + f) if re.match(r'^\| [A-Z]', l) and not l.startswith('| Finding')]
    st, nohash = {}, []
    for l in rows:
        x = [y.strip() for y in l.strip().strip('|').split('|')]
        st[x[2]] = st.get(x[2], 0) + 1
        if x[2] == 'commit' and not re.search(r'\b[0-9a-f]{7,40}\b', x[3]):
            nohash.append(x[0])
    print(f, len(rows), st, 'commit rows with no hash:', nohash)
