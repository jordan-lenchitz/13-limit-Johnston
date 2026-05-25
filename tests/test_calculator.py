import sys
import io
from calculator import note_name

def test_note_name():
    # Helper to capture stdout
    def get_note_name(x, y):
        captured_output = io.StringIO()
        sys.stdout = captured_output
        note_name(x, y)
        sys.stdout = sys.__stdout__
        return captured_output.getvalue().strip()

    # Examples from README
    assert get_note_name(49, 55) == "Cb77v+"
    assert get_note_name(31, 16) == "Sorry - that is a 31-limit pitch!"
    
    # Common ratios
    assert get_note_name(3, 2) == "G"
    assert get_note_name(5, 4) == "E"
    assert get_note_name(7, 4) == "Bb7"
    assert get_note_name(11, 8) == "F^"
    assert get_note_name(13, 8) == "Ab3"
    
    # Some other common ones if I can guess them, 
    # but let's start with these.
    print("Tests passed!")

if __name__ == "__main__":
    test_note_name()
