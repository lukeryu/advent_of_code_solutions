import {readFileSync} from 'fs';

const readFile = (filename: string): string[] => {
    const buffer = readFileSync('data/' + filename, 'utf-8');

    return buffer.split(/\r?\n/);
};

const toNumber = (input: string): number => {
    return +input;
};

const reduceSum = (previousValue: number, currentValue: number): number => {
    return previousValue + currentValue;
};

function isBetween<T>(lowerBound: T, upperBound: T, value: T): boolean {
    return value >= lowerBound && value <= upperBound;
}

class Range<T> {
    lhs: T;
    rhs: T;

    constructor(lhs: T, rhs: T) {
        if (lhs < rhs) {
            this.lhs = lhs;
            this.rhs = rhs;
        } else {
            this.lhs = rhs;
            this.rhs = lhs;
        }

    }

    encompasses(otherRange: Range<T>): boolean {
        return this.lhs <= otherRange.lhs && this.rhs >= otherRange.rhs;
    }

    overlaps(otherRange: Range<T>): boolean {
        return isBetween(this.lhs, this.rhs, otherRange.lhs)
            || isBetween(this.lhs, this.rhs, otherRange.rhs)
            || otherRange.encompasses(this);
    }

}

class Stack<T> {
    private storage: T[] = [];

    constructor(private capacity: number = Infinity) {}

    push(item: T): void {
        if (this.size() === this.capacity) {
            throw Error("Stack has reached max capacity, you cannot add more items");
        }
        this.storage.push(item);
    }

    pop(): T | undefined {
        return this.storage.pop();
    }

    peek(): T | undefined {
        return this.storage[this.size() - 1];
    }

    size(): number {
        return this.storage.length;
    }
}

export {readFile, toNumber, reduceSum, Range, Stack};