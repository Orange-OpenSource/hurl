from typing import Callable


class Parser:
    """Base class for text parser.

    This class provides method for a text parser. This can be used as an instance
    or be extended for a custom parser.

    Attributes:
          buffer: the text buffer to parse.
          offset: offset of the current parsing position within the buffer.
    """

    buffer: str
    offset: int

    def __init__(self, buffer: str) -> None:
        self.buffer = buffer
        self.offset = 0

    def read(self, count: int = 1) -> str:
        """Return count characters from the buffer."""
        if self.left() < count:
            return ""
        ret = self.buffer[self.offset : self.offset + count]
        self.offset += count
        return ret

    def read_while(self, f: Callable[[str], bool]) -> str:
        """Return characters from the buffer while the current character meet a criteria."""
        offset = self.offset
        while self.left() > 0:
            c = self.peek()
            if f(c):
                _ = self.read()
            else:
                break
        return self.buffer[offset : self.offset]

    def read_while_prev(self, f: Callable[[str, str], bool]) -> str:
        """Return characters from the buffer while the current and previous character meet a criteria."""
        offset = self.offset
        while self.left() > 0:
            c = self.peek()
            prev = self.buffer[self.offset - 1]
            if f(c, prev):
                _ = self.read()
            else:
                break
        return self.buffer[offset : self.offset]

    def peek(self, count: int = 1) -> str:
        """Return count characters from the buffer, without modifying the parsing offset."""
        if self.left() < count:
            return ""
        return self.buffer[self.offset : self.offset + count]

    def peek_while(self, f) -> str:
        """Return characters from the buffer while the current character meet a criteria, without modifying the
        parsing offset."""
        offset = self.offset
        while offset < len(self.buffer):
            c = self.buffer[offset]
            if f(c):
                offset += 1
            else:
                break
        return self.buffer[self.offset : offset]

    def left(self) -> int:
        """Return the number of characters left to read."""
        return len(self.buffer) - self.offset
