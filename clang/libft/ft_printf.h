/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_printf.h                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vvobis <marvin@42.fr>                      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/04/18 12:50:35 by vvobis            #+#    #+#             */
/*   Updated: 2024/04/18 20:47:09 by vvobis           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef FT_PRINTF_H

# define FT_PRINTF_H

# include <stdarg.h>
# include <stdlib.h>
# include <unistd.h>

int		ft_printf(const char *format, ...);
void	ft_puthex_lower(unsigned long nbr, int *count);
void	ft_puthex_upper(unsigned long nbr, int *count);
void	ft_putchar(int c, int *count);
void	ft_putnbr(long n, int *count);
void	ft_putstr(const char *str, int *count);
void	ft_putptr(void *ptr, int *count);

#endif
