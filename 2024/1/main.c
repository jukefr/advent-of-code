/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: kjullien <kjullien@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/01 21:03:01 by kjullien          #+#    #+#             */
/*   Updated: 2024/12/01 23:47:05 by kjullien         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "main.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void	process(t_list *list, int *left, int *right)
{
	size_t	counter;
	int		*extracted;
	t_list	*current;

	current = list;
	counter = 0;
	while (counter < list_length(&list))
	{
		extracted = get_numbers_from_line(current->content);
		left[counter] = extracted[0];
		right[counter] = extracted[1];
		free(extracted);
		current = current->next;
		counter++;
	}
	qsort(left, list_length(&list), sizeof(int), compare);
	qsort(right, list_length(&list), sizeof(int), compare);
}

void	part1(t_list *list, int *left, int *right)
{
	size_t		counter;
	long long	total;

	counter = 0;
	total = 0;
	while (counter < list_length(&list))
	{
		if (left[counter] > right[counter])
			total += left[counter] - right[counter];
		else
			total += right[counter] - left[counter];
		counter++;
	}
	printf("[Part 1] %lld\n", total);
}

void	part2(t_list *list, int *left, int *right)
{
	size_t		counter;
	size_t		counter2;
	long long	total;

	counter = 0;
	total = 0;
	while (counter < list_length(&list))
	{
		counter2 = 0;
		while (counter2 < list_length(&list))
		{
			if (left[counter] == right[counter2])
				total += left[counter];
			counter2++;
		}
		counter++;
	}
	printf("[Part 2] %lld\n", total);
}

int	main(void)
{
	t_list	*list;
	int		*left;
	int		*right;

	list = NULL;
	read_file_to_list("input", &list);
	left = malloc(sizeof(int) * list_length(&list));
	right = malloc(sizeof(int) * list_length(&list));
	process(list, left, right);
	part1(list, left, right);
	part2(list, left, right);
	return (0);
}
