with open("input.txt") as f:
    content = f.read()

decks = content.strip().split('\n\n')
decks = [deck.split(':\n')[1].split('\n') for deck in decks]

player1 = [int(card) for card in decks[0]]
player2 = [int(card) for card in decks[1]]


def play_game(deck1, deck2):
    previous = set()

    while len(deck1) != 0 and len(deck2) != 0:
        if str([deck1,deck2]) in previous:
            return (1, deck1)

        previous.add(str([deck1,deck2]))

        card1 = deck1.pop(0)
        card2 = deck2.pop(0)

        if len(deck1) >= card1 and len(deck2) >= card2:
            winner = play_game(deck1[:card1],deck2[:card2])[0]

            if winner == 1:
                deck1.extend([card1, card2])

            else:
                deck2.extend([card2, card1])

        else:
            if card1 > card2:
                deck1.extend([card1, card2])

            else:
                deck2.extend([card2, card1])

    return (1, deck1) if len(deck1) != 0 else (2, deck2)

winner = play_game(player1[:], player2[:])[1]
winner.reverse()

score = 0

for i, card in enumerate(winner):
    score += card * (i + 1)

print(score)