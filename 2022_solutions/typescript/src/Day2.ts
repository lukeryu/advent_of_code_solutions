enum RPS {
    ROCK,
    PAPER,
    SCISSORS
}

const getRPSScore = (rps: RPS): number => {
    if (rps === RPS.ROCK) {
        return 1;
    }
    if (rps === RPS.PAPER) {
        return 2;
    }
    if (rps === RPS.SCISSORS) {
        return 3
    }

    return 0;
}

const getRPS = (string: string): RPS => {
    if (string === "A" || string === "X") {
        return RPS.ROCK;
    }
    if (string === "B" || string === "Y") {
        return RPS.PAPER;
    }
    if (string === "C" || string === "Z") {
        return RPS.SCISSORS;
    }
    return undefined;
}

const wins = (myRPS: RPS, opponentRPS: RPS): boolean => {
    if (myRPS === RPS.ROCK && opponentRPS === RPS.SCISSORS) {
        return true;
    }
    if (myRPS === RPS.SCISSORS && opponentRPS === RPS.PAPER) {
        return true;
    }
    if (myRPS === RPS.PAPER && opponentRPS === RPS.ROCK) {
        return true;
    }
    return false;
}

const getMyRPS = (opponentRPS: RPS, codeString: string): RPS => {
    if (codeString === "X") {
//LOSE
        if (opponentRPS === RPS.ROCK) {
            return RPS.SCISSORS;
        }
        if (opponentRPS === RPS.PAPER) {
            return RPS.ROCK;
        }
        if (opponentRPS === RPS.SCISSORS) {
            return RPS.PAPER;
        }
    }
    if (codeString === "Y") {
//DRAW
        return opponentRPS;
    }
    if (codeString === "Z") {
//WIN
        if (opponentRPS === RPS.ROCK) {
            return RPS.PAPER;
        }
        if (opponentRPS === RPS.PAPER) {
            return RPS.SCISSORS;
        }
        if (opponentRPS === RPS.SCISSORS) {
            return RPS.ROCK;
        }
    }
}

const puzzle1 = (data: string[]): number => {
    let sum = 0;
    for (const datum of data) {
        const strings = datum.split(' ');
        const opponentRPS = getRPS(strings[0]);
        const myRPS = getRPS(strings[1]);
        sum += getRPSScore(myRPS);
        if (wins(myRPS, opponentRPS)) {
            sum += 6;
        } else if (myRPS === opponentRPS) {
            sum += 3;
        }
    }
    return sum;
}

const puzzle2 = (data: string[]): number => {
    let sum = 0;
    for (const datum of data) {
        const strings = datum.split(' ');
        const opponentRPS = getRPS(strings[0]);
        const myRPS = getMyRPS(opponentRPS, strings[1]);
        sum += getRPSScore(myRPS);
        if (wins(myRPS, opponentRPS)) {
            sum += 6;
        } else if (myRPS === opponentRPS) {
            sum += 3;
        }
    }
    return sum;
}

export {puzzle1, puzzle2};

