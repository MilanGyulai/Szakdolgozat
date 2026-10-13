# SZAKDOLGOZAT FELADAT

Gyulai Milán (JEH60C) BSc programtervező informatikus jelölt részére.

- A szakdolgozat tárgyköre: MongoDB, optimalizálás

- A szakdolgozat címe: A MongoDB wire protokolljának optimalizálási lehetőségei

## A feladat részletezése:

A MongoDB egy igen elterjedt dokumentum orientált adatbáziskezelő rendszer.
Ennél a szerver és kliens közötti kommunikáció egyik módja a wire protokoll használata.

A szakdolgozat azt vizsgálja, hogy milyen lehetőségek vannak a protokoll hatékonyabbá tételére.

- Vizsgálatra kerül a hálózaton átküldött adatok mennyiségének csökkentése egy plusz, beiktatott, transzparens réteg segítségével.

- A dolgozat foglalkozik továbbá a MongoDB adatbázis performanciájának, memóriaigényének optimalizálási lehetőségeivel is.

A vizsgálatokhoz az alkalmazások elsősorban Rust programozási nyelven készülnek.

## Használt technológiák és verziók:

- rustc 1.99.0

  **[dependencies]**
  - bson = { version = "3", features = ["serde", "serde_json-1"] }
  - serde_json = "1"
  - tokio = { version = "1", features = ["full", "rt"] }
  - serde = { version = "1.0.229", features = ["derive"] }
  - http = "1.5.0"

  **[dependencies.mongodb]**
  - version = "3.9.1"

- cargo 1.99.0
- mongosh 2.12.0
- MongoDB 9.0.2

## Hivatkozások

1. https://doc.rust-lang.org/book/
2. https://www.mongodb.com/docs/manual/reference/mongodb-wire-protocol/
3. https://docs.rs/serde_json/latest/serde_json/
4. https://docs.rs/bson/latest/bson/
