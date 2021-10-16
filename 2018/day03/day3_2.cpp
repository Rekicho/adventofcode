#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

void expandMap(vector<vector<int>> &map, const unsigned int x, const unsigned int y)
{
	if (map.empty())
	{
		for (int i = 0; i < y; ++i)
		{
			vector<int> temp;

			for (int j = 0; j < x; ++j)
				temp.push_back(0);

			map.push_back(temp);
		}
	}

	else
	{
		for (int i = 0; i < map.size(); ++i)
			for (int j = map[i].size(); j < x; ++j)
				map[i].push_back(0);

		for (int i = map.size(); i < y; ++i)
		{
			vector<int> temp;

			for (int j = 0; j < x; ++j)
				temp.push_back(0);

			map.push_back(temp);
		}
	}
}

void buildMap(vector<vector<int>> &map, const unsigned int id, const unsigned int x, const unsigned int y, const unsigned int width, const unsigned int height)
{
	expandMap(map, x + width, y + height);

	for (int i = y; i < y + height; ++i)
		for (int j = x; j < x + width; ++j)
			switch (map[i][j])
			{
			case 0:
				map[i][j] = id;
				break;
			case -1:
				break;
			default:
				map[i][j] = -1;
				break;
			}
}

int main()
{
	vector<vector<int>> map;
	unsigned int id, x, y, width, height;
	string line;

	ifstream input("input.txt");

	while (!input.eof())
	{
		input.ignore();
		getline(input, line, ' ');
		id = stoi(line);

		input.ignore();
		input.ignore();
		getline(input, line, ',');
		x = stoi(line);

		getline(input, line, ':');
		y = stoi(line);

		input.ignore();
		getline(input, line, 'x');
		width = stoi(line);

		getline(input, line);
		height = stoi(line);

		buildMap(map, id, x, y, width, height);
	}

	input.close();

	input.open("input.txt");

	int counter;

	while (!input.eof())
	{
		input.ignore();
		getline(input, line, ' ');
		id = stoi(line);

		input.ignore();
		input.ignore();
		getline(input, line, ',');
		x = stoi(line);

		getline(input, line, ':');
		y = stoi(line);

		input.ignore();
		getline(input, line, 'x');
		width = stoi(line);

		getline(input, line);
		height = stoi(line);

		counter = 0;

		for (int i = 0; i < map.size(); ++i)
			for (int j = 0; j < map[i].size(); ++j)
				counter += map[i][j] == id ? 1 : 0;

		if (counter == width * height)
			break;
	}

	input.close();

	cout << id;

	return 0;
}