# Function list
- [Function list](#function-list)
  - [Execute Shell script](#execute-shell-script)
    - [execute registered shell script](#execute-registered-shell-script)
      - [detail](#detail)
      - [command](#command)
  - [Add Shell script](#add-shell-script)
    - [Register shell script](#register-shell-script)
      - [detail](#detail-1)
      - [command](#command-1)
    - [Create new shell script](#create-new-shell-script)
      - [detail](#detail-2)
      - [command](#command-2)
    - [Record command to shell script](#record-command-to-shell-script)
      - [detail](#detail-3)
      - [command](#command-3)
  - [Edit shell and group](#edit-shell-and-group)
    - [Rename group and shell](#rename-group-and-shell)
      - [detail](#detail-4)
      - [command](#command-4)
    - [Delete group and shell](#delete-group-and-shell)
      - [detail](#detail-5)
      - [command](#command-5)
    - [Edit registerd shell script by text editor](#edit-registerd-shell-script-by-text-editor)
      - [detail](#detail-6)
      - [command](#command-6)
  - [Print informations](#print-informations)
    - [Print registerd shell script](#print-registerd-shell-script)
      - [detail](#detail-7)
      - [command](#command-7)
    - [Print list of registered shell scripts](#print-list-of-registered-shell-scripts)
      - [detail](#detail-8)
      - [command](#command-8)
  - [Configuration](#configuration)
    - [View configuration](#view-configuration)
      - [detail](#detail-9)
      - [command](#command-9)
    - [Set configuration](#set-configuration)
      - [detail](#detail-10)
      - [command](#command-10)
    - [Delete configuration](#delete-configuration)
      - [detail](#detail-11)
      - [command](#command-11)

## Execute Shell script
### execute registered shell script
#### detail
- execute selected registered shell script
- user can specify following arguments:
  - register-path
    - optional
    - example: group1/script1
    - autocomplete support for group name and shell script name
    - if not specified, open following interactive prompts:
      - select group from interactive shell script selection menu
#### command
- `abs call <register-path> [<script-args>]`

## Add Shell script
### Register shell script
#### detail
- register existing shell script file
  - user can specify following arguments:
    - script-path
      - mandatory
      - path to shell script file to register
      - autocomplete support for file path
    - register-path
      - optional
      - path to register shell script
      - example: group1/script1
      - if not specified, open following interactive prompts:
        - select group from interactive group selection menu
        - input script name in interactive name input prompt
          - default value is script file name without extension
      - create new group if needed
      - autocomplete support for group name

#### command
- `abs register <script-path> [<register-path>]`

### Create new shell script
#### detail
- create new shell script file and register it
  - user can specify following arguments:
    - register-path
      - optional
      - path to register shell script
      - example: group1/script1
      - if not specified, open following interactive prompts:
        - select group from interactive group selection menu
        - input script name in interactive name input prompt
          - default value is "new_script_<timestamp>"
      - create new group if needed
      - autocomplete support for group name
    - --open
      - optional
      - open created shell script in configured editor
        - user can configure editor command in config file
        - if not configured, use system default text editor
#### command
- `abs create [<register-path>] [--open]`

### Record command to shell script
#### detail
- enter command to record mode
  - --disable-replace-absolute-path
    - optional
    - disable replacing absolute path to relative path in recorded commands
- in record mode, all command line inputs are recorded to tmp file
- exit record mode by special command and save recorded commands to selected group and shell script name
    - register-path
      - optional
      - path to register shell script
      - example: group1/script1
      - if not specified, open following interactive prompts:
        - select group from interactive group selection menu
        - input script name in interactive name input prompt
          - default value is "new_script_<timestamp>"
      - create new group if needed
      - autocomplete support for group name
#### command
- `abs record [--disable-replace-absolute-path]`
    - start record mode
    - in record mode, use command `:wq <register-path>` to save and exit record mode


## Edit shell and group
### Rename group and shell
#### detail
- delete selected group and shell script
- user can specify following arguments:
  - before-register-path
    - mandatory
    - example: group1/script1
    - autocomplete support for group name and shell script name
  - after-register-path
    - mandatory
    - example: group2/script2
    - autocomplete support for group name and shell script name
    - create new group if needed
#### command
- `abs move <before-register-path> <after-register-path>`

### Delete group and shell
#### detail
- delete selected group and shell script
- user can specify following arguments:
- register-path
    - mandatory
    - example: group1/script1
    - autocomplete support for group name and shell script name
#### command
- `abs delete <register-path>`

### Edit registerd shell script by text editor
#### detail
- open in configured editor to edit selected shell script
- user can specify following arguments:
  - register-path
    - optional
    - example: group1/script1
    - autocomplete support for group name and shell script name
    - if not specified, open following interactive prompts:
      - select group from interactive shell script selection menu
- user can configure editor command in config file
- if not configured, use system default text editor
#### command
- `abs edit [<register-path>]`


## Print informations
### Print registerd shell script
#### detail
- print shell script content
- user can specify following arguments:
  - register-path
    - optional
    - example: group1/script1
    - autocomplete support for group name and shell script name
    - if not specified, open following interactive prompts:
      - select group from interactive shell script selection menu
#### command
- `abs print [<register-path>]`

### Print list of registered shell scripts
#### detail
- print list of registered shell scripts
#### command
- `abs list`

## Configuration
### View configuration
#### detail
- print current configuration
#### command
- `abs config view`

### Set configuration
#### detail
- set configuration value
- user can specify following arguments:
- key
    - mandatory
    - configuration key
    - available keys:
      - editor_command
        - command to open shell script in text editor
- value
    - mandatory
    - configuration value
#### command
- `abs config set <key> <value>`

### Delete configuration
#### detail
- delete configuration value
- user can specify following arguments:
- key
    - mandatory
    - configuration key
    - available keys:
      - editor_command
        - command to open shell script in text editor
#### command
- `abs config delete <key>`