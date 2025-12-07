# Function list

## Register shell script
### detail
- register selected shell script
- user can specify shell script name (default: file name without extension as shell script name)
- all shell scripts are grouped
  - create new group if needed
  - select group to register shell script
### command
- `abs register <script-path> [--group <group-path>] [--name <script-name>]`

## Edit registerd information
### detail
- edit group infomation
  - create new group
  - delete group
  - rename group
  - move group
- move shell script to another group
- rename registered shell script
### command
- `abs group create <group-name>`
- `abs group delete <group-name>`
- `abs group rename <old-group-name> <new-group-name>`
- `abs group move <script-name> <target-group-name>`
- `abs script rename <old-script-name> <new-script-name>`

## Call registered shell script
### detail
- call selected shell script
- use this command without arguments
  - show list of registered shell scripts
  - user can select shell script to call
- use this command with group name argument
  - show list of registered shell scripts in the group
  - user can select shell script to call
- use this command with group name and shell script name arguments
  - call specified shell script directly
- autocomplete support for group name and shell script name
- if shell script has arguments, user can use arguments
### command
- `abs call [--group <group-name>] [--script <script-name>] [-- <script-args>]`

## Print list of registered shell scripts
### detail
- print list of registered shell scripts
### command
- `abs list`

## Print registered shell scripts
### detail
- print shell script content
- use this command without arguments
  - show list of registered shell scripts
  - user can select shell script to print
- use this command with group name argument
  - show list of registered shell scripts in the group
  - user can select shell script to print
- use this command with group name and shell script name arguments
  - print specified shell script content directly
- autocomplete support for group name and shell script name
### command
- `abs print [--group <group-name>] [--script <script-name>]`

## Edit registered shell script
### detail
- open in configured editor to edit selected shell script
  - user can configure editor command in config file
- use this command without arguments
  - show list of registered shell scripts
  - user can select shell script to edit
- use this command with group name argument
  - show list of registered shell scripts in the group
  - user can select shell script to edit
- use this command with group name and shell script name arguments
  - open specified shell script in editor directly
- autocomplete support for group name and shell script name
### command
- `abs edit [--group <group-name>] [--script <script-name>]`

## Record command to shell script
### detail
- enter command to record mode
- in record mode, all command line inputs are recorded to tmp file
- exit record mode by special command and save recorded commands to selected group and shell script name
### command
- `abs record [--group <group-name>] [--script <script-name>]`