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

	string res;

	while (!steps.empty())
	{
		Step step = steps[0];

		bool canBeDone = true;

		for (int i = 0; canBeDone && i < step.parents.size(); ++i)
			if (res.find(step.parents[i].letter) == string::npos)
				canBeDone = false;

		if (canBeDone)
			res += step.letter;

		steps.erase(steps.begin());

		if (canBeDone)
			sort(steps.begin(), steps.end(), fncomp);

		else
			steps.push_back(step);
	}

	cout << res;

	return 0;
}