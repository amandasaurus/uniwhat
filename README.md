# uniwhat

Reads standard input, and printing out the unicode characters.

    $ echo "✨Hello! ä€" | uniwhat
    character   byte  UTF-32  encoded as     glyph    name
            0      0  002728  E2 9C A8         ✨      SPARKLES
            1      3  000048  48               H      LATIN CAPITAL LETTER H
            2      4  000065  65               e      LATIN SMALL LETTER E
            3      5  00006C  6C               l      LATIN SMALL LETTER L
            4      6  00006C  6C               l      LATIN SMALL LETTER L
            5      7  00006F  6F               o      LATIN SMALL LETTER O
            6      8  000021  21               !      EXCLAMATION MARK
            7      9  000020  20                      SPACE
            8     10  0000E4  C3 A4            ä      LATIN SMALL LETTER A WITH DIAERESIS
            9     12  0020AC  E2 82 AC         €      EURO SIGN
           10     15  00000A  0A               \n     LINE FEED (LF)


Or use standard arguments rather than stdin:

	$ uniwhat foo bar
	character   byte  UTF-32  encoded as     glyph    name
			0      0  000066  66               f      LATIN SMALL LETTER F
			1      1  00006F  6F               o      LATIN SMALL LETTER O
			2      2  00006F  6F               o      LATIN SMALL LETTER O
			3      3  000020  20                      SPACE
			4      4  000062  62               b      LATIN SMALL LETTER B
			5      5  000061  61               a      LATIN SMALL LETTER A
			6      6  000072  72               r      LATIN SMALL LETTER R


Inspired by [uniname](https://manpages.debian.org/testing/uniutils/uniname.1.en.html), which hasn't been maintained in years, so lacks recent versions of unicode.

## Copyright

© 2020. GNU Affero GPL v3 or later. See [LICENCE.md](LICENCE.md).
