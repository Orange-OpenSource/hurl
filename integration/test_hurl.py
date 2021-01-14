#!/usr/bin/env python3
# test hurl file
#
import sys
import subprocess
import os

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
        print('expected: {expected}  actual:{result.returncode}')
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
    f = hurl_file.replace('.hurl','.err')
    if os.path.exists(f):
         expected = open(f).read().strip()
         actual = result.stderr.decode("utf-8").strip() 
         if expected != actual:
             print('>>> error in stderr')
             print(f'actual: <{actual}>\nexpected: <{expected}>')
             sys.exit(1)


def main():
    for hurl_file in sys.argv[1:]:
        test(hurl_file)


if __name__ == '__main__':
    main()




