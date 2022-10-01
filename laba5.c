#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>

int main() {
	int arr[] = { 23, 5678, 2, 564, 365, 77, 443 };
	for (int i = 0; i < 7; ++i)
		printf("%d \t", arr[i]);

	int arrfirst[2][2] = { {1, 0}, {1, 4} };
	int arrsecond[2][2] = { {1, 2}, {0, 1} };
	int arrrezult[4];
	int counter = 0;
	for (int i = 0; i < 2; ++i) {
		for (int k = 0; k < 2; ++k) {
			int tmp = 0;
			for (int j = 0; j < 2; j++) {
				tmp += arrfirst[i][j] * arrsecond[j][k];
			}
			arrrezult[counter] = tmp;
			counter += 1;
		}
	}
	printf("\n%d\t%d\n%d\t%d", arrrezult[0], arrrezult[1], arrrezult[2], arrrezult[3]);
}
