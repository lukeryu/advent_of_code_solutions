import {reduceSum} from "./Utils";

function parseElfPacks(data: string[]): Map<number, number[]> {
    let elf_count = 1;
    let current_array: number[] = [];
    const elf_packs = new Map<number, number[]>();

    for (let index = 0; index < data.length; index++) {
        const dataRow = data[index];
        if (dataRow.length == 0) {
            elf_packs.set(elf_count, current_array);
            current_array = [];
            elf_count = elf_count + 1;
        } else {
            current_array.push(+dataRow);
        }
    }

    if (current_array.length > 0) {
        elf_packs.set(elf_count, current_array);
    }

    return elf_packs;
}

const puzzle1 = (data: string[]): number => {
    const elf_packs: Map<number, number[]> = parseElfPacks(data);
    let most_calories_elf = 0;
    let most_calories_amount = 0;

    elf_packs.forEach((value, key) => {
        const total_amount = value.reduce(reduceSum);
        if (total_amount > most_calories_amount) {
            most_calories_amount = total_amount;
            most_calories_elf = key;
        }
    });

    return most_calories_amount;
}

const puzzle2 = (data: string[]): number => {
    const elf_packs: Map<number, number[]> = parseElfPacks(data);
    const elfSums: number[] = [];

    elf_packs.forEach((value, key) => {
        const total_amount = value.reduce(reduceSum);
        elfSums.push(total_amount);
    });

    elfSums.sort((a, b) => a < b ? 1 : -1);

    return elfSums[0] + elfSums[1] + elfSums[2];
}

export {puzzle1, puzzle2};

