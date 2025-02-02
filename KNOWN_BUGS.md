# Dirt doc

This document tracks known bugs or issues with the project as it goes through development.

## `parker::auction::Auction`'s `Display` implementation displays differently at start of bidding when North is dealing vs other dealers

Empty/new auctions for E/S/W dealers display like the following:

```
+-- N --+-- E --+-- S --+-- W --+
|       |       |       |       |
+-------+-------+-------+-------+
```

But auctions with North dealing only display the header lines:

```
+-- N --+-- E --+-- S --+-- W --+
```

This is because `Auction`'s `Display` imlementation inserts empty spaces for seats "before" the dealer's to pad out the first row (but there aren't any seats before `Seat::North`).