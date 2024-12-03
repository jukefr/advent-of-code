/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.h                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: kjullien <kjullien@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/12/01 21:20:26 by kjullien          #+#    #+#             */
/*   Updated: 2024/12/01 23:18:35 by kjullien         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef MAIN_H
# define MAIN_H

# include <stddef.h>

typedef struct s_list
{
	char			*content;
	struct s_list	*next;
}					t_list;

void				list_add_back(t_list *to_add, t_list **list);
size_t				list_length(t_list **list);
void				read_file_to_list(char *filename, t_list **list);
int					*get_numbers_from_line(char *line);
int					compare(const void *a, const void *b);

#endif
