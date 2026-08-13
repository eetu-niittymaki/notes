# notes

![image](https://i.postimg.cc/Y9gXKThL/img.png)

## CLI tool for locally saving, viewing, editing and exporting notes

# Installation
## Download and run install.ps1 in powershell 
### On Windows 11 if you have execution policy blocked run: 
### powershell -ExecutionPolicy Bypass -File "\Path\To\install.ps1"
## Or download and extract notes.zip manually, add location of notes.exe to PATH

# Usage
### notes COMMAND [OPTIONS]

## Commands
### all - Get all notes from database
### get - Get specific note by ID or title 
### new - Add new note to database
### edit - Change content or title of saved note
### search - Search for note that includes title or text content 
### export - Export all notes to file (txt, md, html, png or pdf)
### import - Import a files (txt, md, html) contents as plain text to database
### tag - Attach a tag to a note, delete tag, list all tags
### version - Prints the programs version number
