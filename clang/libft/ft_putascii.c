/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_putascii.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vvobis <marvin@42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/04/18 17:54:53 by vvobis            #+#    #+#             */
/*   Updated: 2024/04/18 17:55:04 by vvobis           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "ft_printf.h"

void	ft_putchar(int c, int *count)
{
	write(1, &c, 1);
	*count += 1;
}

void	ft_putnbr(long n, int *count)
{
	if (n < 0)
		ft_putchar(0x2d, count);
	if (n <= -10)
		ft_putnbr(n / -10, count);
	if (n >= 10)
		ft_putnbr(n / 10, count);
	if (n >= 0)
		ft_putchar(n % 10 + 0x30, count);
	if (n < 0)
		ft_putchar(-(n % -10) + 0x30, count);
}

void	ft_putstr(const char *str, int *count)
{
	if (!str)
	{
		*count += ft_printf("(null)");
		return ;
	}
	while (*str)
		ft_putchar(*str++, count);
}
