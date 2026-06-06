package calculator

import (
	"fmt"
	"math"
	"strings"
)

// GetPrimeFactors returns a map of prime factors and their counts
func GetPrimeFactors(n int) map[int]int {
	factors := make(map[int]int)
	d := 2
	temp := n
	if temp < 0 {
		temp = -temp
	}
	if temp == 0 {
		return factors
	}
	for d*d <= temp {
		for temp%d == 0 {
			factors[d]++
			temp /= d
		}
		d++
	}
	if temp > 1 {
		factors[temp]++
	}
	return factors
}

// JohnstonComponents represents the breakdown of a ratio into Johnston notation parts
type JohnstonComponents struct {
	Letter string
	Sf     int
	Pm     int
	SL     int
	Ud     int
	E3     int
}

// GetJohnstonComponents calculates the Johnston notation components for ratio x/y
func GetJohnstonComponents(x, y int) (*JohnstonComponents, string) {
	if y == 0 {
		return nil, "denominator cannot be zero"
	}

	px := GetPrimeFactors(x)
	py := GetPrimeFactors(y)

	allPrimes := make(map[int]bool)
	for p := range px {
		allPrimes[p] = true
	}
	for p := range py {
		allPrimes[p] = true
	}

	limitNum := 1
	if len(allPrimes) > 0 {
		for p := range allPrimes {
			if p > limitNum {
				limitNum = p
			}
		}
	}

	if limitNum > 13 {
		return nil, fmt.Sprintf("Sorry - that is a %d-limit pitch!", limitNum)
	}

	// Helper to get prime exponent difference
	diff := func(p int) int {
		return px[p] - py[p]
	}

	// 3-limit
	n3 := diff(3)
	pm3, sf3 := 0, 0
	
	// calc_3_limit equivalent
	qn := n3
	if n3 > 0 {
		for qn > 0 {
			if (qn-1)%7%3 == 2 {
				pm3++
			}
			qn--
		}
	} else if n3 < 0 {
		for qn < 0 {
			k := (qn + 1) % 7
			if k < 0 {
				k += 7
			}
			if k != 0 && k%3 == 0 {
				pm3--
			}
			qn++
		}
	}

	qn = n3
	if n3 > 0 {
		for qn > 0 {
			if (qn-1)%7 == 5 {
				sf3++
			}
			qn--
		}
	} else if n3 < 0 {
		for qn < 0 {
			k := qn % 7
			if k < 0 {
				k += 7
			}
			if k == 5 {
				sf3--
			}
			qn++
		}
	}

	// 5-limit
	n5 := diff(5)
	shift3 := n3 % 7
	if shift3 < 0 {
		shift3 += 7
	}
	baseLetters := "CGDAEBF"
	letters := baseLetters[shift3:] + baseLetters[:shift3]

	s := make([]byte, 7)
	for i := 0; i < 7; i++ {
		s[(2*i)%7] = letters[i]
	}
	sfs5 := string(s)

	pm5, sf5 := 0, 0
	qn = n5
	if n5 > 0 {
		for qn > 0 {
			k := (qn - 1) % 7
			if k < 0 {
				k += 7
			}
			char := sfs5[k]
			if strings.ContainsAny(string(char), "AEB") {
				sf5++
			} else if char == 'D' {
				sf5++
				pm5++
			}
			qn--
		}
	} else if n5 < 0 {
		for qn < 0 {
			k := (qn + 1) % 7
			if k < 0 {
				k += 7
			}
			char := sfs5[k]
			if strings.ContainsAny(string(char), "GCD") {
				sf5--
			} else if char == 'F' {
				sf5--
				pm5--
			}
			qn++
		}
	}

	// 7-limit
	n7 := diff(7)
	pn5 := (4 * n5) % 7
	if pn5 < 0 {
		pn5 += 7
	}
	
	base7Letters := "CBAGFED"
	preshift7 := map[byte]int{'C': 0, 'B': 1, 'A': 2, 'G': 3, 'F': 4, 'E': 5, 'D': 6}
	shift7 := preshift7[letters[pn5]]
	letters7 := base7Letters[shift7:] + base7Letters[:shift7]

	sf7, pm7, sL := 0, 0, 0
	qn = n7
	if n7 > 0 {
		for qn > 0 {
			k := (qn - 1) % 7
			if k < 0 {
				k += 7
			}
			char := letters7[k]
			if strings.ContainsAny(string(char), "GBD") {
				pm7++
				sL++
			} else if strings.ContainsAny(string(char), "CF") {
				sf7--
				sL++
			} else {
				sL++
			}
			qn--
		}
	} else if n7 < 0 {
		for qn < 0 {
			k := (qn + 1) % 7
			if k < 0 {
				k += 7
			}
			char := letters7[k]
			if strings.ContainsAny(string(char), "CFA") {
				pm7--
				sL--
			} else if strings.ContainsAny(string(char), "BE") {
				sf7++
				sL--
			} else {
				sL--
			}
			qn++
		}
	}

	// 11-limit
	n11 := diff(11)
	pn7 := n7 % 7
	if pn7 < 0 {
		pn7 += 7
	}
	base11Letters := "CFBEADG"
	preshift11 := map[byte]int{'C': 0, 'F': 1, 'B': 2, 'E': 3, 'A': 4, 'D': 5, 'G': 6}
	shift11 := preshift11[letters7[pn7]]
	letters11 := base11Letters[shift11:] + base11Letters[:shift11]

	sf11, pm11, ud := 0, 0, 0
	qn = n11
	if n11 > 0 {
		for qn > 0 {
			k := (qn - 1) % 7
			if k < 0 {
				k += 7
			}
			char := letters11[k]
			if char == 'A' {
				pm11--
				ud++
			} else if char == 'F' {
				sf11--
				pm11--
				ud++
			} else {
				ud++
			}
			qn--
		}
	} else if n11 < 0 {
		for qn < 0 {
			k := (qn + 1) % 7
			if k < 0 {
				k += 7
			}
			char := letters11[k]
			if char == 'D' {
				pm11++
				ud--
			} else if char == 'B' {
				sf11++
				pm11++
				ud--
			} else {
				ud--
			}
			qn++
		}
	}

	// 13-limit
	n13 := diff(13)
	pn11 := n11 % 7
	if pn11 < 0 {
		pn11 += 7
	}
	base13Letters := "CAFDBGE"
	preshift13 := map[byte]int{'C': 0, 'A': 1, 'F': 2, 'D': 3, 'B': 4, 'G': 5, 'E': 6}
	shift13 := preshift13[letters11[pn11]]
	letters13 := base13Letters[shift13:] + base13Letters[:shift13]

	sf13, pm13, e3 := 0, 0, 0
	qn = n13
	if n13 > 0 {
		for qn > 0 {
			k := (qn - 1) % 7
			if k < 0 {
				k += 7
			}
			char := letters13[k]
			if strings.ContainsAny(string(char), "CDG") {
				sf13--
				e3++
			} else if char == 'F' {
				sf13--
				pm13--
				e3++
			} else {
				e3++
			}
			qn--
		}
	} else if n13 < 0 {
		for qn < 0 {
			k := (qn + 1) % 7
			if k < 0 {
				k += 7
			}
			char := letters13[k]
			if strings.ContainsAny(string(char), "EAB") {
				sf13++
				e3--
			} else if char == 'D' {
				sf13++
				pm13++
				e3--
			} else {
				e3--
			}
			qn++
		}
	}

	pn13 := n13 % 7
	if pn13 < 0 {
		pn13 += 7
	}
	nameBase := string(letters13[pn13])
	sf := sf3 + sf5 + sf7 + sf11 + sf13
	pm := pm3 + pm5 + pm7 + pm11 + pm13

	return &JohnstonComponents{
		Letter: nameBase,
		Sf:     sf,
		Pm:     pm,
		SL:     sL,
		Ud:     ud,
		E3:     e3,
	}, ""
}

// NoteName returns the Johnston note name for ratio x/y
func NoteName(x, y int, quiet bool) string {
	comp, errStr := GetJohnstonComponents(x, y)
	if errStr != "" {
		if !quiet {
			fmt.Println(errStr)
		}
		return errStr
	}

	var parts []string
	parts = append(parts, comp.Letter)

	if comp.Sf > 0 {
		parts = append(parts, strings.Repeat("#", comp.Sf))
	} else if comp.Sf < 0 {
		parts = append(parts, strings.Repeat("b", -comp.Sf))
	}

	if comp.SL > 0 {
		parts = append(parts, strings.Repeat("7", comp.SL))
	} else if comp.SL < 0 {
		parts = append(parts, strings.Repeat("L", -comp.SL))
	}

	if comp.Ud > 0 {
		parts = append(parts, strings.Repeat("^", comp.Ud))
	} else if comp.Ud < 0 {
		parts = append(parts, strings.Repeat("v", -comp.Ud))
	}

	if comp.E3 > 0 {
		parts = append(parts, strings.Repeat("3", comp.E3))
	} else if comp.E3 < 0 {
		parts = append(parts, strings.Repeat("e", -comp.E3))
	}

	if comp.Pm > 0 {
		parts = append(parts, strings.Repeat("+", comp.Pm))
	} else if comp.Pm < 0 {
		parts = append(parts, strings.Repeat("-", -comp.Pm))
	}

	name := strings.Join(parts, "")

	if !quiet {
		fmt.Println(name)
	}

	absAny := func(vals ...int) bool {
		for _, v := range vals {
			if math.Abs(float64(v)) > 4 {
				return true
			}
		}
		return false
	}

	if absAny(comp.Sf, comp.Pm, comp.SL, comp.Ud, comp.E3) {
		aka := fmt.Sprintf("(%s %d %d %d %d %d)", comp.Letter, comp.Sf, comp.SL, comp.Ud, comp.E3, comp.Pm)
		if !quiet {
			fmt.Println(aka)
		}
		name += "\n" + aka
	}

	return name
}
