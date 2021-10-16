#include <fstream>
#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

struct Step
{
	char letter;
	vector<Step> parents;
};

bool fncomp(Step lhs, Step rhs)
{
	return lhs.letter < rhs.letter;
}

void removeDuplicates(vector<Step> &steps)
{
	for (int i = 0; i < steps.size(); ++i)
		for (int j = i + 1; j < steps.size(); ++j)
			if (steps[i].letter == steps[j].letter)
			{
				for (int k = 0; k < steps[j].parents.size(); ++k)
					steps[i].parents.push_back(steps[j].parents[k]);

				steps.erase(steps.begin() + j);
				j--;
			}
}

struct Worker
{
	char letter;
	int secondFinish;
};

int main()
{
	vector<Step> steps;
	char parent, child;
	string waste;

	ifstream input("input.txt");

	while (!input.eof())
	{
		getline(input, waste, ' ');

		getline(input, waste, ' ');
		parent = waste[0];

		getline(input, waste, ' ');
		getline(input, waste, ' ');
		getline(input, waste, ' ');
		getline(input, waste, ' ');
		getline(input, waste, ' ');

		getline(input, waste, ' ');
		child = waste[0];

		Step parentStep;
		parentStep.letter = parent;

		Step childStep;
		childStep.letter = child;

		childStep.parents.push_back(parentStep);

		steps.push_back(parentStep);
		steps.push_back(childStep);

		getline(input, waste);
	}

	input.close();

	removeDuplicates(steps);
	sort(steps.begin(), steps.end(), fncomp);

	vector<Worker> workers;
	Worker temp;
	temp.letter = -1;
	temp.secondFinish = -2;

	for (int i = 0; i < 5; ++i)
		workers.push_back(temp);

	string res;
	int second = -1;
	bool finisihed = false;

	while (!finisihed)
	{
		int availableWorkers = 0;

		for (int i = 0; i < workers.size(); ++i)
			if (second > workers[i].secondFinish)
			{
				if (workers[i].letter != -1)
				{
					res += workers[i].letter;
					workers[i].letter = -1;
				}

				++availableWorkers;
			}

		vector<Step> available;

		for (int i = 0; i < steps.size() && availableWorkers > 0; ++i)
		{
			bool canBeDone = true;

			for (int j = 0; canBeDone && j < steps[i].parents.size(); ++j)
				if (res.find(steps[i].parents[j].letter) == string::npos)
					canBeDone = false;

			if (canBeDone)
			{
				available.push_back(steps[i]);
				steps.erase(steps.begin() + i);
				--availableWorkers;
			}
		}

		for (int i = 0; i < workers.size() && !available.empty(); ++i)
			if (second > workers[i].secondFinish)
			{
				workers[i].letter = available[0].letter;
				workers[i].secondFinish = second + 60 + (available[0].letter - 'A');
				available.erase(available.begin());
			}

		++second;

		if (availableWorkers == workers.size())
			finisihed = true;
	}

	cout << second;

	return 0;
}