document.addEventListener('DOMContentLoaded', () => {
    console.log('Poker Game Frontend Loaded');

    const canvas = document.getElementById('game-canvas');
    const ctx = canvas.getContext('2d');

    // Establish WebSocket connection to the backend
    const socket = new WebSocket('ws://127.0.0.1:8080/ws');

    socket.addEventListener('open', () => {
        console.log('Connected to the backend');
        // Send initial message to join the game
        socket.send(JSON.stringify("Join"));
    });

    socket.addEventListener('message', (event) => {
        console.log('Message from server:', event.data);
        const data = JSON.parse(event.data);
        // Handle game state updates here
        drawGameBoard(data);
    });

    socket.addEventListener('close', () => {
        console.log('Disconnected from the backend');
    });

    socket.addEventListener('error', (error) => {
        console.error('WebSocket error:', error);
    });

    function drawGameBoard(gameState) {
        // Clear the canvas
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // Draw the game board
        ctx.fillStyle = '#fff';
        ctx.fillRect(10, 10, canvas.width - 20, canvas.height - 20);

        // Draw community cards
        ctx.fillStyle = '#000';
        ctx.font = '20px Arial';
        ctx.fillText('Community Cards:', 20, 50);

        // Draw player hands
        ctx.fillText('Player Hands:', 20, 100);

        // Draw pot and current bet
        ctx.fillText(`Pot: ${gameState.pot}`, 20, canvas.height - 30);
        ctx.fillText(`Current Bet: ${gameState.current_bet}`, 20, canvas.height - 10);
    }

    // Add event listeners for game actions
    document.addEventListener('keydown', (event) => {
        switch (event.key) {
            case 'b':
                socket.send(JSON.stringify({ Bet: 10 }));
                break;
            case 'f':
                socket.send(JSON.stringify("Fold"));
                break;
            case 'c':
                socket.send(JSON.stringify("Check"));
                break;
            case 'a':
                socket.send(JSON.stringify("Call"));
                break;
        }
    });
});