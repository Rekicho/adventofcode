#include <fstream>
#include <iostream>
#include <list>

using namespace std;

void advance(list<int>::iterator &it, list<int> &circle, int times)
{
	for (int i = 0; i < times; ++i)
	{
		++it;
		if (it == circle.end())
			it = circle.begin();
	}
}

void back(list<int>::iterator &it, list<int> &circle, int times)
{
	for (int i = 0; i < 7; ++i)
	{
		if (it == circle.begin())
			it = circle.end();

		--it;
	}
}

int main()
{
	string waste;
	ifstream input("input.txt");

	getline(input, waste, ' ');
	int playerNum = stoi(waste);

	getline(input, waste, 'h');
	input.ignore();
	getline(input, waste, ' ');
	int lastMarble = stoi(waste);

	input.close();

	int scores[playerNum];

	for (int i = 0; i < playerNum; ++i)
		scores[i] = 0;

	list<int> circle;
	circle.push_front(0);
	list<int>::iterator it = circle.begin();

	for (int i = 1; i <= lastMarble; ++i)
	{
		if (i % 23 == 0)
		{
			scores[i % playerNum] += i;

			back(it, circle, 7);

			scores[i % playerNum] += (*it);

			circle.erase(it);
			advance(it, circle, 1);
		}

		else
		{
			advance(it, circle, 2);
			circle.insert(it, i);
			--it;
		}
	}

	int bestScore = 0;

	for (int i = 0; i < playerNum; ++i)
		bestScore = max(bestScore, scores[i]);

	cout << bestScore;

	return 0;
}