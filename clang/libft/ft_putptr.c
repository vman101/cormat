/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_putptr.c                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vvobis <marvin@42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/04/18 17:53:53 by vvobis            #+#    #+#             */
/*   Updated: 2024/04/18 20:44:46 by vvobis           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "ft_printf.h"

void	ft_putptr(void *ptr, int *count)
{
	void	**to_print;

	if (!ptr)
	{
		*count += ft_printf("(nil)");
		return ;
	}
	to_print = &ptr;
	*count += ft_printf("0x");
	ft_puthex_lower((long)*to_print, count);
}
