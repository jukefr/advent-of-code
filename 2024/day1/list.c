/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   list.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: kjullien <kjullien@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/01 21:58:15 by kjullien          #+#    #+#             */
/*   Updated: 2024/12/01 22:04:31 by kjullien         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "main.h"

size_t	list_length(t_list **list)
{
	t_list	*current;
	size_t	counter;

	if (*list == NULL)
		return (0);
	current = *list;
	counter = 0;
	while (current)
	{
		counter++;
		current = current->next;
	}
	return (counter);
}

void	list_add_back(t_list *to_add, t_list **list)
{
	t_list	*current;

	if (*list == NULL)
	{
		*list = to_add;
		return ;
	}
	current = *list;
	while (current->next)
		current = current->next;
	current->next = to_add;
}
