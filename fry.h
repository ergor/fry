
#ifndef FRY_H
#define FRY_H

#include <stdbool.h>

#define INVERT      "\033[7m"
#define RST_INVERT  "\033[27m"

#define EE      ' '     /* empty square */

#define WP      'P'     /* white pawn   */
#define WK      'K'     /* white king   */
#define WQ      'Q'     /* white queen  */
#define WR      'R'     /* white rook   */
#define WN      'N'     /* white knight */
#define WB      'B'     /* white bishop */

#define BP      'p'     /* black pawn   */
#define BK      'k'     /* black king   */
#define BQ      'q'     /* black queen  */
#define BR      'r'     /* black rook   */
#define BN      'n'     /* black knight */
#define BB      'b'     /* black bishop */

struct vect {
    int x;
    int y;
};

struct piece {
    int val;                /* centipawn value of piece */
    char sym;               /* ASCII representation of the piece */
    bool iter;              /* whether the vector should be iterated */
    int mvt_len;            /* movement vecetor length */
    int att_len;            /* attack vector length */
    struct vect * mvt;      /* movement vector */
    struct vect * att;      /* attack vector (NULL if same as mvmt) */
};

extern struct piece pieces[128];
void init_pieces();

#endif
