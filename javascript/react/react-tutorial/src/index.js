import React from 'react'
import ReactDom from 'react-dom'
import './index.css'

class Square extends React.Component {
    render() {
        const style = this.props.isHightLight ? { backgroundColor: "yellow" } : {};

        return (
            <button className="square" onClick={ this.props.onClick } style={style}>
                { this.props.value }
            </button>
        );
    }
}

class Board extends React.Component {
    renderSquare(i, isWinSquare) {
        return <Square
                value = { this.props.squares[i] }
                key = { i }
                isHightLight = { isWinSquare }
                onClick = { () => this.props.onClick(i) } />;
    }

    renderBoard() {
        const rows =
            [0, 1, 2].map((row) => {
                const squares = [];

                [0, 1, 2].forEach((col) => {
                    const number = row * 3 + col;

                    let isWinSquare = false;
                    if (this.props.winCause) {
                        isWinSquare = this.props.winCause.includes(number);
                    }

                    squares.push(this.renderSquare(number, isWinSquare));
                });

                return (
                    <div key={row} className="board-row">
                        {squares}
                    </div>
                );
            });

        return rows;
    }

    render() {
        return (
            <div>
                {this.renderBoard()}
            </div>
        );
    }
}

class MoveOrderModeChangeButton extends React.Component {
    render() {
        return (
            <button onClick={ this.props.onClick }>
                { this.props.isAsc ? 'ASC' : 'DESC' }
            </button>
        );
    }
}

class Game extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            isHistoryOrderByAsc: true,
            history: [{
                squares: Array(9).fill(null),
                location: {
                    col: 0,
                    row: 0,
                },
            }],
            stepNumber: 0,
            xIsNext: true,
        };
    }

    handleClick(i) {
        const history = this.state.history.slice(0, this.state.stepNumber + 1);
        const current = history[history.length - 1];
        const squares = current.squares.slice();
        if (calculateWinner(squares) || squares[i]) {
            return;
        }

        squares[i] = this.state.xIsNext ? 'X' : 'O';
        const col = i % 3;
        const row = Math.floor(i / 3);

        this.setState({
            history: history.concat([{
                squares: squares,
                location: {
                    col: col,
                    row: row,
                },
            }]),
            stepNumber: history.length,
            xIsNext: !this.state.xIsNext,
        });
    }

    jumpTo(step) {
        this.setState({
            stepNumber: step,
            xIsNext: (step % 2) === 0,
        });
    }

    toggleOrderMode() {
        this.setState({
            isHistoryOrderByAsc: !this.state.isHistoryOrderByAsc,
        });
    }

    render() {
        const history = this.state.history;
        const current = history[this.state.stepNumber];
        
        let move = history.map((step, move) => {
            const desc = move ?
            'Go to move #' + move +
            ' col: ' + step.location.col +
            ' row: ' + step.location.row :
            'Go to game start';
            
            const style = this.state.stepNumber === move ? { fontWeight: "bold" } : {};
            
            return (
                <li key={move}>
                    <button style={style} onClick = { () => this.jumpTo(move) }>{desc}</button>
                </li>
            );
        });
        
        const isOrderByAsc = this.state.isHistoryOrderByAsc;
        move = move.sort((a, b) => {
            if (a["key"] < b["key"]) return isOrderByAsc ? -1 :  1;
            if (a["key"] > b["key"]) return isOrderByAsc ?  1 : -1;
            return 0;
        });
        
        const winner = calculateWinner(current.squares);
        const allPanelIsChecked = calculateAllPanelIsClicked(current.squares);
        let status;
        if (winner) {
            status = 'Winner: ' + winner;
        } else if (allPanelIsChecked) {
            status = 'No winner. Game Over';
        } else {
            status = 'Next player: ' + (this.state.xIsNext ? 'X' : 'O');
        }

        const winCause = calculateWinCause(current.squares);

        return(
            <div className="game">
                <div className="game-board">
                    <Board
                        winCause= { winCause }
                        squares = { current.squares }
                        onClick = { (i) => this.handleClick(i) }/>
                </div>
                <div className="game-info">
                    <div>{ status }</div>
                    <MoveOrderModeChangeButton
                        isAsc = { this.state.isHistoryOrderByAsc }
                        onClick = { () => this.toggleOrderMode() } />
                    <div>{ move }</div>
                </div>
            </div>
        );
    }
}

function calculateWinner(squares) {
    const winCause = calculateWinCause(squares);

    if (winCause) {
        return squares[winCause[0]];
    }

    return null;
}

function calculateWinCause(squares) {
    const lines = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for (let i = 0; i < lines.length; i++) {
        const [a, b, c] = lines[i];
        if (squares[a] && squares[a] === squares[b] && squares[a] === squares[c]) {
            return [a, b, c];
        }
    }

    return null;
}

function calculateAllPanelIsClicked(squares) {
    return squares.filter((v) => v === null).length === 0;
}



ReactDom.render(
    <Game />,
    document.getElementById('root')
);