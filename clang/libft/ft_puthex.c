/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_puthex.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vvobis <marvin@42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/04/18 17:52:38 by vvobis            #+#    #+#             */
/*   Updated: 2024/04/18 20:46:13 by vvobis           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "ft_printf.h"

void	ft_puthex_upper(unsigned long nbr, int *count)
{
	char	*base_str;

	base_str = "0123456789ABCDEF";
	if (nbr >= 16)
		ft_puthex_upper(nbr / 16, count);
	ft_putchar(base_str[(nbr % 16)], count);
}

void	ft_puthex_lower(unsigned long nbr, int *count)
{
	char	*base_str;

	base_str = "0123456789abcdef";
	if (nbr >= 16)
		ft_puthex_lower(nbr / 16, count);
	ft_putchar(base_str[(nbr % 16)], count);
}
