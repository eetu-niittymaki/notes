# notes

![image](https://i.postimg.cc/zXybQCZh/img.png)

## CLI tool for saving, viewing, editing, importing and exporting notes

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
### history - Attach a tag to a note, delete tag, list all tags
### register - Create user account for service
### login - Login to service, save credentials to OS specific credential manager
### logout - Delete saved login credentials from credential manager
### version - Prints the programs version number
