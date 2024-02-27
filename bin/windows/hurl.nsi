; includes
!include "MUI2.nsh"
!include "FileFunc.nsh"

; define icons
!define MUI_ICON "..\..\bin\windows\logo.ico"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "..\..\bin\windows\logo.bmp"
!define MUI_HEADERIMAGE_RIGHT

; define version
!define /file VERSION "version.txt"

; define windows uninstall panel registry path
!define UNINSTALLPANELKEY "SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Hurl"

; The name of the installer
Name "hurl ${VERSION}"

; The file to write

OutFile "hurl-${VERSION}-x86_64-pc-windows-msvc-installer.exe"

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
    ; Write installation dir to user Path variable
    EnVar::SetHKCU
    EnVar::Check "NULL" "NULL"
    EnVar::DeleteValue "Path" "$INSTDIR"
    EnVar::AddValue "Path" "$INSTDIR"
    SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
    ; Write the uninstall
    WriteUninstaller "$INSTDIR\uninstall.exe"
    ; Write windows uninstall panel information
    ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
    IntFmt $0 "0x%08X" $0
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "DisplayName" "Hurl"
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "DisplayVersion" "${VERSION}"
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "DisplayIcon" "$INSTDIR\hurl.exe"
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "UninstallString" "$\"$INSTDIR\uninstall.exe$\""
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "URLUpdateInfo" "https://github.com/Orange-OpenSource/hurl/releases"
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "URLInfoAbout" "https://github.com/Orange-OpenSource/hurl"
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "HelpLink" "https://hurl.dev"
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "RegCompany" "Orange-OpenSource"
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "Readme" "$INSTDIR\README.md"
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "NoModify" 1
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "NoRepair" 1
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "InstallLocation" "$INSTDIR"
    WriteRegStr HKLM "${UNINSTALLPANELKEY}" "Publisher" "Orange-OpenSource"
    WriteRegDWORD HKLM "${UNINSTALLPANELKEY}" "EstimatedSize" "$0"
  SectionEnd
  Section "hurlfmt.exe"
    SectionIn RO
    SetOutPath $INSTDIR
    File "hurlfmt.exe"
  SectionEnd
SectionGroupEnd

SectionGroup "dlls"
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
  Section "zlib1.dll"
    SectionIn RO
    SetOutPath $INSTDIR
    File "zlib1.dll"
  SectionEnd
  Section "libcurl.dll"
    SectionIn RO
    SetOutPath $INSTDIR
    File "libcurl.dll"
  SectionEnd
  Section "nghttp2.dll"
    SectionIn RO
    SetOutPath $INSTDIR
    File "nghttp2.dll"
  SectionEnd
SectionGroupEnd

SectionGroup "doc"
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
  Section "completions\_hurl.ps1"
    SectionIn 3
    SetOutPath $INSTDIR\completions
    File "..\..\completions\_hurl.ps1"
  SectionEnd
  Section "completions\_hurlfmt.ps1"
    SectionIn 3
    SetOutPath $INSTDIR\completions
    File "..\..\completions\_hurlfmt.ps1"
  SectionEnd
SectionGroupEnd

; Uninstaller
Section "Uninstall"
  ; Remove installed files
  Delete $INSTDIR\*
  RMDir "$INSTDIR"
  ; Remove entry from windows uninstaller panel
  DeleteRegKey HKLM "${UNINSTALLPANELKEY}"
  ; Remove install dir from user Path variable
  EnVar::SetHKCU
  EnVar::Check "NULL" "NULL"	
  EnVar::DeleteValue "Path" "$INSTDIR"
SectionEnd
