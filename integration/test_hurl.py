#!/usr/bin/env python3
# test hurl file
#
import codecs
import sys
import subprocess
import os
import platform


def decode_string(encoded):
    if encoded.startswith(codecs.BOM_UTF8):
        return encoded.decode('utf-8-sig')
    elif encoded.startswith(codecs.BOM_UTF16):
        encoded = encoded[len(codecs.BOM_UTF16):]
        return encoded.decode('utf-16')
    else:
        return encoded.decode()


# return linux, osx or windows
def get_os():
    if platform.system() == 'Linux':
        return 'linux'
    elif platform.system() == 'Darwin':
        return 'osx'
    elif platform.system() == 'Windows':
        return 'windows'
    else:
        raise Error('Invalid Platform ' + platform.system())


def test(hurl_file):
  
    options_file = hurl_file.replace('.hurl','.options')
    options = []
    if os.path.exists(options_file):
         options = open(options_file).read().strip().split(' ')

    cmd = ['hurl', hurl_file] + options
    print(' '.join(cmd))
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
   
    # exit code 
    f = hurl_file.replace('.hurl','.exit')
    expected = int(open(f).read().strip())
    if result.returncode != expected:
        print('>>> error in return code')
        print(f'expected: {expected}  actual:{result.returncode}')
        sys.exit(1)

    # stdout
    f = hurl_file.replace('.hurl','.out')
    if os.path.exists(f):
         expected = open(f, 'rb').read()
         actual = result.stdout
         if expected != actual:
             print('>>> error in stdout')
             print(f'actual: <{actual}>\nexpected: <{expected}>')
             sys.exit(1)

    # stderr
    f = hurl_file.replace('.hurl', '.' + get_os() + '.err')
    if os.path.exists(f):
        expected = open(f).read().strip()
        actual = decode_string(result.stderr).strip()
        if expected != actual:
            print('>>> error in stderr')
            print(f'actual: <{actual}>\nexpected: <{expected}>')
            sys.exit(1)
    else:
        f = hurl_file.replace('.hurl', '.err')
        if os.path.exists(f):
            expected = open(f).read().strip()
            actual = decode_string(result.stderr).strip()
            if expected != actual:
                print('>>> error in stderr')
                print(f'actual: <{actual}>\nexpected: <{expected}>')
                sys.exit(1)

def main():
    for hurl_file in sys.argv[1:]:
        test(hurl_file)


if __name__ == '__main__':
    main()




