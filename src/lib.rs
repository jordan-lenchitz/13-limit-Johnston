use std::collections::HashMap;

pub fn get_prime_factors(n: i64) -> HashMap<i64, i64> {
    let mut factors = HashMap::new();
    let mut d = 2;
    let mut temp = n.abs();
    if temp == 0 {
        return factors;
    }
    while d * d <= temp {
        while temp % d == 0 {
            *factors.entry(d).or_insert(0) += 1;
            temp /= d;
        }
        d += 1;
    }
    if temp > 1 {
        *factors.entry(temp).or_insert(0) += 1;
    }
    factors
}

#[derive(Debug)]
pub struct JohnstonComponents {
    pub letter: String,
    pub sf: i32,
    pub pm: i32,
    pub sl: i32,
    pub ud: i32,
    pub e3: i32,
    pub a17: i32,
    pub a19: i32,
    pub a23: i32,
    pub a29: i32,
    pub a31: i32,
}

pub fn get_johnston_components(x: i64, y: i64) -> Result<JohnstonComponents, String> {
    if y == 0 {
        return Err("denominator cannot be zero".to_string());
    }

    let px = get_prime_factors(x);
    let py = get_prime_factors(y);

    let mut all_primes: Vec<i64> = px.keys().chain(py.keys()).cloned().collect();
    all_primes.sort();
    all_primes.dedup();

    let limit_num = if all_primes.is_empty() {
        1
    } else {
        *all_primes.iter().max().unwrap()
    };

    if limit_num > 31 {
        return Err(format!("Sorry - that is a {}-limit pitch!", limit_num));
    }

    let diff = |p: i64| *px.get(&p).unwrap_or(&0) - *py.get(&p).unwrap_or(&0);

    // 3-limit
    let n3 = diff(3);
    let mut pm3 = 0;
    let mut sf3 = 0;

    let mut qn = n3;
    if n3 > 0 {
        while qn > 0 {
            if (qn - 1) % 7 % 3 == 2 { pm3 += 1; }
            qn -= 1;
        }
    } else if n3 < 0 {
        while qn < 0 {
            let mut k = (qn + 1) % 7;
            if k < 0 { k += 7; }
            if k != 0 && k % 3 == 0 { pm3 -= 1; }
            qn += 1;
        }
    }

    qn = n3;
    if n3 > 0 {
        while qn > 0 {
            if (qn - 1) % 7 == 5 { sf3 += 1; }
            qn -= 1;
        }
    } else if n3 < 0 {
        while qn < 0 {
            let mut k = qn % 7;
            if k < 0 { k += 7; }
            if k == 5 { sf3 -= 1; }
            qn += 1;
        }
    }

    // 5-limit
    let n5 = diff(5);
    let mut shift3 = n3 % 7;
    if shift3 < 0 { shift3 += 7; }
    let base_letters = "CGDAEBF";
    let letters = format!("{}{}", &base_letters[shift3 as usize..], &base_letters[..shift3 as usize]);

    let mut s = vec![' '; 7];
    for i in 0..7 {
        s[(2 * i) % 7] = letters.chars().nth(i).unwrap();
    }
    let sfs5: String = s.into_iter().collect();

    let mut pm5 = 0;
    let mut sf5 = 0;
    qn = n5;
    if n5 > 0 {
        while qn > 0 {
            let mut k = (qn - 1) % 7;
            if k < 0 { k += 7; }
            let char = sfs5.chars().nth(k as usize).unwrap();
            if "AEB".contains(char) { sf5 += 1; }
            else if char == 'D' { sf5 += 1; pm5 += 1; }
            qn -= 1;
        }
    } else if n5 < 0 {
        while qn < 0 {
            let mut k = (qn + 1) % 7;
            if k < 0 { k += 7; }
            let char = sfs5.chars().nth(k as usize).unwrap();
            if "GCD".contains(char) { sf5 -= 1; }
            else if char == 'F' { sf5 -= 1; pm5 -= 1; }
            qn += 1;
        }
    }

    // 7-limit
    let n7 = diff(7);
    let mut pn5 = (4 * n5) % 7;
    if pn5 < 0 { pn5 += 7; }

    let base7_letters = "CBAGFED";
    let preshift7 = |c: char| match c { 'C' => 0, 'B' => 1, 'A' => 2, 'G' => 3, 'F' => 4, 'E' => 5, 'D' => 6, _ => 0 };
    let shift7 = preshift7(letters.chars().nth(pn5 as usize).unwrap());
    let letters7 = format!("{}{}", &base7_letters[shift7 as usize..], &base7_letters[..shift7 as usize]);

    let mut sf7 = 0;
    let mut pm7 = 0;
    let mut sl = 0;
    qn = n7;
    if n7 > 0 {
        while qn > 0 {
            let mut k = (qn - 1) % 7;
            if k < 0 { k += 7; }
            let char = letters7.chars().nth(k as usize).unwrap();
            if "GBD".contains(char) { pm7 += 1; sl += 1; }
            else if "CF".contains(char) { sf7 -= 1; sl += 1; }
            else { sl += 1; }
            qn -= 1;
        }
    } else if n7 < 0 {
        while qn < 0 {
            let mut k = (qn + 1) % 7;
            if k < 0 { k += 7; }
            let char = letters7.chars().nth(k as usize).unwrap();
            if "CFA".contains(char) { pm7 -= 1; sl -= 1; }
            else if "BE".contains(char) { sf7 += 1; sl -= 1; }
            else { sl -= 1; }
            qn += 1;
        }
    }

    // 11-limit
    let n11 = diff(11);
    let mut pn7 = n7 % 7;
    if pn7 < 0 { pn7 += 7; }
    let base11_letters = "CFBEADG";
    let preshift11 = |c: char| match c { 'C' => 0, 'F' => 1, 'B' => 2, 'E' => 3, 'A' => 4, 'D' => 5, 'G' => 6, _ => 0 };
    let shift11 = preshift11(letters7.chars().nth(pn7 as usize).unwrap());
    let letters11 = format!("{}{}", &base11_letters[shift11 as usize..], &base11_letters[..shift11 as usize]);

    let mut sf11 = 0;
    let mut pm11 = 0;
    let mut ud = 0;
    qn = n11;
    if n11 > 0 {
        while qn > 0 {
            let mut k = (qn - 1) % 7;
            if k < 0 { k += 7; }
            let char = letters11.chars().nth(k as usize).unwrap();
            if char == 'A' { pm11 -= 1; ud += 1; }
            else if char == 'F' { sf11 -= 1; pm11 -= 1; ud += 1; }
            else { ud += 1; }
            qn -= 1;
        }
    } else if n11 < 0 {
        while qn < 0 {
            let mut k = (qn + 1) % 7;
            if k < 0 { k += 7; }
            let char = letters11.chars().nth(k as usize).unwrap();
            if char == 'D' { pm11 += 1; ud -= 1; }
            else if char == 'B' { sf11 += 1; pm11 += 1; ud -= 1; }
            else { ud -= 1; }
            qn += 1;
        }
    }

    // 13-limit
    let n13 = diff(13);
    let mut pn11 = n11 % 7;
    if pn11 < 0 { pn11 += 7; }
    let base13_letters = "CAFDBGE";
    let preshift13 = |c: char| match c { 'C' => 0, 'A' => 1, 'F' => 2, 'D' => 3, 'B' => 4, 'G' => 5, 'E' => 6, _ => 0 };
    let shift13 = preshift13(letters11.chars().nth(pn11 as usize).unwrap());
    let letters13 = format!("{}{}", &base13_letters[shift13 as usize..], &base13_letters[..shift13 as usize]);

    let mut sf13 = 0;
    let mut pm13 = 0;
    let mut e3 = 0;
    qn = n13;
    if n13 > 0 {
        while qn > 0 {
            let mut k = (qn - 1) % 7;
            if k < 0 { k += 7; }
            let char = letters13.chars().nth(k as usize).unwrap();
            if "CDG".contains(char) { sf13 -= 1; e3 += 1; }
            else if char == 'F' { sf13 -= 1; pm13 -= 1; e3 += 1; }
            else { e3 += 1; }
            qn -= 1;
        }
    } else if n13 < 0 {
        while qn < 0 {
            let mut k = (qn + 1) % 7;
            if k < 0 { k += 7; }
            let char = letters13.chars().nth(k as usize).unwrap();
            if "EAB".contains(char) { sf13 += 1; e3 -= 1; }
            else if char == 'D' { sf13 += 1; pm13 += 1; e3 -= 1; }
            else { e3 -= 1; }
            qn += 1;
        }
    }

    // 17-limit (augmented unison, 0 steps)
    let n17 = diff(17) as i32;
    let mut sf17 = 0;
    let mut a17 = 0;
    if n17 > 0 {
        sf17 += n17;
        a17 += n17;
    } else if n17 < 0 {
        sf17 += n17;
        a17 += n17;
    }
    // 17 doesn't shift the base letter
    let letters17 = letters13.clone();
    let pn17 = (n13 % 7) as i32;

    // 19-limit (+2 steps)
    let n19 = diff(19) as i32;
    let mut pn17_shift = pn17;
    if pn17_shift < 0 { pn17_shift += 7; }
    let base19_letters = "CEGBDFA";
    let preshift19 = |c: char| match c { 'C'=>0, 'E'=>1, 'G'=>2, 'B'=>3, 'D'=>4, 'F'=>5, 'A'=>6, _=>0 };
    let shift19 = preshift19(letters17.chars().nth(pn17_shift as usize).unwrap());
    let letters19 = format!("{}{}", &base19_letters[shift19 as usize..], &base19_letters[..shift19 as usize]);
    
    let mut sf19 = 0;
    let mut pm19 = 0;
    let mut a19 = 0;
    qn = n19 as i64;
    if n19 > 0 {
        while qn > 0 {
            let mut k = (qn - 1) % 7;
            if k < 0 { k += 7; }
            let char = letters19.chars().nth(k as usize).unwrap();
            if "CFG".contains(char) { sf19 -= 1; }
            else if char == 'D' { pm19 += 1; }
            a19 += 1;
            qn -= 1;
        }
    } else if n19 < 0 {
        while qn < 0 {
            let mut k = (qn + 1) % 7;
            if k < 0 { k += 7; }
            let char = letters19.chars().nth(k as usize).unwrap();
            if char == 'F' { pm19 -= 1; }
            else if "AEB".contains(char) { sf19 += 1; }
            a19 -= 1;
            qn += 1;
        }
    }

    // 23-limit (+3 steps)
    let n23 = diff(23) as i32;
    let mut pn19 = n19 % 7;
    if pn19 < 0 { pn19 += 7; }
    let base23_letters = "CFBEADG";
    let preshift23 = |c: char| match c { 'C'=>0, 'F'=>1, 'B'=>2, 'E'=>3, 'A'=>4, 'D'=>5, 'G'=>6, _=>0 };
    let shift23 = preshift23(letters19.chars().nth(pn19 as usize).unwrap());
    let letters23 = format!("{}{}", &base23_letters[shift23 as usize..], &base23_letters[..shift23 as usize]);
    
    let mut sf23 = 0;
    let mut pm23 = 0;
    let mut a23 = 0;
    qn = n23 as i64;
    if n23 > 0 {
        while qn > 0 {
            let mut k = (qn - 1) % 7;
            if k < 0 { k += 7; }
            let char = letters23.chars().nth(k as usize).unwrap();
            if char == 'A' { sf23 += 1; }
            else if "CBEDG".contains(char) { sf23 += 1; pm23 += 1; }
            a23 += 1;
            qn -= 1;
        }
    } else if n23 < 0 {
        while qn < 0 {
            let mut k = (qn + 1) % 7;
            if k < 0 { k += 7; }
            let char = letters23.chars().nth(k as usize).unwrap();
            if char == 'D' { sf23 -= 1; }
            else if "CFEAG".contains(char) { sf23 -= 1; pm23 -= 1; }
            a23 -= 1;
            qn += 1;
        }
    }

    // 29-limit (-1 step)
    let n29 = diff(29) as i32;
    let mut pn23 = n23 % 7;
    if pn23 < 0 { pn23 += 7; }
    let base29_letters = "CBAGFED";
    let preshift29 = |c: char| match c { 'C'=>0, 'B'=>1, 'A'=>2, 'G'=>3, 'F'=>4, 'E'=>5, 'D'=>6, _=>0 };
    let shift29 = preshift29(letters23.chars().nth(pn23 as usize).unwrap());
    let letters29 = format!("{}{}", &base29_letters[shift29 as usize..], &base29_letters[..shift29 as usize]);
    
    let mut sf29 = 0;
    let mut pm29 = 0;
    let mut a29 = 0;
    qn = n29 as i64;
    if n29 > 0 {
        while qn > 0 {
            let mut k = (qn - 1) % 7;
            if k < 0 { k += 7; }
            let char = letters29.chars().nth(k as usize).unwrap();
            if "GBD".contains(char) { pm29 += 1; }
            else if "CF".contains(char) { sf29 -= 1; }
            a29 += 1;
            qn -= 1;
        }
    } else if n29 < 0 {
        while qn < 0 {
            let mut k = (qn + 1) % 7;
            if k < 0 { k += 7; }
            let char = letters29.chars().nth(k as usize).unwrap();
            if "CFA".contains(char) { pm29 -= 1; }
            else if "BE".contains(char) { sf29 += 1; }
            a29 -= 1;
            qn += 1;
        }
    }

    // 31-limit (-1 step)
    let n31 = diff(31) as i32;
    let mut pn29 = n29 % 7;
    if pn29 < 0 { pn29 += 7; }
    let base31_letters = "CBAGFED";
    let preshift31 = |c: char| match c { 'C'=>0, 'B'=>1, 'A'=>2, 'G'=>3, 'F'=>4, 'E'=>5, 'D'=>6, _=>0 };
    let shift31 = preshift31(letters29.chars().nth(pn29 as usize).unwrap());
    let letters31 = format!("{}{}", &base31_letters[shift31 as usize..], &base31_letters[..shift31 as usize]);
    
    let mut sf31 = 0;
    let mut pm31 = 0;
    let mut a31 = 0;
    qn = n31 as i64;
    if n31 > 0 {
        while qn > 0 {
            let mut k = (qn - 1) % 7;
            if k < 0 { k += 7; }
            let char = letters31.chars().nth(k as usize).unwrap();
            if "GBD".contains(char) { sf31 += 1; pm31 += 1; }
            else if "EA".contains(char) { sf31 += 1; }
            a31 += 1;
            qn -= 1;
        }
    } else if n31 < 0 {
        while qn < 0 {
            let mut k = (qn + 1) % 7;
            if k < 0 { k += 7; }
            let char = letters31.chars().nth(k as usize).unwrap();
            if "CFA".contains(char) { pm31 -= 1; sf31 -= 1; }
            else if "GD".contains(char) { sf31 -= 1; }
            a31 -= 1;
            qn += 1;
        }
    }

    let mut pn31 = n31 % 7;
    if pn31 < 0 {
        pn31 += 7;
    }
    let name_base = letters31.chars().nth(pn31 as usize).unwrap().to_string();
    
    let sf = sf3 + sf5 + sf7 + sf11 + sf13 + sf17 + sf19 + sf23 + sf29 + sf31;
    let pm = pm3 + pm5 + pm7 + pm11 + pm13 + pm19 + pm23 + pm29 + pm31;

    Ok(JohnstonComponents {
        letter: name_base,
        sf,
        pm,
        sl: sl,
        ud: ud,
        e3: e3,
        a17,
        a19,
        a23,
        a29,
        a31,
    })
}

pub fn note_name(x: i64, y: i64, quiet: bool) -> String {
    let result = get_johnston_components(x, y);
    match result {
        Err(msg) => {
            if !quiet {
                println!("{}", msg);
            }
            msg
        }
        Ok(comp) => {
            let mut parts = vec![comp.letter.clone()];
            if comp.sf > 0 {
                parts.push("#".repeat(comp.sf as usize));
            } else if comp.sf < 0 {
                parts.push("b".repeat(comp.sf.abs() as usize));
            }
            if comp.sl > 0 {
                parts.push("7".repeat(comp.sl as usize));
            } else if comp.sl < 0 {
                parts.push("L".repeat(comp.sl.abs() as usize));
            }
            if comp.ud > 0 {
                parts.push("^".repeat(comp.ud as usize));
            } else if comp.ud < 0 {
                parts.push("v".repeat(comp.ud.abs() as usize));
            }
            if comp.e3 > 0 {
                parts.push("3".repeat(comp.e3 as usize));
            } else if comp.e3 < 0 {
                parts.push("e".repeat(comp.e3.abs() as usize));
            }
            
            // New 31-limit primes go before +/- as requested to keep things canonical
            if comp.a17 > 0 { parts.push("17".repeat(comp.a17 as usize)); }
            else if comp.a17 < 0 { parts.push("v17".repeat(comp.a17.abs() as usize)); }
            if comp.a19 > 0 { parts.push("19".repeat(comp.a19 as usize)); }
            else if comp.a19 < 0 { parts.push("v19".repeat(comp.a19.abs() as usize)); }
            if comp.a23 > 0 { parts.push("23".repeat(comp.a23 as usize)); }
            else if comp.a23 < 0 { parts.push("v23".repeat(comp.a23.abs() as usize)); }
            if comp.a29 > 0 { parts.push("29".repeat(comp.a29 as usize)); }
            else if comp.a29 < 0 { parts.push("v29".repeat(comp.a29.abs() as usize)); }
            if comp.a31 > 0 { parts.push("31".repeat(comp.a31 as usize)); }
            else if comp.a31 < 0 { parts.push("v31".repeat(comp.a31.abs() as usize)); }

            if comp.pm > 0 {
                parts.push("+".repeat(comp.pm as usize));
            } else if comp.pm < 0 {
                parts.push("-".repeat(comp.pm.abs() as usize));
            }

            let mut name = parts.join("");

            if !quiet {
                println!("{}", name);
            }

            if [comp.sf, comp.pm, comp.sl, comp.ud, comp.e3, comp.a17, comp.a19, comp.a23, comp.a29, comp.a31].iter().any(|&v| v.abs() > 4) {
                let aka = format!("({} {} {} {} {} {} {} {} {} {} {})", comp.letter, comp.sf, comp.sl, comp.ud, comp.e3, comp.a17, comp.a19, comp.a23, comp.a29, comp.a31, comp.pm);
                if !quiet {
                    println!("{}", aka);
                }
                name = format!("{}\n{}", name, aka);
            }

            name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_name() {
        assert_eq!(note_name(49, 55, true), "Cb77v+");
        assert_eq!(note_name(37, 16, true), "Sorry - that is a 37-limit pitch!");
        assert_eq!(note_name(3, 2, true), "G");
        assert_eq!(note_name(5, 4, true), "E");
        assert_eq!(note_name(7, 4, true), "Bb7");
        assert_eq!(note_name(11, 8, true), "F^");
        assert_eq!(note_name(13, 8, true), "Ab3");

        // 31-limit new tests
        assert_eq!(note_name(17, 16, true), "C#17");
        assert_eq!(note_name(19, 16, true), "Eb19");
        assert_eq!(note_name(23, 16, true), "F#23+");
        assert_eq!(note_name(29, 16, true), "Bb29");
        assert_eq!(note_name(31, 16, true), "B31");
    }
}
