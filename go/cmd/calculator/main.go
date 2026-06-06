package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"johnston-notation/pkg/calculator"
)

func main() {
	args := os.Args[1:]
	if len(args) == 0 {
		fmt.Println("Please provide a ratio (e.g., 49/55) or two integers.")
		os.Exit(1)
	}

	var x, y int
	var err error

	if len(args) >= 2 {
		x, err = strconv.Atoi(args[0])
		if err != nil {
			fmt.Printf("Invalid numerator: %s\n", args[0])
			os.Exit(1)
		}
		y, err = strconv.Atoi(args[1])
		if err != nil {
			fmt.Printf("Invalid denominator: %s\n", args[1])
			os.Exit(1)
		}
	} else {
		arg := args[0]
		if strings.Contains(arg, "/") {
			parts := strings.Split(arg, "/")
			if len(parts) != 2 {
				fmt.Println("Invalid ratio format. Use x/y.")
				os.Exit(1)
			}
			x, err = strconv.Atoi(parts[0])
			if err != nil {
				fmt.Printf("Invalid numerator: %s\n", parts[0])
				os.Exit(1)
			}
			y, err = strconv.Atoi(parts[1])
			if err != nil {
				fmt.Printf("Invalid denominator: %s\n", parts[1])
				os.Exit(1)
			}
		} else {
			// Try to see if there's a space if it was passed as one quoted arg, 
			// though usually shell splits it.
			parts := strings.Fields(arg)
			if len(parts) == 2 {
				x, err = strconv.Atoi(parts[0])
				if err != nil {
					os.Exit(1)
				}
				y, err = strconv.Atoi(parts[1])
				if err != nil {
					os.Exit(1)
				}
			} else {
				fmt.Println("Please provide a ratio (e.g., 49/55) or two integers.")
				os.Exit(1)
			}
		}
	}

	calculator.NoteName(x, y, false)
}
