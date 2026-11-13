# Backend

## Észrevételek, hibák

### mongodb és bson crate-k
A mongodb és bson file implementációi között különbség található,
így a projektben érdemes lenne ezt megnézni melyik optimálisabb.
Egyelőre csak a **Cargo.toml** fájlban van megoldva egységesítve.

A projektben közvetlenül is használom a bson crate-et (bson = "3"),
a driver viszont alapból a 2.15-ös verziót használta.
Így a programban két különböző bson verzió volt egyszerre,
és azok típusai (Document, ObjectId) a Rust szemében különböző típusok,
még ha ugyanúgy hívják őket.

A két verzió közül a 3.x-et választottam most.

### szerver cím megadása:

A szerver címet érdemes ipv4 címként megadni
különben feloldódik ipv6 címre és kikerüli a sniffert, mert az ipv4-es címen figyel

- mongodb://localhost:27017/?directConnection=true

- mongodb://127.0.0.1:27018/?directConnection=true

## Parancsok:

TODO