import {puzzle1, puzzle2} from "../src/Day3";
import {readFile} from "../src/Utils";

describe('Day3', () => {
    const testData = ['vJrwpWtwJgWrhcsFMMfFFhFp',
        'jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL',
        'PmmdzqPrVvPwwTWBwg',
        'wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn',
        'ttgJtRGJQctTZtZT',
        'CrZsJsPPZsGzwwsLwLmpwMDw'];

    describe('puzzle1', () => {
        it('Expectation scenario', () => {
            const result = puzzle1(testData);

            expect(result).toBe(157);
        });

        it('real data scenario', () => {
            const data = readFile('Day3.txt');
            const result = puzzle1(data);

            expect(result).toBe(8185);
        });

    });

    describe('puzzle2', () => {
        it('Expectation scenario', () => {

            const result = puzzle2(testData);

            expect(result).toBe(70);
        });

        it('real data scenario', () => {
            const data = readFile('Day3.txt');
            const result = puzzle2(data);

            expect(result).toBe(2817);
        });
    });
});