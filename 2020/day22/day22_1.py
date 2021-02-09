with open("input.txt") as f:
    content = f.read()

decks = content.strip().split('\n\n')
decks = [deck.split(':\n')[1].split('\n') for deck in decks]

deck1 = [int(card) for card in decks[0]]
deck2 = [int(card) for card in decks[1]]

while len(deck1) != 0 and len(deck2) != 0:
    card1 = deck1.pop(0)
    card2 = deck2.pop(0)

    if card1 > card2:
        deck1.extend([card1, card2])

    else:
        deck2.extend([card2, card1])

winner = deck1 if len(deck1) != 0 else deck2
winner.reverse()

score = 0

for i, card in enumerate(winner):
    score += card * (i + 1)

print(score)