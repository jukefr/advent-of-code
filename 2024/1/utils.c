/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   utils.c                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: kjullien <kjullien@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/01 21:59:10 by kjullien          #+#    #+#             */
/*   Updated: 2024/12/01 23:22:07 by kjullien         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "main.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void	read_file_to_list(char *filename, t_list **list)
{
	FILE	*file;
	char	*line;
	size_t	line_length;
	t_list	*new;
	ssize_t	getline_value;

	file = fopen(filename, "r");
	if (file == NULL)
		exit(EXIT_FAILURE);
	while (1)
	{
		line = NULL;
		getline_value = getline(&line, &line_length, file);
		if (getline_value == -1)
			break ;
		new = malloc(sizeof(t_list *));
		new->content = line;
		list_add_back(new, list);
	}
}

int	*get_numbers_from_line(char *line)
{
	char	*left;
	char	*right;
	int		*result;

	result = malloc(sizeof(int) * 2);
	left = strtok(line, " \n");
	result[0] = strtol(left, NULL, 10);
	right = strtok(NULL, " \n");
	result[1] = strtol(right, NULL, 10);
	return (result);
}

int	compare(const void *a, const void *b)
{
	int	ia;
	int	ib;

	ia = *(const int *)a;
	ib = *(const int *)b;
	return (ia - ib);
}
