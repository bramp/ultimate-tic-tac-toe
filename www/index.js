import { Game, Square, RandomAI, MonteCarloAI } from "ultimate-tic-tac-toe";

//const ai = new RandomAI();
const ai_assist = new MonteCarloAI();

const game = new Game();
let best;

const calculate_probabilies = () => {
	// Do this in the background.
	setTimeout(() => {
		const turns = game.turns();
		best = ai_assist.choose(game);

		// Only update if we are still on this turn.
		if (game.turns() == turn) {
			drawBoard(game);
		}
	}, 0);
}

const play = (board, square) => {
	try {
		game.play(board, square);
	} catch(err) {
		console.log(err);
		return;
	}

	drawBoard(game);
	calculate_probabilies();

/*
	if (game.current_turn() == Square.X) {
		const p = ai.choose(game);
		//play(p.board_pos, p.square_pos);

		console.log("Runs", ai.runs());
		for (let i = 0; i < 9; i++) {
			for (let j = 0; j < 9; j++) {
				let s = ai.stats(i, j);
				console.log("Square", i, j, s);
			}
		}
	}
*/
}

const squareName = (square) => {
	switch (square) {
		case Square.O: return "O";
		case Square.X: return "X";
	}
	return "";
}

const drawBoard = (game) => {
	const current_player = game.current_player();
	const current_board = game.current_board();

	const turn = document.querySelector("#turn");
	turn.innerHTML = squareName(current_player) + "'s turn";

	const megaBoard = document.querySelector("#mega-board");
	switch (current_player) {
		case Square.O: {
			megaBoard.setAttribute("class", "blue");
			break;
		}
		case Square.X: {
			megaBoard.setAttribute("class", "red");
			break;
		}
	}

	let boards = [];
	for (let i = 0; i < 9; i++) {
		const b = game.board(i);
		let board;

		board = document.createElement("div");
		board.classList.add("board");

		const current = (current_board === i || (current_board === undefined && b.playable()));
		if (current) {
			board.classList.add("current");
		}

		for (let j = 0; j < 9; j++) {
			const s = b.square(j);

			const square = document.createElement("div");
			square.classList.add("square");

			if (current && s == Square.None) {
				square.classList.add("available");

				if (ai_assist.runs() > 0) {
					let stats = ai_assist.stats(i, j);
					let runs = ai_assist.runs();

					let win_p = stats.wins / stats.totals;
					let lose_p = stats.loses / stats.totals;
					let draw_p = (stats.totals - stats.wins - stats.loses) / stats.totals;

					if (i == best.board_pos && j == best.square_pos) {
						square.classList.add("best");
					}
 
					square.innerHTML = 
						"w: " + (win_p * 100).toFixed(2) + "%<br/>" +
						"l: " + (lose_p * 100).toFixed(2) + "%<br/>" +
						"d: " + (draw_p * 100).toFixed(2) + "%";
				}
			} else if (s != Square.None) {
				square.classList.add("won");

				square.innerHTML = squareName(s);
			}

			
			square.addEventListener("click", () => {
				play(i, j);
			});

			board.appendChild(square);
		}
	
		// Add a board winner overlay
		let winner = b.winner();
		if (winner !== Square.None) {
			let w = document.createElement("div");
			w.classList.add("winner");
			if (winner == Square.O) {
				w.classList.add("winner-blue");
			} else if (winner == Square.X) {
				w.classList.add("winner-red");
			}

			w.innerHTML = squareName(winner);
			board.appendChild(w);
		}

		boards.push(board);
	}

	// Add a megagrid winner overlay
	if (!game.playable()) {
		const winner = game.winner();

		let w = document.createElement("div");
		w.classList.add("mega-winner");

		if (winner == Square.O) {
			w.classList.add("winner-blue");
			w.innerHTML = "O";
		} else if (winner == Square.X) {
			w.classList.add("winner-red");
			w.innerHTML = "X";
		} else {
			w.classList.add("winner-draw");
			w.innerHTML = "Draw";
		}

		boards.push(w);
	}

	megaBoard.replaceChildren(...boards);


	if (ai_assist) {
		const stats = ai_assist.totals();
		if (stats.totals > 0) {
			let o; let x;
			let totals = stats.totals;
			if (current_player == Square.O) {
				o = stats.wins;
				x = stats.loses;
			} else {
				x = stats.wins;
				o = stats.loses;		
			}

			const div = document.querySelector("#ai-stats");
			div.innerHTML =  
				" O Win:" + ((o / totals) * 100).toFixed(1) + "%" +
				" X Win:" + ((x / totals) * 100).toFixed(1) + "%" +
				"  Draw:" + ((1 - (x+o) / totals) * 100).toFixed(1) + "%";
		}
	}

}


drawBoard(game);
calculate_probabilies();

// Play a few rounds (must be without AI)
/*
play(0, 0); // O
play(0, 5);

play(5, 0); // O
play(0, 2);
play(2, 0); // O
play(0, 3);
play(3, 0); // O
play(0, 4); // X wins 0 now jumps to 4
play(4, 8); // O
play(8, 7);
play(7, 8); // O
play(8, 2);
play(2, 8); // O
play(8, 4);
play(4, 4); // O
play(4, 0); // X jumps to 0
*/
