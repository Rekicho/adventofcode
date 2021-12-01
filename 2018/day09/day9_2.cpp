#include <fstream>
#include <iostream>
#include <list>

using namespace std;

void advance(list<long long>::iterator &it, list<long long> &circle, long long times)
{
	for (long long i = 0; i < times; ++i)
	{
		++it;
		if (it == circle.end())
			it = circle.begin();
	}
}

void back(list<long long>::iterator &it, list<long long> &circle, long long times)
{
	for (long long i = 0; i < 7; ++i)
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
	long long playerNum = stoi(waste);

	getline(input, waste, 'h');
	input.ignore();
	getline(input, waste, ' ');
	long long lastMarble = stoi(waste) * 100;

	input.close();

	long long scores[playerNum];

	for (long long i = 0; i < playerNum; ++i)
		scores[i] = 0;

	list<long long> circle;
	circle.push_front(0);
	list<long long>::iterator it = circle.begin();

	for (long long i = 1; i <= lastMarble; ++i)
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

	long long bestScore = 0;

	for (long long i = 0; i < playerNum; ++i)
		bestScore = max(bestScore, scores[i]);

	cout << bestScore;

	return 0;
}