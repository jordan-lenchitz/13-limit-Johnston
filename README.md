# 13-limit-johnston

13-limit calculator for note names in extended just intonation using ben johnston's system. turns a ratio x/y into a 13-limit johnston note name; returns the prime limit larger than 13 implicated for such ratios. 

## usage

### command line interface

you can run the calculator directly from the command line:

```bash
python3 calculator.py 49/55
# output: cb77v+

python3 calculator.py 3 2
# output: g
```

### python library

you can also use it as a library in your own python projects:

```python
from calculator import note_name

name = note_name(49, 55)
print(name) # cb77v+
```

## johnston notation symbols

outputs consist of a letter name plus some combination of #b7l^v3e+-.

#, b, +, -, and 7 correspond directly to their johnston notation symbols.

l = upside down 7; ^ = ↑, v = ↓, 3 = 13, e = upside down 13.

## references

for more on the johnston notation system see: fonville, john. “ben johnston's extended just intonation: a guide for interpreters.” perspectives of new music 29/2 (1991): 106–137. https://www.jstor.org/stable/833435

for more on the algorithm used here, see appendix a in: johnston, timothy ernest. “13-limit extended just intonation in ben johnston’s string quartet no. 7 and toby twining’s chrysalid requiem, ‘gradual/tract.’” d.m.a. diss., university of illinois, 2008. https://www.proquest.com/openview/f9d0a27d5b2d659c700412aa241f7265/
