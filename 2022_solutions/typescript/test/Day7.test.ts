import {puzzle1, puzzle2} from "../src/Day7";
import {readFile} from "../src/Utils";

describe('Day7', () => {

    const TEST_DATA: string[] = [
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k"];

    describe('puzzle1', () => {
        it('Expectation scenario', () => {

            const result = puzzle1(TEST_DATA);

            expect(result).toBe(95437);
        });

        it('real data scenario', () => {
            const data = readFile('Day7.txt');
            const result = puzzle1(data);

            expect(result).toBe(1243729);
        });

    });

    describe('puzzle2', () => {
        it('Expectation scenario', () => {
            const result = puzzle2(TEST_DATA);

            expect(result).toBe(24933642);
        });

        it('real data scenario', () => {
            const data = readFile('Day7.txt');
            const result = puzzle2(data);

            expect(result).toBe(4443914);
        });
    });
});