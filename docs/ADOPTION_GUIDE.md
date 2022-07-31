# Compiler adoption guide

## Definitions

### DNA

DNA represents what the NFT is made out of. Every layer that gets added to the NFT gets hashed in a string as seen below. DNA is the literal representation of that NFT and what layers it is made out of.

## Functions

### createDNA

This was a trickier one to understand as it requires knowing what a DNA is. From what I can gather, it loops through each NFTs layers.

A layer comes from grabbing 1 "element" or file, from `/layers/_folder/_filename#rarity` and repeating for each child folder in `/layers`.

It will loop through each element for that NFT, grab the weight which is previously extracted from the filename.

Each weight gets added to a `totalWeight` variable which will equal the total rarity for that NFT. A `random` number between 0 and 1 will be multiplied against the `totalWeight` and then rounded down.

The `for loop` will go through the total number of elements and subtract the weight of that layer from the `random` value. If the `random` variable after the subtraction is less than `0`, a string is pushed to an array.

This function outputs:

```text
0:Black#1.png!1:White#50.png!1:Green#1.png!2:Small#60.png!0:Shapes#100.png!1:Low#40.png!2:Middle#50.png
```

This is the backbone of an NFT. It details what this NFT is made out of. This string, the "DNA" of the NFT gets hashed with `sha1` which results in:

```text
DNA: 0613418f2b3ecfb275c7af39574f9480252f5916
```
