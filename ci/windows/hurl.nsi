﻿; includes
!include "MUI2.nsh"

; define icons
!define MUI_ICON "..\..\ci\windows\logo.ico"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "..\..\ci\windows\logo.png"
!define MUI_HEADERIMAGE_RIGHT

; define version
!define /file VERSION "version.txt"

; The name of the installer
Name "hurl ${VERSION}"

; The file to write

OutFile "hurl-${VERSION}-win64-installer.exe"

; Request application privileges for Windows Vista and higher
RequestExecutionLevel admin

; Build Unicode installer
Unicode False

; The default installation directory
InstallDir $PROGRAMFILES64\hurl

; Start pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE ..\..\LICENSE

Page components
Page directory
Page instfiles

; Finish page
!define MUI_FINISHPAGE_LINK 'Click here to visit us at https://hurl.dev/'
  !define MUI_FINISHPAGE_LINK_LOCATION https://hurl.dev/
!define MUI_FINISHPAGE_TITLE_3LINES
  !define MUI_FINISHPAGE_TITLE "Congratulation, hurl ${VERSION} is ready to use on your favorite windows terminal (cmd and powershell)"
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_LANGUAGE English

UninstPage uninstConfirm
UninstPage instfiles

; The stuff to install
SectionGroup "executables"
  Section "hurl.exe"
    SectionIn RO
    SetOutPath $INSTDIR
    File "hurl.exe"
    ; Write installation path
    EnVar::SetHKCU
	EnVar::Check "NULL" "NULL"
	EnVar::DeleteValue "Path" ";$INSTDIR"
	EnVar::DeleteValue "Path" "$INSTDIR;"
	EnVar::AddValue "Path" ";$INSTDIR"
	; ReadRegStr $0  HKCU "Environment" "Path"
    ; WriteRegStr HKCU "Environment" "path" "$0;$INSTDIR"
	SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
    ; Write the uninstall
    WriteUninstaller "$INSTDIR\uninstall.exe"
  SectionEnd
  Section "hurlfmt.exe"
    SectionIn RO
    SetOutPath $INSTDIR
    File "hurlfmt.exe"
  SectionEnd  
SectionGroupEnd

SectionGroup "dlls"
  Section "charset-1.dll"
    SectionIn RO
    SetOutPath $INSTDIR
    File "charset-1.dll"
  SectionEnd
  Section "iconv-2.dll"
    SectionIn RO
    SetOutPath $INSTDIR
    File "iconv-2.dll"
  SectionEnd
  Section "libxml2.dll"
    SectionIn RO
    SetOutPath $INSTDIR
    File "libxml2.dll"
  SectionEnd
  Section "lzma.dll"
    SectionIn RO
    SetOutPath $INSTDIR
    File "lzma.dll"
  SectionEnd
  Section "zlib1.dll"
    SectionIn RO
    SetOutPath $INSTDIR
    File "zlib1.dll"
  SectionEnd 
SectionGroupEnd

SectionGroup "txt"
  Section "version.txt"
    SectionIn 3
    SetOutPath $INSTDIR
    File "version.txt"
  SectionEnd
  Section "README.md"
    SectionIn 3
    SetOutPath $INSTDIR
    File "..\..\README.md"
  SectionEnd
  Section "CHANGELOG.md"
    SectionIn 3
    SetOutPath $INSTDIR
    File "..\..\CHANGELOG.md"
  SectionEnd    
  Section "LICENSE"
    SectionIn 3
    SetOutPath $INSTDIR
    File "..\..\LICENSE"
  SectionEnd  
SectionGroupEnd

; Uninstaller
Section "Uninstall" 
  ; Remove files and uninstaller
  Delete $INSTDIR\*
  ; Remove directories
  RMDir "$INSTDIR"
SectionEnd