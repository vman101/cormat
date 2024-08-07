#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "libft/ft_printf.h"
#include "libft/libft.h"

#define ERROR(string)	do {	\
							perror(string); \
							ft_printf( \
							"ERROR: "string": file: %s line: %d\n",\
							__FILE__, __LINE__); \
							exit (1); \
						} while (0)

#define UNIMPLEMENTED(string)	do {	\
									ft_printf( \
									"UNIMPLEMENTED: "string": file: %s line: %d\n",\
									__FILE__, __LINE__); \
								} while (0)

void	print_usage(void)
{
	printf("Usage:\tcormat [options ...] <files ...>\n");
	exit (1);
}

void	format_run_and_replace(char **argv)
{
	(void)argv;
	UNIMPLEMENTED("running and replace");
}

void	format_run_check_only(char **argv)
{
	(void)argv;
	UNIMPLEMENTED("running check only");
}

int main(int argc, char **argv)
{
	FILE	**input_file;

	if (argc < 2)
		print_usage();
	else
	{
		if (*argv[1] == '-')
		{
			if (ft_strncmp(argv[1], "-c", 3) == 0)
				format_run_check_only(&argv[2]);
			else if (ft_strncmp(argv[1], "-r", 3) == 0)
				format_run_and_replace(&argv[2]);
		}
		else
		{
			uint32_t	i = 1;
			while (argv[i])
				i++;
			input_file = ft_calloc(i + 1, sizeof(input_file));
			if (*input_file)
				ERROR("malloc");
			i = 0;
			while (argv[i])
			{
				input_file[i] = fopen(argv[i], "r");
				if (input_file[i] == NULL)
					ERROR("fopen");
				i++;
			}
		}
	}
	uint32_t	i = 1;
	char		*file_content = NULL;
	uint32_t	file_size = 0;

	fseek(input_file[i], 0L, SEEK_END);
	file_size = ftell(input_file[i]);
	rewind(input_file[i]);
	if (file_size == 0)
		ft_printf("%s: file is empty!\n", argv[i]);
	file_content = ft_calloc(file_size + 1, sizeof(*file_content));
	if (!file_content)
		ERROR("malloc");
	fread(file_content, file_size, 1, input_file[i]);
	ft_printf("%s", file_content);
	i = 1;
	while (i < argc)
	{
		fclose(input_file[i]);
		i++;
	}
	ft_free(&file_content);
	ft_free(&input_file);
}
