import { noteName } from './calculator';

function main() {
    const args = process.argv.slice(2);
    if (args.length === 0) {
        console.log("Please provide a ratio (e.g., 49/55) or two integers.");
        process.exit(1);
    }

    let x: number, y: number;

    if (args.length >= 2) {
        x = parseInt(args[0]);
        y = parseInt(args[1]);
    } else {
        const arg = args[0];
        if (arg.includes('/')) {
            const parts = arg.split('/');
            if (parts.length !== 2) {
                console.log("Invalid ratio format. Use x/y.");
                process.exit(1);
            }
            x = parseInt(parts[0]);
            y = parseInt(parts[1]);
        } else if (arg.includes(' ')) {
            const parts = arg.split(/\s+/);
            if (parts.length !== 2) {
                console.log("Invalid ratio format. Use 'x y'.");
                process.exit(1);
            }
            x = parseInt(parts[0]);
            y = parseInt(parts[1]);
        } else {
            console.log("Please provide a ratio (e.g., 49/55) or two integers.");
            process.exit(1);
        }
    }

    if (isNaN(x) || isNaN(y)) {
        console.log("Invalid input. Please provide integers.");
        process.exit(1);
    }

    noteName(x, y);
}

main();
