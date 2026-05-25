# 13-limit-Johnston

13-limit calculator for note names in extended just intonation using Ben Johnston's system. Turns a ratio x/y into a 13-limit Johnston note name; returns the prime limit larger than 13 implicated for such ratios. 

## Usage

### Command Line Interface

You can run the calculator directly from the command line:

```bash
python3 calculator.py 49/55
# Output: Cb77v+

python3 calculator.py 3 2
# Output: G
```

### Python Library

You can also use it as a library in your own Python projects:

```python
from calculator import note_name

name = note_name(49, 55)
print(name) # Cb77v+
```

## Johnston Notation Symbols

Outputs consist of a letter name plus some combination of #b7L^v3e+-.

#, b, +, -, and 7 correspond directly to their Johnston notation symbols.

L = upside down 7; ^ = ↑, v = ↓, 3 = 13, e = upside down 13.

## References

For more on the Johnston notation system see: Fonville, John. “Ben Johnston's Extended Just Intonation: A Guide for Interpreters.” Perspectives of New Music 29/2 (1991): 106–137. https://www.jstor.org/stable/833435

For more on the algorithm used here, see Appendix A in: Johnston, Timothy Ernest. “13-Limit Extended Just Intonation in Ben Johnston’s String Quartet No. 7 and Toby Twining’s Chrysalid Requiem, ‘Gradual/Tract.’” D.M.A. diss., University of Illinois, 2008. https://www.proquest.com/openview/f9d0a27d5b2d659c700412aa241f7265/
