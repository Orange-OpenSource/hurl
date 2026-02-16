#!/usr/bin/env python3
# TTY and PTY runners for executing commands in terminal and pseudo-terminal environment.
#
import subprocess
from typing import List


class Term:
    def run(self, cmd: List[str]) -> subprocess.CompletedProcess:
        result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        return result


class PseudoTerm:
    """Pseudo-terminal wrapper for running commands with TTY simulation.

    This PTY isn't available on Windows platform.
    """

    def __init__(self, row: int = 24, col: int = 80, read_chunk_size: int = 4096):
        """Initialize Term with terminal dimensions.

        Args:
            row: Terminal height in rows (default: 24)
            col: Terminal width in columns (default: 80)
        """
        self.row = row
        self.width = col
        self.read_chunk_size = read_chunk_size

    def run(self, cmd: List[str]) -> subprocess.CompletedProcess:
        """Run command with pseudo-terminals to simulate real TTYs for both stdout and stderr.

        Args:
            cmd: Command and arguments as a list

        Returns:
            CompletedProcess with captured stdout and stderr
        """
        import fcntl
        import os
        import pty
        import select
        import struct
        import termios
        import tty

        # Create separate PTY pairs for stdout and stderr
        stdout_master, stdout_slave = pty.openpty()
        stderr_master, stderr_slave = pty.openpty()

        try:
            # Set PTYs to raw mode to prevent newline translation
            tty.setraw(stdout_slave)
            tty.setraw(stderr_slave)

            # Set terminal size (rows, cols, xpixel, ypixel)
            winsize = struct.pack("HHHH", self.row, self.width, 0, 0)
            fcntl.ioctl(stdout_slave, termios.TIOCSWINSZ, winsize)
            fcntl.ioctl(stderr_slave, termios.TIOCSWINSZ, winsize)

            # Run process with separate PTYs for stdout and stderr
            process = subprocess.Popen(
                cmd, stdout=stdout_slave, stderr=stderr_slave, close_fds=True
            )

            # Close slave fds in parent process
            os.close(stdout_slave)
            os.close(stderr_slave)

            # Read from both masters concurrently using select
            stdout_output = b""
            stderr_output = b""
            fds = {stdout_master: "stdout", stderr_master: "stderr"}

            while fds:
                # Wait for data on any of the file descriptors
                ready_fds, _, _ = select.select(list(fds.keys()), [], [])

                for fd in ready_fds:
                    try:
                        chunk = os.read(fd, self.read_chunk_size)
                        if chunk:
                            if fds[fd] == "stdout":
                                stdout_output += chunk
                            else:
                                stderr_output += chunk
                        else:
                            # EOF reached, remove this fd
                            del fds[fd]
                    except OSError:
                        # Error reading, remove this fd
                        if fd in fds:
                            del fds[fd]

            # Wait for process to finish
            return_code = process.wait()

            return subprocess.CompletedProcess(
                args=cmd,
                returncode=return_code,
                stdout=stdout_output,
                stderr=stderr_output,
            )
        finally:
            os.close(stdout_master)
            os.close(stderr_master)
