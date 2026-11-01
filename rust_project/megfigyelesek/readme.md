# A Rust és MongoDB bson összehasonlítás

A tesztnek a célja az volt, hogy a BSON-ből és a MongoDB által generált illetve dumpolt .bson kiterjesztésű file-ok byte-ra megegyezőek vagy sem.

## Rust:

A Rust-hoz a `rust/converter.rs`-ből a `bson_to_relaxed_json()` és a `data_to-bson()` metódusokat használtam, amelyek generáltak egy json és egy BSON kiterjeszétsű fájlt.

## MongoDB:

A MongoDB-be beimportáltam ugyanazt az értékekkel bíró dokumentumot ezután pedig kiírattam azt a mongosh `db.[kollekció_neve].find()` parancsával és összehasonlítottam a Rust által generálttal.

## Megfigyelés eredménye:

- A megfigyeléshez a **ImHex** nevű alkalmazást használtam ami megmutatta, hogy byte szinten hol van eltérés és utánna a két .json formátumú fájlban könnyen meg lehetett keresni a különbséget.
- Byte szintű eltérést csak az ObjectId-nél volt található ami annak köszönhető, hogy idő alapú generálást használ a típus.

## Futtatás:

```bash
cargo build
cargo run
```

Ekkor létrejön egy `rust/tanulo_relaxed.json` fájl

Ez abban az esetben történik meg, ha a tartalmazza a .bson fájlt amit át szeretnénk alakítani.

- rustfmt
- compressbson column
- milyen tipusokat használ a tároláshoz a bson
- tömörítésre milyen opciók vannak
- snifferként egy rust alkalmazást
- megnézni milyen módszerekkel lehet mérni az adatokat

///snifferhez:
érdemes előbb **libpcap** vagy **pcap**, majd ezt megnézni low-level-el mint például **eBPF** vagy akár legalacsonyabb szinten **ftrace**(LINUX), **ETW**(WINDOWS)
