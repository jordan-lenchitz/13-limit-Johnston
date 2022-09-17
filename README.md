# 13-limit-Johnston

13-limit calculator for note names in extended just intonation using Ben Johnston's system.

Turns a ratio x/y into a 13-limit Johnston note name; returns the prime limit larger than 13 implicated for such ratios.

Outputs consist of a letter name plus some combination of #b7L^v3e+-.

#, b, +, -, and 7 correspond directly to their Johnston notation symbols.

L = upside down 7; ^ = ↑, v = ↓, 3 = 13, e = upside down 13.

Example: convert the pitch 49/55 into a Johnston note name

>note_name(49, 55)

Cb77v+

Example: attempt to convert the pitch 31/16 into a Johnston note name

>note_name(31, 16)

Sorry - that is a 31-limit pitch!

For more on the Johnston notation system see: Fonville, John. “Ben Johnston's Extended Just Intonation: A Guide for Interpreters.” Perspectives of New Music 29/2 (1991): 106–137. https://www.jstor.org/stable/833435

For more on the algorithm used here, see Appendix A in: Johnston, Timothy Ernest. “13-Limit Extended Just Intonation in Ben Johnston’s String Quartet No. 7 and Toby Twining’s Chrysalid Requiem, ‘Gradual/Tract.’” D.M.A. diss., University of Illinois, 2008. https://www.proquest.com/openview/f9d0a27d5b2d659c700412aa241f7265/
