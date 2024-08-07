CC := cc

CFLAGS := -Wall -Wextra -g3 -fsanitize=leak,address,undefined

SRC := formatter.c

OBJDIR := obj

OBJ := $(SRC:%.c=$(OBJDIR)/%.o)

LIBS := libft/libft.a

NAME := cormat

all: $(NAME)

$(NAME): $(OBJ) $(LIBS)
	$(CC) $(CFLAGS) -o $@ $^

$(OBJ): $(SRC) $(OBJDIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(LIBS):
	make re -C libft

$(OBJDIR):
	mkdir -p $(OBJDIR)

clean:
	rm -rf $(OBJDIR)

fclean: clean
	rm -f $(NAME)

re: fclean all
