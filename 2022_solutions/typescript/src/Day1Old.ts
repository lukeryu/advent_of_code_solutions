const puzzle1 = (data: string[]): number => {
    let count = 0;
    let lastValue = undefined;
    for (const currentValue of data) {
        if (lastValue && +currentValue > +lastValue) {
            count++;
        }
        lastValue = currentValue;
    }
    return count;
}

const sumWindow = (data: string[], index: number): number => {
    let sum = 0;

    sum += +data[index];
    sum += +data[index - 1];
    sum += +data[index - 2];

    return sum;
}

const puzzle2 = (data: string[]): number => {
    let count = 0;
    let previousSum = undefined;
    for (let i = 2; i < data.length; i++) {
        const currentValue = sumWindow(data, i);
        if (previousSum && currentValue > previousSum) {
            count++;
        }
        previousSum = currentValue;
    }
    return count;
}

export {puzzle1, puzzle2};