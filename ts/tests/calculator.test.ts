import { noteName } from '../src/calculator';

describe('noteName', () => {
    test('examples from README', () => {
        expect(noteName(49, 55, true)).toBe("Cb77v+");
        expect(noteName(31, 16, true)).toBe("Sorry - that is a 31-limit pitch!");
    });

    test('common ratios', () => {
        expect(noteName(3, 2, true)).toBe("G");
        expect(noteName(5, 4, true)).toBe("E");
        expect(noteName(7, 4, true)).toBe("Bb7");
        expect(noteName(11, 8, true)).toBe("F^");
        expect(noteName(13, 8, true)).toBe("Ab3");
    });
});
