/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: kjullien <kjullien@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/02 22:38:52 by kjullien          #+#    #+#             */
/*   Updated: 2024/12/02 23:32:54 by kjullien         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "string.h"
#include <stdio.h>
#include <stdlib.h>

int	get_number_amount(int numbers[20])
{
	int	counter;

	counter = 0;
	while (numbers[counter])
		counter++;
	return (counter);
}

int	check_ascending(int numbers[20])
{
	int	counter;
	int	numbers_amount;

	counter = 0;
	numbers_amount = get_number_amount(numbers);
	while (counter < numbers_amount - 1)
	{
		if (!(numbers[counter] < numbers[counter + 1] && numbers[counter + 1]
				- numbers[counter] <= 3))
			return (0);
		counter++;
	}
	return (1);
}

int	check_descending(int numbers[20])
{
	int	counter;
	int	numbers_amount;

	counter = 0;
	numbers_amount = get_number_amount(numbers);
	while (counter < numbers_amount - 1)
	{
		if (!(numbers[counter] > numbers[counter + 1] && numbers[counter]
				- numbers[counter + 1] <= 3))
			return (0);
		counter++;
	}
	return (1);
}

int	generate_permutations_and_check_ascending(int numbers[20])
{
	int	new_numbers[100][20] = {0};
	int	counter;
	int	counter2;
	int	numbers_amount;
	int	counter3;

	numbers_amount = get_number_amount(numbers);
	counter = 0;
	counter2 = 0;
	counter3 = 0;
	while (counter < numbers_amount)
	{
		counter2 = 0;
		counter3 = 0;
		while (counter2 < numbers_amount)
		{
			if (counter2 == counter)
			{
				counter2++;
				continue ;
			}
			new_numbers[counter][counter3] = numbers[counter2];
			counter2++;
			counter3++;
		}
		counter++;
	}
	counter2 = 0;
	while (counter2 < numbers_amount)
	{
		new_numbers[counter][counter2] = numbers[counter2];
		counter2++;
	}
	counter = 0;
	while (new_numbers[counter][0])
	{
		if (check_ascending(new_numbers[counter]) == 1)
			return (1);
		counter++;
	}
	return (0);
}

int	generate_permutations_and_check_descending(int numbers[20])
{
	int	new_numbers[100][20] = {0};
	int	counter;
	int	counter2;
	int	numbers_amount;
	int	counter3;

	numbers_amount = get_number_amount(numbers);
	counter = 0;
	counter2 = 0;
	counter3 = 0;
	while (counter < numbers_amount)
	{
		counter2 = 0;
		counter3 = 0;
		while (counter2 < numbers_amount)
		{
			if (counter2 == counter)
			{
				counter2++;
				continue ;
			}
			new_numbers[counter][counter3] = numbers[counter2];
			counter2++;
			counter3++;
		}
		counter++;
	}
	counter2 = 0;
	while (counter2 < numbers_amount)
	{
		new_numbers[counter][counter2] = numbers[counter2];
		counter2++;
	}
	counter = 0;
	while (new_numbers[counter][0])
	{
		if (check_descending(new_numbers[counter]) == 1)
			return (1);
		counter++;
	}
	return (0);
}


int	main(void)
{
	FILE	*file;
	char	*numbers[1000] = {0};
	int		counter;
	int		counter2;
	char	*line;
	size_t	line_len;
	int		numbers2[1000][20] = {0};
	char	*string;
	int		total;

	file = fopen("input", "r");
	counter = 0;
	while (counter < 1000)
	{
		getline(&line, &line_len, file);
		numbers[counter] = strdup(line);
		counter++;
	}
	counter = 0;
	while (counter < 1000)
	{
		counter2 = 0;
		string = strtok(strdup(numbers[counter]), " \n");
		while (string != NULL)
		{
			numbers2[counter][counter2] = strtol(string, NULL, 10);
			string = strtok(NULL, " \n");
			counter2++;
		}
		counter++;
	}
	counter = 0;
	total = 0;
	while (counter < 1000)
	{
		total += check_ascending(numbers2[counter]);
		total += check_descending(numbers2[counter]);
		counter++;
	}
	printf("[Part 1] %i\n", total);
	counter = 0;
	total = 0;
	while (counter < 1000)
	{
		total += generate_permutations_and_check_ascending(numbers2[counter]);
		total += generate_permutations_and_check_descending(numbers2[counter]);
		counter++;
	}
	printf("[Part 2] %i\n", total);
}
