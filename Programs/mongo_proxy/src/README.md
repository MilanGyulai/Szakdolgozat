# Version 0.1

## Működés:

Minden ami beérkezik azt továbbítja változatlanul.

backend => mongo_sniffer (127.0.0.1:27018) => MongoDB (127.0.0.1:27017) 

Ez a verzió egy intercepting proxy. A backend ehhez a service-hez csatlakozik és
ez továbbítja a MongoDB-nek.

A **try_join!** és a tokio aszikron crate miatt mind a két irányt kezeli párhuzamosan.

A TCP bájtfolyamot szállít, nem üzeneteket. Egy MongoDB üzenet mindig
egy 16 bájtos fejléccel kezdődik (little-endian i32 mezők):

messageLength | requestID | responseTo | opCode

A read_frame pontosan 16 bájtot olvas és kiveszi a teljes hosszt majd beolvassa az
üzenet többi részét.

Minden üzenetet előszőr továbbítok és majd azután dekódolom elemzem őket a kapcsolat stabil
és gyors működése érdekében.

A driverek **OP_MSG** üzenetet használnak. 
### OP_MSG felépítés:
```
messageLength   i32 a teljes üzenet hossza, a fejléccel együtt
requestID       i32 a küldő által adott azonosító
responseTo      i32 kérésnél 0, válasznál a megválaszolt kérés requestID-je
opCode          i32 2013 = OP_MSG
flagBits        u32 kapcsolók
sections[]          egy vagy több szekció
checksum        u32 CRC-32C, csak ha a checksumPresent flag be van állítva
```
A mai driverek **OP_MSG** üzenetet használnak. Ennek törzse egy flag-mezőből és szekciókból áll.
A 0-s típusú szekció tartalmazza a parancsdokumentumot, a parancs neve pedig a 
dokumentum első kulcsa (find, insert, aggregate). Ennek a kulcsnak az értéke 
a collection neve, a $db mező pedig az
adatbázis. A legelső kézfogás még a régi **OP_QUERY** formában is érkezhet, ezért azt is dekódolom.


## Példa kimenet:

```
sniffer listening on 127.0.0.1:27018, forwarding to 127.0.0.1:27017
[conn 1] opened from 127.0.0.1:51781
[conn 1] -> hello admin (324 B)
[conn 1] <- hello admin (313 B) latency 0.669 ms
[conn 1] -> hello admin (176 B)
[conn 3] -> ping admin (105 B)
[conn 3] <- ping admin (38 B) latency 0.097 ms
[conn 3] -> find School.students_imported (244 B)
[conn 3] <- find School.students_imported (923 B) latency 0.957 ms
[conn 4] -> aggregate School.students_imported (226 B)
[conn 4] <- aggregate School.students_imported (137 B) latency 0.688 ms
[conn 1] <- hello admin (313 B) latency 10000.157 ms
[conn 1] <- OP_MSG (313 B)
```