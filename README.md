# 13-limit-johnston

13-limit calculator for note names in extended just intonation using ben johnstons system turns a ratio xy into a 13-limit johnston note name returns the prime limit larger than 13 implicated for such ratios 

## usage

### command line interface

you can run the calculator directly from the command line

```bash
python3 calculator.py 49/55
# output cb77v+

python3 calculator.py 3 2
# output g
```

### python library

you can also use it as a library in your own python projects

```python
from calculator import note_name

name = note_name(49, 55)
print(name) # cb77v+
```

### go version

```bash
cd go
go run cmd/calculator/main.go 49/55
```

### typescript version

```bash
cd ts
npm start 49/55
```

### rust version

```bash
cd rust
cargo run -- 49/55
```

## johnston notation symbols

outputs consist of a letter name plus some combination of #b7l^v3e+-

# b + - and 7 correspond directly to their johnston notation symbols

l = upside down 7 ^ = up arrow v = down arrow 3 = 13 e = upside down 13

## references

for more on the johnston notation system see fonville john ben johnstons extended just intonation a guide for interpreters perspectives of new music 292 1991 106137 httpswwwjstororgstable833435

for more on the algorithm used here see appendix a in johnston timothy ernest 13-limit extended just intonation in ben johnstons string quartet no 7 and toby twinings chrysalid requiem gradualtract dma diss university of illinois 2008 httpswwwproquestcomopenviewf9d0a27d5b2d659c700412aa241f7265
