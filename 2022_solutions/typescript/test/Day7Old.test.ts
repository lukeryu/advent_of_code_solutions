import {puzzle1, puzzle2} from "../src/Day7Old";
import {readFile} from "../src/Utils";

describe('Day7Old', () => {
    describe('puzzle1', () => {
        it('Expectation scenario', () => {
            const result = puzzle1('16,1,2,0,4,2,7,1,2,14');

            expect(result).toBe(37);
        });

        it('real data scenario', () => {
            const data = readFile('Day7Old.txt');
            const result = puzzle1(data[0]);

            expect(result).toBe(340056);
        });

    });

    describe('puzzle2', () => {
        it('Expectation scenario', () => {
            const result = puzzle2('16,1,2,0,4,2,7,1,2,14');

            expect(result).toBe(168);
        });

        it('real data scenario', () => {
            const data = readFile('Day7Old.txt');
            const result = puzzle2(data[0]);

            expect(result).toBe(96592275);
        });
    });
});